use super::*;
use alloc::format;

pub async fn command_prompt() {
    info!("Command shell started");
    loop {
        unsafe {
            SERIAL1.tx('#');
            SERIAL1.tx(' ');
            let in_s = SERIAL1.rx_line(true).await;
            let res = run_command(&in_s).await;
            SERIAL1.tx_string(&format!("\n{}", res));
        }
    }
}

async fn run_command(cmd: &str) -> String {
    match cmd {
        "reset" => System::reset(),
        "wifi init" => {
            Network::init();
            Wifi::init().unwrap_or_default();
            "done!".into()
        }
        "wifi connect" => Wifi::connect_wpa2("testing", "12345678")
            .map(|_| "connected!".into())
            .unwrap_or_else(|e| format!("Connect failed: {:?}", e)),
        "wifi scan" => {
            Wifi::wifi_scan();
            "started".into()
        }
        "wifi error" => Wifi::get_last_error(),
        "ip dhcp up" => {
            let r = Network::dhcp(DHCPAction::Start);
            format!("Result: {}", r)
        }
        "ip address" => {
            let ip = Network::get_ip();
            format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])
        }
        "status" => {
            let wifi_on = Wifi::is_connected();
            let mac = Wifi::get_mac_address().unwrap_or_else(|_| "Unknown".into());
            let ts = System::now() / 1000;
            let mem = System::memory_available() / 1024;
            let ip = Network::get_ip();
            format!(
                "System online: {} seconds\r\nWifi online: {:?}\r\nMac address: {}\r\nMemory: {}k\r\nIP Address: {}.{}.{}.{}",
                ts, wifi_on, mac, mem, ip[0], ip[1], ip[2], ip[3]
            )
        }
        "now" => {
            let ts = System::now();
            format!("Current time {}", ts)
        }
        "" => "".into(),
        unk => format!("Unknown command: '{}'", unk),
    }
}
