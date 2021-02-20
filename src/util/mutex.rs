use crate::prelude::yield_now;
use core::cell::UnsafeCell;
use core::mem::transmute;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering::SeqCst;

pub enum StaticMutex<T: 'static + Default> {
    Initialized(Mutex<T>),
    Unitialized,
}

impl<T: Default> StaticMutex<T> {
    pub unsafe fn init(&self) {
        #[allow(mutable_transmutes)]
        let i_live_for_danger = transmute::<&StaticMutex<T>, &mut StaticMutex<T>>(self);
        *i_live_for_danger = StaticMutex::Initialized(Mutex::new(T::default()));
    }

    pub unsafe fn bypass_lock(&self) -> &'_ mut T {
        match self {
            StaticMutex::Initialized(m) => m,
            StaticMutex::Unitialized => panic!("Attempted to use an unitialized static!"),
        }
        .bypass_lock()
    }

    pub async fn lock(&self) -> MutexGuard<'_, T> {
        match self {
            StaticMutex::Initialized(m) => m,
            StaticMutex::Unitialized => panic!("Attempted to use an unitialized static!"),
        }
        .lock()
        .await
    }

    pub fn spin_lock(&self) -> MutexGuard<'_, T> {
        match self {
            StaticMutex::Initialized(m) => m,
            StaticMutex::Unitialized => panic!("Attempted to use an unitialized static!"),
        }
        .spin_lock()
    }
}

#[derive(Debug, Default)]
pub struct Mutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T> {
    __ptr: &'a Mutex<T>,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(t),
        }
    }

    pub unsafe fn bypass_lock(&self) -> &'_ mut T {
        &mut *self.data.get()
    }

    fn try_lock(&self) -> bool {
        !self.locked.swap(true, SeqCst)
    }

    fn unlock(&self) {
        self.locked.store(false, SeqCst);
    }

    pub async fn lock(&self) -> MutexGuard<'_, T> {
        loop {
            if self.try_lock() {
                return MutexGuard { __ptr: self };
            } else {
                yield_now().await;
            }
        }
    }

    pub fn spin_lock(&self) -> MutexGuard<'_, T> {
        loop {
            if self.try_lock() {
                return MutexGuard { __ptr: self };
            } else {
                core::hint::spin_loop();
            }
        }
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.__ptr.data.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.__ptr.data.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.__ptr.unlock();
    }
}
