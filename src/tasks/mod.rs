use crate::prelude::*;

pub use command::command_prompt;
// pub use serial::{serial_rx_task, serial_tx_task};

mod command;

// pub async fn serial_tx_task() {
//     let mut s = Serial::new();
//     s.tx_string("Serial TX");

//     loop {
//         let ticks = RTOS::ticks_elapsed();
//         let upd_str = format!("Ticks: {}", ticks);
//         s.tx_string(&upd_str);
//         let ms = RTOS::ms_elapsed();
//         let upd_str = format!("ms: {}", ms);
//         s.tx_string(&upd_str);
//         Delay::new(10000).await;
//     }
// }
