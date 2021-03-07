use crate::prelude::*;

pub async fn command_prompt() {
    info!("Command shell started");
    loop {
        WifiClient::get_mac_address().unwrap_or_else(|_| "Unknown".into());
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
        "status" => {
            let mac = WifiClient::get_mac_address().unwrap_or_else(|_| "Unknown".into());
            let ts = System::now() / 1000;
            let mem = System::memory_available() / 1024;
            let ip = LwipInterface::get_ip(0);
            format!(
                "System online: {} seconds\r\n\r\nMac address: {}\r\nMemory: {}k\r\nIP Address: {}",
                ts, mac, mem, ip
            )
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
        "config load" => {
            CONFIG.load().unwrap_or_default();
            format!(
                "SSID selected: {} WifiMode: {:?} Password: {}",
                CONFIG.get_ssid(),
                CONFIG.get_wifi_mode(),
                CONFIG.get_password()
            )
        }
        "config save" => {
            CONFIG.set_wifi_mode(crate::config::WifiMode::WPA2);
            CONFIG.set_password("TEST".into());
            CONFIG.set_ssid("TEST".into());
            CONFIG.save().unwrap_or_default();
            format!(
                "SSID selected: {} WifiMode: {:?} Password: {}",
                CONFIG.get_ssid(),
                CONFIG.get_wifi_mode(),
                CONFIG.get_password()
            )
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
            let buf = STORAGE.read(0, 100);
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
        "" => "".into(),
        unk => format!("Unknown command: '{}'", unk),
    }
}
