macro_rules! info {
    ($( $content:tt )*) => {{
        use crate::hal::SERIAL1;
        use alloc::format;
        let mut msg = format!($( $content )*);
        msg = format!("[INFO]: {}", msg);
        SERIAL1.tx_string(&msg);
    }}
}

#[allow(unused_macros)]
macro_rules! debug {
    ($( $content:tt )*) => {{
        use crate::hal::SERIAL1;
        let mut msg = format!($( $content )*);
        msg = format!("[DBG ]: {}", msg);
        SERIAL1.tx_string(&msg);
    }}
}

#[allow(unused_macros)]
macro_rules! warn {
    ($( $content:tt )*) => {{
        use crate::hal::SERIAL1;
        let mut msg = format!($( $content )*);
        msg = format!("[WARN]: {}", msg);
        SERIAL1.tx_string(&msg);
    }}
}

#[allow(unused_macros)]
macro_rules! crit {
    ($( $content:tt )*) => {{
        use crate::hal::SERIAL1;
        let mut msg = format!($( $content )*);
        msg = format!("[CRIT]: {}", msg);
        SERIAL1.tx_string(&msg);
    }}
}
