#!/usr/bin/env bash
set -Eeuo pipefail

rm -f tools/lib-rtl8721d.a
ar -rcT tools/lib-rtl8721d.a \
    target/thumbv8m.main-none-eabihf/debug/*.rlib \
    tools/sdk/hardware/variants/rtl8721d/*.a \
    tools/toolchain/arm-none-eabi/lib/v8-m.main/fpu/fpv5-sp-d16/*.a \
    tools/toolchain/arm-none-eabi/lib/v8-m.main/fpu/fpv5-sp-d16/*.o
