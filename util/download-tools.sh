#!/usr/bin/env bash
set -Eeuo pipefail

ARDUINO_SDK_DOWNLOAD="https://github.com/ambiot/ambd_arduino/raw/master/Arduino_package/release/ameba_d-3.0.7-v02.tar.gz"
ARDUINO_TOOLS_DOWNLOAD="https://github.com/ambiot/ambd_arduino/raw/master/Arduino_package/release/ameba_d_tools_linux-1.0.5.tar.gz"
ARDUINO_TOOLCHAIN_DOWNLOAD="https://github.com/ambiot/ambd_arduino/raw/master/Arduino_package/release/ameba_d_toolchain_linux_64-1.0.1.tar.bz2"
COMPLETE_SDK_DOWNLOAD="https://github.com/ambiot/ambd_sdk/archive/master.zip"

TOOL_DIR="./tools"
SDK_DIR="${TOOL_DIR}/sdk"
TOOLCHAIN_DIR="${TOOL_DIR}/toolchain"
TOOL_BIN_DIR="${TOOL_DIR}/bin"
TXT_OK=" [ OK ]"

mkdir -p ${SDK_DIR} ${TOOLCHAIN_DIR} ${TOOL_BIN_DIR}

echo -n downloading toolchain...
curl -sL "${ARDUINO_TOOLCHAIN_DOWNLOAD}" >"${TOOL_DIR}/toolchain.tar.bz2"
echo "${TXT_OK}"
echo -n extracting toolchain to ${TOOLCHAIN_DIR}...
tar -xjf "${TOOL_DIR}/toolchain.tar.bz2" -C "${TOOLCHAIN_DIR}" --strip-components=1
rm "${TOOL_DIR}/toolchain.tar.bz2"
echo "${TXT_OK}"

echo -n downloading tools...
curl -sL "${ARDUINO_TOOLS_DOWNLOAD}" >"${TOOL_DIR}/tools.tar.gz"
echo "${TXT_OK}"
echo -n extracting tools to ${TOOL_BIN_DIR}...
tar -xzf "${TOOL_DIR}/tools.tar.gz" -C "${TOOL_BIN_DIR}" --strip-components=1
rm "${TOOL_DIR}/tools.tar.gz"
echo "${TXT_OK}"

echo -n downloading sdk...
curl -sL "${ARDUINO_SDK_DOWNLOAD}" >"${TOOL_DIR}/ambd_sdk.tar.gz"
echo "${TXT_OK}"
echo -n extracting sdk to ${SDK_DIR}...
tar -xzf "${TOOL_DIR}/ambd_sdk.tar.gz" -C "${SDK_DIR}"
rm "${TOOL_DIR}/ambd_sdk.tar.gz"
echo "${TXT_OK}"
