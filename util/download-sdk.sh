#!/usr/bin/env bash
set -Eeuo pipefail

ARDUINO_SDK_DOWNLOAD="https://github.com/ambiot/ambd_arduino/raw/master/Arduino_package/release/ameba_d-3.0.7-v03.tar.gz"

TOOL_DIR="./tools"
SDK_DIR="${TOOL_DIR}/sdk"
TXT_OK=" [ OK ]"

mkdir -p ${SDK_DIR}

echo -n downloading sdk...
curl -sL "${ARDUINO_SDK_DOWNLOAD}" >"${TOOL_DIR}/ambd_sdk.tar.gz"
echo "${TXT_OK}"
echo -n extracting sdk to ${SDK_DIR}...
tar -xzf "${TOOL_DIR}/ambd_sdk.tar.gz" -C "${SDK_DIR}"
rm "${TOOL_DIR}/ambd_sdk.tar.gz"
echo "${TXT_OK}"
