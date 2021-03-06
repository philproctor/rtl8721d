#!/usr/bin/env bash
set -Eeuo pipefail
bindgen \
    "c/inc/bindgen.h" \
    --use-core \
    --with-derive-default \
    --impl-debug \
    --whitelist-function '(serial_|rtl_|rltk_|wifi_|wext_|sys_|device|init|Psram_|app_|flash_|FLASH_|gtimer_).*' \
    --whitelist-function '.*(Port|Task|Queue).*' \
    --whitelist-var '(RTW|rtw|RT|rt)_.*' \
    --whitelist-type '(RTW|rtw|RT|rt|_RT|ip_|ip4)_.*' \
    --whitelist-var 'DISABLE' \
    --no-doc-comments \
    --no-layout-tests \
    --no-prepend-enum-name \
    --fit-macro-constant-types \
    --size_t-is-usize \
    --generate-block \
    --default-enum-style "consts" \
    --ctypes-prefix "crate::ctypes" \
    --rust-target "nightly" \
    -o src/bindings.rs -- \
        "-I." \
        "-Ic/inc/" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/os/freertos" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/os/freertos/freertos_v10.2.0/Source/include" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/os/freertos/freertos_v10.2.0/Source/portable/GCC/ARM_CM33/non_secure" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/os/os_dep/include" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/os/customer_rtos" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/soc/realtek/amebad/misc" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/api/network/include" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/api" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/api/at_cmd" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/api/platform" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/api/wifi" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/api/wifi/rtw_wpa_supplicant/src" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/api/wifi/rtw_wpa_supplicant/src/crypto" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/application" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/media/framework" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/example" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/example/wlan_fast_connect" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/mbed/api" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/mbed/hal" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/mbed/hal_ext" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/mbed/targets/hal/rtl8721d" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/network" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/network/lwip/lwip_v2.0.2/port/realtek/freertos" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/network/lwip/lwip_v2.0.2/src/include" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/network/lwip/lwip_v2.0.2/src/include/lwip" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/network/lwip/lwip_v2.0.2/port/realtek" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/test" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/soc/realtek/amebad/cmsis" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/soc/realtek/amebad/fwlib" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/soc/realtek/amebad/misc" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/drivers/wlan/realtek/include" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/drivers/wlan/realtek/src/osdep" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/drivers/wlan/realtek/src/hci" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/network/ssl/ssl_ram_map/rom" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/utilities" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/video/v4l2/inc" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/media/rtp_codec" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/file_system/fatfs" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/file_system/fatfs/r0.10c/include" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/file_system/ftl" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/drivers/sdio/realtek/sdio_host/inc" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/audio" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/drivers/i2s" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/application/xmodem" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/network/mDNS" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/soc/realtek/amebad/fwlib/include" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/soc/realtek/amebad/swlib/string" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/soc/realtek/amebad/app/monitor/include" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/soc/realtek/amebad/app/xmodem" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/network/ssl/mbedtls-2.4.0/include" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/bluetooth/realtek/sdk/board/amebad/lib" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/bluetooth/realtek/sdk/board/amebad/src" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/bluetooth/realtek/sdk/board/amebad/src/vendor_cmd" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/bluetooth/realtek/sdk/board/common/inc" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/bluetooth/realtek/sdk/example/bt_config" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/bluetooth/realtek/sdk/inc/app" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/bluetooth/realtek/sdk/inc/bluetooth/gap" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/bluetooth/realtek/sdk/inc/bluetooth/profile" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/bluetooth/realtek/sdk/inc/bluetooth/profile/client" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/bluetooth/realtek/sdk/inc/bluetooth/profile/server" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/bluetooth/realtek/sdk/inc/os" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/bluetooth/realtek/sdk/inc/platform" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/bluetooth/realtek/sdk/inc/stack" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/bluetooth/realtek/sdk/src/app/ble_central_client" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/bluetooth/realtek/sdk/src/mcu/module/data_uart_cmd" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/drivers/ir/protocol" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/drivers/wlan/realtek/src/core/option" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/drivers/wlan/realtek/src/hal" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/drivers/wlan/realtek/src/hal/phydm" \
        "-I../tools/sdk/hardware/system/libameba/sdk/component/common/network/ssl/mbedtls-2.4.0/include" \
        "-I../tools/toolchain/lib/gcc/arm-none-eabi/6.5.0/include" \
        "-I../tools/toolchain/lib/gcc/arm-none-eabi/6.5.0/include-fixed" \
        "-I../tools/toolchain/arm-none-eabi/include" \
        "-DARDUINO_SDK" \
        "-DCONFIG_PLATFORM_8721D" \
        "-DCONFIG_USE_MBEDTLS_ROM_ALG" \
        "-DCONFIG_FUNCION_O0_OPTIMIZE" \
        "-DDM_ODM_SUPPORT_TYPE=32" \
        "-D__CMSIS_GCC_H" \
        "-D_TRUSTZONE_H_" \
        "-D_STRPROC_H_" \
        "-mthumb" \
        "-w"
