pub use crate::prelude::*;
pub use rtl8720_wlan::WifiClient;

pub async fn debug_task() {
    loop {
        WifiClient::get_mac_address().unwrap_or_default();
        // rltk_wlan_control
        Delay::new(10).await;
    }
}
