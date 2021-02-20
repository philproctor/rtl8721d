pub mod c {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    #![allow(safe_packed_borrows)]
    #![allow(improper_ctypes)]

    use crate::ffi;

    include!("bindings.rs");
}

pub mod ctypes;
