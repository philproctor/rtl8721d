#!/usr/bin/env bash
set -Eeuo pipefail

TARGET_DIR="./target"
TOOL_ROOT="./tools/bin"
OUT_DIR="${TARGET_DIR}/rtl8721d/bin"
UPLOAD_PORT="/dev/ttyACM1"
AMEBA_UPLOAD_TOOL=$(realpath ${TOOL_ROOT}/tools/linux/image_tool/amebad_image_tool)

(
    cd ${OUT_DIR}
    ${AMEBA_UPLOAD_TOOL} ${UPLOAD_PORT}
)
