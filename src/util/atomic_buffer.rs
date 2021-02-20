use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct AtomicBuffer<T: Sized, const SIZE: usize> {
    data: UnsafeCell<[T; SIZE]>,
    write_cursor: AtomicUsize,
    read_cursor: AtomicUsize,
}

unsafe impl<T, const SIZE: usize> Sync for AtomicBuffer<T, SIZE> {}
unsafe impl<T, const SIZE: usize> Send for AtomicBuffer<T, SIZE> {}

impl<T, const SIZE: usize> AtomicBuffer<T, SIZE> {
    pub const fn new(initial: [T; SIZE]) -> Self {
        AtomicBuffer {
            data: UnsafeCell::new(initial),
            write_cursor: AtomicUsize::new(0),
            read_cursor: AtomicUsize::new(0),
        }
    }

    const fn advance(&self, c: usize) -> usize {
        (c + 1) % SIZE
    }

    pub fn write(&self, value: T) {
        let read = self.read_cursor.load(Ordering::Relaxed);
        let write = self.write_cursor.load(Ordering::Relaxed);
        if self.advance(write) != read {
            self.write_cursor
                .store(self.advance(write), Ordering::Relaxed);
            let p = unsafe { &mut *self.data.get() };
            p[write] = value;
        }
    }

    pub fn read(&self) -> Option<T>
    where
        T: Copy,
    {
        let write = self.write_cursor.load(Ordering::Relaxed);
        let read = self.read_cursor.load(Ordering::Relaxed);

        if read == write {
            return None;
        }

        self.read_cursor
                .store(self.advance(read), Ordering::Relaxed);

        let p = unsafe { &*self.data.get() };
        Some(p[read])
    }
}
