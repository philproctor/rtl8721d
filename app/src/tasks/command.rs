use crate::prelude::*;
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
            WifiClient::init().unwrap_or_default();
            "done!".into()
        }
        "tasks" => {
            let count;
            let mut task_status;
            let mut runtime = 0;
            let mut result = String::new();
            unsafe {
                count = c::uxTaskGetNumberOfTasks();
                task_status = vec![Default::default(); count as usize];
                c::uxTaskGetSystemState(task_status.as_mut_ptr(), count, &mut runtime);
                for task in task_status.iter() {
                    let name = cstr_core::CStr::from_ptr(task.pcTaskName.cast());
                    let task_rt = task.ulRunTimeCounter;
                    let task_hwm = task.usStackHighWaterMark;
                    result.push_str(&format!(
                        "Name: {} -> Runtime: {} -> HWM: {}\r\n",
                        name.to_str().unwrap_or_default(),
                        task_rt,
                        task_hwm,
                    ));
                }
            }
            format!(
                "Task Count: {}\r\nTasks: {:?}\r\nRuntime: {}\r\nDetails: {}",
                count, task_status, runtime, result
            )
        }
        // "wifi send" => {
        //     let mut dat = vec![1, 2, 3, 4, 5];
        //     if let Ok(_) = WifiClient::send(dat.as_mut_slice()) {
        //         "Sent".into()
        //     } else {
        //         "failed".into()
        //     }
        // }
        "wifi connect" => WifiClient::connect_wpa2("DragonsDen", "ravenwood5068")
            .map(|_| "connected!".into())
            .unwrap_or_else(|e| format!("Connect failed: {:?}", e)),
        // "wifi scan" => {
        //     WifiClient::wifi_scan();
        //     "started".into()
        // }
        // "wifi error" => WifiClient::get_last_error(),
        "ip dhcp up" => {
            let r = Network::dhcp(DHCPAction::Start);
            format!("Result: {}", r)
        }
        "ip address" => Network::get_ip().ip_string(),
        "status" => {
            // let wifi_on = WifiClient::is_connected();
            let mac = WifiClient::get_mac_address().unwrap_or_else(|_| "Unknown".into());
            let ts = System::now() / 1000;
            let mem = System::memory_available() / 1024;
            let ip = Network::get_ip().ip_string();
            format!(
                "System online: {} seconds\r\n\r\nMac address: {}\r\nMemory: {}k\r\nIP Address: {}",
                ts, mac, mem, ip
            )
        }
        "now" => {
            let ts = System::now();
            format!("Current time {}", ts)
        }
        "psram init" => {
            unsafe {
                crate::ffi::c::app_init_psram();
                crate::ffi::c::Psram_heap_init();
            }
            "OK".into()
        }
        "psram status" => {
            let free = unsafe { crate::ffi::c::Psram_reserve_free_size() };
            format!("Free memory: {}", free)
        }
        "flash status" => {
            let status = STORAGE.status();
            format!("Flash status: {}", status)
        }
        "flash id" => {
            let id = STORAGE.get_id();
            format!("Flash ID: {}:{}:{}", id[0], id[1], id[2])
        }
        "flash read" => {
            let buf = STORAGE.read(0, 10);
            format!("contents: {:?}", buf)
        }
        "flash write" => {
            let res = STORAGE.write(0, &[1, 2, 3, 4, 5, 6]);
            format!("result: {:?}", res)
        }
        "flash erase" => {
            let res = STORAGE.erase_all();
            format!("result: {:?}", res)
        }
        "dns" => {
            let mut host = Host::UnresolvedHost("www.google.com".into());
            Network::resolve_host(&mut host).await;
            format!("host: {}", host.ip_string())
        }
        "connect" => {
            let socket = TCP::connect(Host::IPAddress([172, 16, 1, 169]), 2000)
                .await
                .unwrap();
            socket
                .write_all(b"This is a test message")
                .await
                .unwrap_or_default();
            let res1 = socket.read(100).await.unwrap_or_default();
            let str = String::from_utf8(res1).unwrap_or_default();
            format!("data received: {}", str)
        }
        "http" => HttpRequest::new(
            Host::UnresolvedHost("httpbin.org".into()),
            HttpMethod::GET,
            "/get?param=somevalue".into(),
            vec![],
        )
        .send()
        .await
        .unwrap_or_else(|e| format!("Failed! {:?}", e))
        .replace("\n", "\r\n"),
        "" => "".into(),
        unk => format!("Unknown command: '{}'", unk),
    }
}
