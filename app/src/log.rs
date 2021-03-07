macro_rules! info {
    ($( $content:tt )*) => {{
        use alloc::format;
        let mut msg = format!($( $content )*);
        msg = format!("[INFO]: {}", msg);
        SERIAL1.tx_string(&msg);
    }}
}

#[allow(unused_macros)]
macro_rules! debug {
    ($( $content:tt )*) => {{
        use alloc::format;
        let mut msg = format!($( $content )*);
        msg = format!("[DEBG]: {}", msg);
        SERIAL1.tx_string(&msg);
    }}
}

#[allow(unused_macros)]
macro_rules! warn {
    ($( $content:tt )*) => {{
        use alloc::format;
        let mut msg = format!($( $content )*);
        msg = format!("[WARN]: {}", msg);
        SERIAL1.tx_string(&msg);
    }}
}

#[allow(unused_macros)]
macro_rules! crit {
    ($( $content:tt )*) => {{
        use alloc::format;
        let mut msg = format!($( $content )*);
        msg = format!("[CRIT]: {}", msg);
        SERIAL1.tx_string(&msg);
    }}
}
