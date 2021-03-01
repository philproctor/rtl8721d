mod command;

pub use command::command_prompt;

// pub extern "C" fn debug_rxtx() {
//     info!("Receiver thread started");
//     loop {
//         match rtl8720_wlan::WifiClient::recv() {
//             Ok(vu8) => {
//                 // debug!("Received string {}", vu8);
//                 debug!("Received packet of size {}", vu8.len());
//                 System::sleep(1000);
//             }

//             Err(WifiResultCode::Associated) => {
//                 debug!("Packet dropped?");
//                 System::sleep(1000);
//             }

//             o => {
//                 RTOS::do_yield();
//                 debug!("Err: {:?}", o);
//                 System::sleep(1000);
//             }
//         }
//     }
// }
