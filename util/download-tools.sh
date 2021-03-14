#!/usr/bin/env bash
set -Eeuo pipefail

ARDUINO_TOOLS_DOWNLOAD="https://github.com/ambiot/ambd_arduino/raw/master/Arduino_package/release/ameba_d_tools_linux-1.0.5.tar.gz"

TOOL_DIR="./tools"
TOOL_BIN_DIR="${TOOL_DIR}/bin"
TXT_OK=" [ OK ]"

mkdir -p ${TOOL_BIN_DIR}

echo -n downloading tools...
curl -sL "${ARDUINO_TOOLS_DOWNLOAD}" >"${TOOL_DIR}/tools.tar.gz"
echo "${TXT_OK}"
echo -n extracting tools to ${TOOL_BIN_DIR}...
tar -xzf "${TOOL_DIR}/tools.tar.gz" -C "${TOOL_BIN_DIR}" --strip-components=1
rm "${TOOL_DIR}/tools.tar.gz"
echo "${TXT_OK}"
