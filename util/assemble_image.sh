#!/usr/bin/env bash
set -Eeuo pipefail

TARGET_DIR="./target"
OUT_DIR="${TARGET_DIR}/rtl8721d/bin"
TOOL_ROOT="./tools/bin"
TOOL_DIR="${TOOL_ROOT}/tools/linux"
BSP_DIR="${TOOL_ROOT}/bsp/image/PMU_bins/NONE"

NM="arm-none-eabi-nm"
OBJDUMP="arm-none-eabi-objdump"
OBJCOPY="arm-none-eabi-objcopy"
PICK="${TOOL_DIR}/pick"
PAD="${TOOL_DIR}/pad"

AXF_FILE="${TARGET_DIR}/rtl8721d.axf"
ASM_FILE="${TARGET_DIR}/rtl8721d.asm"
MAP_FILE="${TARGET_DIR}/rtl8721d.map"
NMAP_FILE="${TARGET_DIR}/rtl8721d.nmap"

IMGTOOL_BIN="${TOOL_ROOT}/bsp/image/imgtool_flashloader_amebad.bin"

TXT_OK="[ OK ]"

##################

mkdir -p "${OUT_DIR}"

echo -n generating ${NMAP_FILE}...
${NM} ${AXF_FILE} | sort > ${NMAP_FILE}
echo ${TXT_OK}

echo -n "generating ${ASM_FILE}..."
${OBJDUMP} -d ${AXF_FILE} > ${ASM_FILE}
echo ${TXT_OK}

echo  preparing output binaries...

${OBJCOPY} -j .ram_image2.entry -j .ram_image2.text -j .ram_image2.data -Obinary ${AXF_FILE} ${OUT_DIR}/ram_2.r.bin
${OBJCOPY} -j .xip_image2.text  -Obinary ${AXF_FILE} ${OUT_DIR}/xip_image2.bin
${OBJCOPY} -j .psram_image2.text -j .psram_image2.data -Obinary ${AXF_FILE} ${OUT_DIR}/psram_2.r.bin

RAM_IMAGE2_START=$(grep '__ram_image2_text_start__' ${NMAP_FILE} | awk '{print "0x"$1}') ;  \
RAM_IMAGE2_END=$(grep '__ram_image2_text_end__' ${NMAP_FILE} | awk '{print "0x"$1}') ;  \
${PICK} $RAM_IMAGE2_START $RAM_IMAGE2_END ${OUT_DIR}/ram_2.r.bin ${OUT_DIR}/ram_2.bin raw ; \
${PICK} $RAM_IMAGE2_START $RAM_IMAGE2_END ${OUT_DIR}/ram_2.bin ${OUT_DIR}/ram_2.p.bin

FLASH_START=$(grep '__flash_text_start__' ${NMAP_FILE} | awk '{print "0x"$1}') ; \
FLASH_END=$(grep '__flash_text_end__' ${NMAP_FILE} | awk '{print "0x"$1}') ; \
${PICK} $FLASH_START $FLASH_END ${OUT_DIR}/xip_image2.bin ${OUT_DIR}/xip_image2.p.bin

PSRAM_IMAGE2_START=$(grep '__psram_image2_text_start__' ${NMAP_FILE} | awk '{print "0x"$1}') ; \
PSRAM_IMAGE2_END=$(grep '__psram_image2_text_end__' ${NMAP_FILE} | awk '{print "0x"$1}') ; \
${PICK} $PSRAM_IMAGE2_START $PSRAM_IMAGE2_END ${OUT_DIR}/psram_2.r.bin ${OUT_DIR}/psram_2.bin raw ; \
${PICK} $PSRAM_IMAGE2_START $PSRAM_IMAGE2_END ${OUT_DIR}/psram_2.bin ${OUT_DIR}/psram_2.p.bin

cat ${OUT_DIR}/xip_image2.p.bin ${OUT_DIR}/ram_2.p.bin ${OUT_DIR}/psram_2.p.bin > ${OUT_DIR}/km4_image2_all.bin
${PAD} ${OUT_DIR}/km4_image2_all.bin 4096
cat ${BSP_DIR}/km0_image2_all.bin ${OUT_DIR}/km4_image2_all.bin > ${OUT_DIR}/km0_km4_image2.bin
rm -f ${OUT_DIR}/km0_image2_all.bin ${OUT_DIR}/km4_image2_all.bin ${OUT_DIR}/psram* ${OUT_DIR}/ram* ${OUT_DIR}/xip*

cp -f ${IMGTOOL_BIN} ${OUT_DIR}
cp ${BSP_DIR}/*.bin ${OUT_DIR}

echo ...${TXT_OK}

du -h "${OUT_DIR}/km0_km4_image2.bin"
