// use std::env;
// use std::process::Command;

fn main() {
    static SDK_PATH: &str = "../tools/sdk/hardware/system/libameba/sdk";
    let common_path = format!("{}/component/common", SDK_PATH);
    let bt_path = format!("{}/bluetooth/realtek/sdk", common_path);
    let lwip_path = format!("{}/network/lwip/lwip_v2.0.2", common_path);
    let freertos_path = format!("{}/component/os/freertos", SDK_PATH);
    let amebad_path = format!("{}/component/soc/realtek/amebad", SDK_PATH);
    // let tools_path = format!("../tools/sdk/hardware/variants/rtl8721d");

    //Thin package libraries
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    // let rout_dir = out_dir.to_string_lossy();
    // let res = Command::new("sh")
    //     .args(&[
    //         format!("-c"),
    //         format!(
    //             "rm -f {}/lib-rtl8721d.a && arm-none-eabi-ar -rcT {}/lib-rtl8721d.a {}/*.a",
    //             rout_dir, rout_dir, tools_path
    //         ),
    //     ])
    //     .output()
    //     .unwrap();
    // if !res.status.success() {
    //     panic!(
    //         "Failed! {} {}",
    //         String::from_utf8(res.stdout).unwrap(),
    //         String::from_utf8(res.stderr).unwrap()
    //     );
    // }

    //Build main.o
    cc::Build::new()
        .file("c/src/main.c")
        .flag("-nostartfiles")
        .flag("-march=armv8-m.main+dsp")
        .flag("-mthumb")
        .flag("-mcmse")
        .define("CONFIG_PLATFORM_8721D", "1")
        .includes(&[
            format!("c/inc"),
            format!("{}/api", common_path),
            format!("{}/api/at_cmd", common_path),
            format!("{}/api/network/include", common_path),
            format!("{}/api/platform", common_path),
            format!("{}/api/wifi", common_path),
            format!("{}/api/wifi/rtw_wpa_supplicant/src", common_path),
            format!("{}/api/wifi/rtw_wpa_supplicant/src/crypto", common_path),
            format!("{}/application", common_path),
            format!("{}/application/xmodem", common_path),
            format!("{}/audio", common_path),
            format!("{}/board/amebad/lib", bt_path),
            format!("{}/board/amebad/src", bt_path),
            format!("{}/board/amebad/src/vendor_cmd", bt_path),
            format!("{}/board/common/inc", bt_path),
            format!("{}/example/bt_config", bt_path),
            format!("{}/inc/app", bt_path),
            format!("{}/inc/bluetooth/gap", bt_path),
            format!("{}/inc/bluetooth/profile", bt_path),
            format!("{}/inc/bluetooth/profile/client", bt_path),
            format!("{}/inc/bluetooth/profile/server", bt_path),
            format!("{}/inc/os", bt_path),
            format!("{}/inc/platform", bt_path),
            format!("{}/inc/stack", bt_path),
            format!("{}/src/app/ble_central_client", bt_path),
            format!("{}/src/mcu/module/data_uart_cmd", bt_path),
            format!("{}/drivers/i2s", common_path),
            format!("{}/drivers/ir/protocol", common_path),
            format!("{}/drivers/sdio/realtek/sdio_host/inc", common_path),
            format!("{}/drivers/wlan/realtek/include", common_path),
            format!("{}/drivers/wlan/realtek/src/core/option", common_path),
            format!("{}/drivers/wlan/realtek/src/hal", common_path),
            format!("{}/drivers/wlan/realtek/src/hal/phydm", common_path),
            format!("{}/drivers/wlan/realtek/src/hci", common_path),
            format!("{}/drivers/wlan/realtek/src/osdep", common_path),
            format!("{}/file_system/ftl", common_path),
            format!("{}/mbed/api", common_path),
            format!("{}/mbed/hal", common_path),
            format!("{}/mbed/hal_ext", common_path),
            format!("{}/mbed/targets/hal/rtl8721d", common_path),
            format!("{}/media/framework", common_path),
            format!("{}/media/rtp_codec", common_path),
            format!("{}/network", common_path),
            format!("{}/port/realtek", lwip_path),
            format!("{}/port/realtek/freertos", lwip_path),
            format!("{}/src/include", lwip_path),
            format!("{}/src/include/lwip", lwip_path),
            format!("{}/network/mDNS", common_path),
            format!("{}/network/ssl/mbedtls-2.4.0/include", common_path),
            format!("{}/network/ssl/ssl_ram_map/rom", common_path),
            format!("{}/utilities", common_path),
            format!("{}/video/v4l2/inc", common_path),
            format!("{}/component/os/os_dep/include", SDK_PATH),
            format!("{}/app/monitor/include", amebad_path),
            format!("{}/app/xmodem", amebad_path),
            format!("{}/cmsis", amebad_path),
            format!("{}/fwlib", amebad_path),
            format!("{}/fwlib/include", amebad_path),
            format!("{}/misc", amebad_path),
            format!("{}/swlib/string", amebad_path),
            format!("{}", freertos_path),
            format!("{}/freertos_v10.2.0/Source/include", freertos_path),
            format!(
                "{}/freertos_v10.2.0/Source/portable/GCC/ARM_CM33/non_secure",
                freertos_path
            ),
        ])
        // .object(format!("{}/btgap.a", tools_path))
        // .object(format!("{}/lib_arduino.a", tools_path))
        // .object(format!("{}/lib_arduino_bt.a", tools_path))
        // .object(format!("{}/lib_arduino_mbedtls240.a", tools_path))
        // .object(format!("{}/lib_cmsis_dsp.a", tools_path))
        // .object(format!("{}/lib_coap.a", tools_path))
        // .object(format!("{}/lib_dct.a", tools_path))
        // .object(format!("{}/lib_eap.a", tools_path))
        // .object(format!("{}/lib_httpc.a", tools_path))
        // .object(format!("{}/lib_httpd.a", tools_path))
        // .object(format!("{}/lib_m4a_self.a", tools_path))
        // .object(format!("{}/lib_mdns.a", tools_path))
        // .object(format!("{}/lib_mmfv2.a", tools_path))
        // .object(format!("{}/lib_rtp_codec.a", tools_path))
        // .object(format!("{}/lib_rtsp.a", tools_path))
        // .object(format!("{}/lib_tftp.a", tools_path))
        // .object(format!("{}/lib_usbd.a", tools_path))
        // .object(format!("{}/lib_usbh.a", tools_path))
        // .object(format!("{}/lib_user.a", tools_path))
        // .object(format!("{}/lib_v4l2.a", tools_path))
        // .object(format!("{}/lib_websocket.a", tools_path))
        // .object(format!("{}/lib_wifi_fw.a", tools_path))
        // .object(format!("{}/lib_wifi_ucps_fw.a", tools_path))
        // .object(format!("{}/lib_wlan.a", tools_path))
        // .object(format!("{}/lib_wps.a", tools_path))
        .static_flag(true)
        .opt_level(2)
        .compile("rtl-sys");
}
