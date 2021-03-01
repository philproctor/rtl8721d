#![allow(dead_code)]

use crate::Result;
use rtl8720_sys as sys;

pub struct RTLCritical {
    phantom: core::marker::PhantomData<()>,
}

impl RTLCritical {
    pub fn enter() -> Self {
        //TODO: Need agnostic impl
        unsafe { sys::c::vPortEnterCritical() };
        Self {
            phantom: core::marker::PhantomData::<()>,
        }
    }

    pub fn drop(self) {}
}

impl Drop for RTLCritical {
    fn drop(&mut self) {
        //TODO: Need agnostic impl
        unsafe { sys::c::vPortExitCritical() };
    }
}

pub struct RTLTXIncrement<const IDX: i32> {
    phantom: core::marker::PhantomData<()>,
}

impl<const IDX: i32> RTLTXIncrement<IDX> {
    pub fn enter() -> Self {
        let cs = RTLCritical::enter();
        unsafe { sys::c::rltk_wlan_tx_inc(IDX) };
        cs.drop();
        Self {
            phantom: core::marker::PhantomData::<()>,
        }
    }

    pub fn drop(self) {}
}

impl<const IDX: i32> Drop for RTLTXIncrement<IDX> {
    fn drop(&mut self) {
        let cs = RTLCritical::enter();
        unsafe { sys::c::rltk_wlan_tx_dec(IDX) };
        cs.drop();
    }
}

pub struct RTLWifiMutex {
    phantom: core::marker::PhantomData<()>,
}

impl RTLWifiMutex {
    pub fn enter() -> Result<Self> {
        unsafe { sys::c::device_mutex_lock(sys::c::_RT_DEV_LOCK_E_RT_DEV_LOCK_WLAN) };
        Ok(Self {
            phantom: core::marker::PhantomData::<()>,
        })
    }

    pub fn drop(self) {}
}

impl Drop for RTLWifiMutex {
    fn drop(&mut self) {
        unsafe { sys::c::device_mutex_unlock(sys::c::_RT_DEV_LOCK_E_RT_DEV_LOCK_WLAN) };
    }
}
