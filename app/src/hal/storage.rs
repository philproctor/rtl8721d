use crate::prelude::*;
use core::cell::UnsafeCell;

pub static STORAGE: StorageApi<0x0010_0000, 0x0010_1999> = StorageApi::new();

extern "C" {
    #[no_mangle]
    static flash_init_para: c::FLASH_InitTypeDef;
}

unsafe impl<const START: u32, const END: u32> Send for StorageApi<START, END> {}
unsafe impl<const START: u32, const END: u32> Sync for StorageApi<START, END> {}

pub struct StorageApi<const START: u32, const END: u32> {
    handle: UnsafeCell<Option<c::flash_t>>,
}

impl<const START: u32, const END: u32> StorageApi<START, END> {
    pub const fn new() -> Self {
        Self {
            handle: UnsafeCell::new(None),
        }
    }

    pub const fn size(&self) -> u32 {
        END - START
    }

    const fn addr(&self, a: u32) -> u32 {
        START + a
    }

    pub unsafe fn init(&self) {
        *(&mut *self.handle.get()) = Some(c::flash_t {
            SpicInitPara: flash_init_para,
        });
    }

    pub fn status(&self) -> i32 {
        unsafe { c::flash_get_status(&mut (*self.handle.get()).unwrap()) }
    }

    pub fn get_id(&self) -> [u8; 3] {
        let mut result = [0u8; 3];
        unsafe {
            c::flash_read_id(
                core::mem::transmute(self.handle.get()),
                result.as_mut_ptr(),
                3,
            )
        };
        result
    }

    pub fn read(&self, address: u32, size: u32) -> Result<Vec<u8>> {
        let mut result = vec![0u8; size as usize];
        if (address + size) > END {
            return Err(SystemError::OutOfRange);
        }
        unsafe {
            if c::flash_stream_read(
                &mut (*self.handle.get()).unwrap(),
                self.addr(address),
                size,
                result.as_mut_ptr(),
            ) == 1
            {
                Ok(result)
            } else {
                Err(SystemError::Unknown)
            }
        }
    }

    pub fn write(&self, address: u32, data: &[u8]) -> Result<()> {
        let size = data.len();
        if (address + size as u32) > END {
            return Err(SystemError::OutOfRange);
        }
        // unsafe { c::FLASH_Write_Unlock() };
        unsafe {
            c::flash_erase_sector(&mut (*self.handle.get()).unwrap(), self.addr(address));
            if c::flash_stream_write(
                &mut (*self.handle.get()).unwrap(),
                self.addr(address),
                size as u32,
                core::mem::transmute(data.as_ptr()),
            ) == 1
            {
                Ok(())
            } else {
                Err(SystemError::Unknown)
            }
        }
    }

    pub fn erase_all(&self) -> Result<()> {
        let data = vec![255u8; self.size() as usize];
        self.write(0, &data)
    }
}

// pub fn flash_erase_sector(obj: *mut flash_t, address: u32);
// pub fn flash_erase_block(obj: *mut flash_t, address: u32);
// pub fn flash_read_word(obj: *mut flash_t, address: u32, data: *mut u32) -> ffi::ctypes::c_int;
// pub fn flash_write_word(obj: *mut flash_t, address: u32, data: u32) -> ffi::ctypes::c_int;
// pub fn flash_stream_read(
//     obj: *mut flash_t,
//     address: u32,
//     len: u32,
//     data: *mut u8,
// ) -> ffi::ctypes::c_int;
// pub fn flash_stream_write(
//     obj: *mut flash_t,
//     address: u32,
//     len: u32,
//     data: *mut u8,
// ) -> ffi::ctypes::c_int;
// pub fn flash_write_protect(obj: *mut flash_t, protect: u32);
// pub fn flash_get_status(obj: *mut flash_t) -> ffi::ctypes::c_int;
// pub fn flash_get_status2(obj: *mut flash_t) -> ffi::ctypes::c_int;
// pub fn flash_set_status(obj: *mut flash_t, data: u32) -> ffi::ctypes::c_int;
// pub fn flash_set_status2(obj: *mut flash_t, data: u32) -> ffi::ctypes::c_int;
// pub fn flash_reset_status(obj: *mut flash_t);
// pub fn flash_burst_write(
//     obj: *mut flash_t,
//     address: u32,
//     Length: u32,
//     data: *mut u8,
// ) -> ffi::ctypes::c_int;
// pub fn flash_burst_read(
//     obj: *mut flash_t,
//     address: u32,
//     Length: u32,
//     data: *mut u8,
// ) -> ffi::ctypes::c_int;
// pub fn flash_set_extend_addr(obj: *mut flash_t, data: u32) -> ffi::ctypes::c_int;
// pub fn flash_get_extend_addr(obj: *mut flash_t) -> ffi::ctypes::c_int;
// pub fn flash_read_id(obj: *mut flash_t, buf: *mut u8, len: u8) -> ffi::ctypes::c_int;
// pub fn flash_read_unique_id(obj: *mut flash_t, buf: *mut u8, len: u8) -> ffi::ctypes::c_int;
// pub fn flash_set_lock_mode(mode: u32);
// pub fn flash_global_lock();
// pub fn flash_global_unlock();
// pub fn flash_individual_lock(address: u32);
// pub fn flash_individual_unlock(address: u32);
// pub fn flash_read_individual_lock_state(address: u32) -> ffi::ctypes::c_int;
