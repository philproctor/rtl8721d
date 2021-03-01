#!/usr/bin/env bash
set -Eeuo pipefail
LWIP_VER="${1}"

echo "This is broken"
exit 1

mkdir -p "lwip/"
echo "Checking if lwip ${LWIP_VER} package is available..."

if [ ! -d "lwip/lwip-${LWIP_VER}/" ]; then
    echo "Not found, retrieving..."
    curl -sL "https://download.savannah.nongnu.org/releases/lwip/lwip-${LWIP_VER}.zip" > "lwip/lwip-${LWIP_VER}.zip"
    unzip "lwip/lwip-${LWIP_VER}.zip" -d "lwip/"
fi

bindgen \
    "inc/bindgen.h" \
    --use-core \
    --with-derive-default \
    --impl-debug \
    --no-doc-comments \
    --no-layout-tests \
    --generate-block \
    --default-enum-style "consts" \
    --ctypes-prefix "ffi::ctypes" \
    --rust-target "nightly" \
    -o "src/lwip-${LWIP_VER}.rs" -- \
    "-Ilwip/lwip-${LWIP_VER}/src/include/" \
    "-Iinc/"
