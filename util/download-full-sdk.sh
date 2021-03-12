#!/usr/bin/env bash
set -Eeuo pipefail

COMPLETE_SDK_DOWNLOAD="https://github.com/ambiot/ambd_sdk/archive/master.zip"

TOOL_DIR="./tools"
COMPLETE_SDK_DIR="${TOOL_DIR}/complete-sdk"
TXT_OK=" [ OK ]"

mkdir -p ${COMPLETE_SDK_DIR}
echo "downloading complete sdk (this may take awhile)..."
curl -L "${COMPLETE_SDK_DOWNLOAD}" >"${TOOL_DIR}/sdk-complete.zip"
echo "${TXT_OK}"
echo -n "Extracting sdk to ${COMPLETE_SDK_DIR}..."
(cd "${COMPLETE_SDK_DIR}" && unzip ../sdk-complete.zip)
rm "${TOOL_DIR}/sdk-complete.zip"
echo "${TXT_OK}"
