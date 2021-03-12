// .flag("-specs")
// .flag("nosys.specs")
// .flag("-mfloat-abi=hard")
// .flag("-mfpu=fpv5-sp-d16")
// .flag("-Wl,--gc-sections")
// .flag("-Wl,--cref")
// .flag("-Wl,--build-id=none")
// .flag("-Wl,--allow-multiple-definition")
// .flag("-lm")
// .flag("-Wl,-wrap,memset")

const SDK_PATH: &str = "../tools/sdk/hardware/system/libameba/sdk";

fn main() {
    cc::Build::new()
        .file("c/src/main.c")
        .flag("-march=armv8-m.main+dsp")
        .flag("-nostartfiles")
        .flag("-mthumb")
        .flag("-mcmse")
        .flag("-specs")
        .flag("nosys.specs")
        .flag("-mfloat-abi=hard")
        .flag("-mfpu=fpv5-sp-d16")
        .flag("-Wl,--gc-sections")
        .flag("-Wl,--cref")
        .flag("-Wl,--build-id=none")
        .flag("-Wl,--allow-multiple-definition")
        .flag("-lm")
        .flag("-Wl,-wrap,memset")
        .define("CONFIG_PLATFORM_8721D", "1")
        .includes(&[
            format!("c/inc"),
            format!("{}/component/common/api", SDK_PATH),
            format!("{}/component/common/api/at_cmd", SDK_PATH),
            format!("{}/component/common/api/network/include", SDK_PATH),
            format!("{}/component/common/api/platform", SDK_PATH),
            format!("{}/component/common/api/wifi", SDK_PATH),
            format!(
                "{}/component/common/api/wifi/rtw_wpa_supplicant/src",
                SDK_PATH
            ),
            format!(
                "{}/component/common/api/wifi/rtw_wpa_supplicant/src/crypto",
                SDK_PATH
            ),
            format!("{}/component/common/application", SDK_PATH),
            format!("{}/component/common/application/xmodem", SDK_PATH),
            format!("{}/component/common/audio", SDK_PATH),
            format!(
                "{}/component/common/bluetooth/realtek/sdk/board/amebad/lib",
                SDK_PATH
            ),
            format!(
                "{}/component/common/bluetooth/realtek/sdk/board/amebad/src",
                SDK_PATH
            ),
            format!(
                "{}/component/common/bluetooth/realtek/sdk/board/amebad/src/vendor_cmd",
                SDK_PATH
            ),
            format!(
                "{}/component/common/bluetooth/realtek/sdk/board/common/inc",
                SDK_PATH
            ),
            format!(
                "{}/component/common/bluetooth/realtek/sdk/example/bt_config",
                SDK_PATH
            ),
            format!(
                "{}/component/common/bluetooth/realtek/sdk/inc/app",
                SDK_PATH
            ),
            format!(
                "{}/component/common/bluetooth/realtek/sdk/inc/bluetooth/gap",
                SDK_PATH
            ),
            format!(
                "{}/component/common/bluetooth/realtek/sdk/inc/bluetooth/profile",
                SDK_PATH
            ),
            format!(
                "{}/component/common/bluetooth/realtek/sdk/inc/bluetooth/profile/client",
                SDK_PATH
            ),
            format!(
                "{}/component/common/bluetooth/realtek/sdk/inc/bluetooth/profile/server",
                SDK_PATH
            ),
            format!("{}/component/common/bluetooth/realtek/sdk/inc/os", SDK_PATH),
            format!(
                "{}/component/common/bluetooth/realtek/sdk/inc/platform",
                SDK_PATH
            ),
            format!(
                "{}/component/common/bluetooth/realtek/sdk/inc/stack",
                SDK_PATH
            ),
            format!(
                "{}/component/common/bluetooth/realtek/sdk/src/app/ble_central_client",
                SDK_PATH
            ),
            format!(
                "{}/component/common/bluetooth/realtek/sdk/src/mcu/module/data_uart_cmd",
                SDK_PATH
            ),
            format!("{}/component/common/drivers/i2s", SDK_PATH),
            format!("{}/component/common/drivers/ir/protocol", SDK_PATH),
            format!(
                "{}/component/common/drivers/sdio/realtek/sdio_host/inc",
                SDK_PATH
            ),
            format!("{}/component/common/drivers/wlan/realtek/include", SDK_PATH),
            format!(
                "{}/component/common/drivers/wlan/realtek/src/core/option",
                SDK_PATH
            ),
            format!("{}/component/common/drivers/wlan/realtek/src/hal", SDK_PATH),
            format!(
                "{}/component/common/drivers/wlan/realtek/src/hal/phydm",
                SDK_PATH
            ),
            format!("{}/component/common/drivers/wlan/realtek/src/hci", SDK_PATH),
            format!(
                "{}/component/common/drivers/wlan/realtek/src/osdep",
                SDK_PATH
            ),
            format!("{}/component/common/example", SDK_PATH),
            format!("{}/component/common/example/wlan_fast_connect", SDK_PATH),
            format!("{}/component/common/file_system/fatfs", SDK_PATH),
            format!(
                "{}/component/common/file_system/fatfs/r0.10c/include",
                SDK_PATH
            ),
            format!("{}/component/common/file_system/ftl", SDK_PATH),
            format!("{}/component/common/mbed/api", SDK_PATH),
            format!("{}/component/common/mbed/hal", SDK_PATH),
            format!("{}/component/common/mbed/hal_ext", SDK_PATH),
            format!("{}/component/common/mbed/targets/hal/rtl8721d", SDK_PATH),
            format!("{}/component/common/media/framework", SDK_PATH),
            format!("{}/component/common/media/rtp_codec", SDK_PATH),
            format!("{}/component/common/network", SDK_PATH),
            format!(
                "{}/component/common/network/lwip/lwip_v2.0.2/port/realtek",
                SDK_PATH
            ),
            format!(
                "{}/component/common/network/lwip/lwip_v2.0.2/port/realtek/freertos",
                SDK_PATH
            ),
            format!(
                "{}/component/common/network/lwip/lwip_v2.0.2/src/include",
                SDK_PATH
            ),
            format!(
                "{}/component/common/network/lwip/lwip_v2.0.2/src/include/lwip",
                SDK_PATH
            ),
            format!("{}/component/common/network/mDNS", SDK_PATH),
            format!(
                "{}/component/common/network/ssl/mbedtls-2.4.0/include",
                SDK_PATH
            ),
            format!("{}/component/common/network/ssl/ssl_ram_map/rom", SDK_PATH),
            format!("{}/component/common/test", SDK_PATH),
            format!("{}/component/common/utilities", SDK_PATH),
            format!("{}/component/common/video/v4l2/inc", SDK_PATH),
            format!("{}/component/os/freertos", SDK_PATH),
            format!(
                "{}/component/os/freertos/freertos_v10.2.0/Source/include",
                SDK_PATH
            ),
            format!(
                "{}/component/os/freertos/freertos_v10.2.0/Source/portable/GCC/ARM_CM33/non_secure",
                SDK_PATH
            ),
            format!("{}/component/os/os_dep/include", SDK_PATH),
            format!(
                "{}/component/soc/realtek/amebad/app/monitor/include",
                SDK_PATH
            ),
            format!("{}/component/soc/realtek/amebad/app/xmodem", SDK_PATH),
            format!("{}/component/soc/realtek/amebad/cmsis", SDK_PATH),
            format!("{}/component/soc/realtek/amebad/fwlib", SDK_PATH),
            format!("{}/component/soc/realtek/amebad/fwlib/include", SDK_PATH),
            format!("{}/component/soc/realtek/amebad/misc", SDK_PATH),
            format!("{}/component/soc/realtek/amebad/swlib/string", SDK_PATH),
        ])
        .static_flag(true)
        .opt_level(2)
        .compile("rust8720");
}
