#![allow(unused_macros)]

use crate::*;

macro_rules! memcpy {
    ($src:expr, $dst:expr, $size:expr) => {
        ($src as *mut u8).copy_to($dst.cast(), $size as usize);
    };
}

macro_rules! cpy_vec {
    ($src:expr, $len:expr) => {{
        if ($src).is_null() {
            None
        } else {
            let slice = core::ptr::slice_from_raw_parts(($src).cast::<u8>(), $len as usize);
            Some(Vec::from((&*slice).clone()))
        }
    }};
}

macro_rules! alloc_skb {
    ($buf:ident) => {{
        let sz = $buf.len() as u32;
        let mut skb = sys::c::rltk_wlan_alloc_skb(sz);
        if skb.is_null() {
            None
        } else {
            memcpy!((*skb).tail, ($buf as *mut [u8]), sz);
            (*skb).len += sz;
            (*skb).tail = (*skb).tail.add(sz as usize);
            Some(skb)
        }
    }};
}

macro_rules! wifi_set_mib {
    () => {
        sys::c::wext_set_adaptivity(sys::c::RTW_ADAPTIVITY_DISABLE);
        sys::c::wext_set_trp_tis(sys::c::DISABLE as u8);
    };
}

pub trait WifiExtensions<const IDX: i32, const MODE: u32> {
    fn mode_init() -> Result<()>;

    fn init() -> Result<()> {
        let lock = RTLWifiMutex::enter()?;
        unsafe {
            sys::c::init_event_callback_list();
            wifi_set_mib!();
            sys::c::rltk_wlan_init(IDX, 0);
            sys::c::rltk_wlan_start(IDX);
        }
        lock.drop();
        Self::mode_init()

        // Originally just this...
        // Self::on()?;
    }

    fn on() -> Result<()> {
        WifiResultCode::i32_to_result(unsafe { sys::c::wifi_on(MODE) }, ())
    }

    fn off() -> Result<()> {
        WifiResultCode::i32_to_result(unsafe { sys::c::wifi_off() }, ())
    }

    fn is_running() -> bool {
        let res = unsafe { sys::c::rltk_wlan_running(IDX as u8) };
        res == 1
    }

    // fn send(buf: &mut [u8]) -> Result<()> {
    //     if !Self::is_running() {
    //         return Err(WifiResultCode::NotUp);
    //     }
    //     let inc = RTLTXIncrement::<IDX>::enter();
    //     unsafe {
    //         let skb = alloc_skb!(buf);
    //         if let Some(skb) = skb {
    //             sys::c::rltk_wlan_send_skb(IDX, skb);
    //         } else {
    //             return Err(WifiResultCode::NoMemory);
    //         }
    //     }
    //     Ok(inc.drop())
    // }

    // fn recv() -> Result<String> {
    // fn recv() -> Result<Vec<u8>> {
    //     if !Self::is_running() {
    //         return Err(WifiResultCode::NotUp);
    //     }
    //     let result;
    //     unsafe {
    //         let skb = sys::c::rltk_wlan_get_recv_skb(IDX);
    //         // use alloc::format;
    //         // result = format!("{:?} -> {}", skb, skb.is_null());
    //         if skb.is_null() {
    //             return Err(WifiResultCode::NotReady);
    //         }
    //         result = cpy_vec!((*skb).data, (*skb).len);
    //         // result = format!("{:?}", skb)
    //         if let Some(result) = result {
    //             sys::c::skb_pull(skb, result.len() as u32);
    //             Ok(result)
    //         } else {
    //             Err(WifiResultCode::Associated)
    //         }
    //     }
    //     // Ok(result)
    // }

    fn get_mac_address() -> Result<String> {
        unsafe {
            let mut mac = vec![0u8; 18];
            WifiResultCode::i32_to_result(sys::c::wifi_get_mac_address(mac.as_mut_ptr()), ())?;
            let mac: String = String::from_utf8(mac).unwrap_or_default();
            Ok(mac)
        }
    }
}
