#!/usr/bin/env bash
set -Eeuo pipefail

rm -f tools/lib-rtl8721d.a
# ar -rcT tools/lib-rtl8721d.a tools/sdk/hardware/variants/rtl8721d/*.a
