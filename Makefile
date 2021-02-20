# Download URLs
ARDUINO_SDK_DOWNLOAD = https://github.com/ambiot/ambd_arduino/raw/master/Arduino_package/release/ameba_d-3.0.7-v02.tar.gz
ARDUINO_TOOLS_DOWNLOAD = https://github.com/ambiot/ambd_arduino/raw/master/Arduino_package/release/ameba_d_tools_linux-1.0.5.tar.gz
ARDUINO_TOOLCHAIN_DOWNLOAD = https://github.com/ambiot/ambd_arduino/raw/master/Arduino_package/release/ameba_d_toolchain_linux_64-1.0.1.tar.bz2

# General
OS = $(shell uname)
BIN_NAME = rtl8721d
RUST_TRIPLE = thumbv8m.main-none-eabihf
UPLOAD_PORT = /dev/ttyACM1

# Ansi codes
ANSI_END          = \033[0m
ANSI_BLACK        = \033[0;30m
ANSI_DARK_GRAY    = \033[1;30m
ANSI_RED          = \033[0;31m
ANSI_LIGHT_RED    = \033[1;31m
ANSI_GREEN        = \033[0;32m
ANSI_LIGHT_GREEN  = \033[1;32m
ANSI_ORANGE       = \033[0;33m
ANSI_YELLOW       = \033[1;33m
ANSI_BLUE         = \033[0;34m
ANSI_LIGHT_BLUE   = \033[1;34m
ANSI_PURPLE       = \033[0;35m
ANSI_LIGHT_PURPLE = \033[1;35m
ANSI_CYAN         = \033[0;36m
ANSI_LIGHT_CYAN   = \033[1;36m
ANSI_LIGHT_GRAY   = \033[0;37m
ANSI_WHITE        = \033[1;37m

# Pass/Fail Text
TXT_OK   = " [$(ANSI_GREEN)OK$(ANSI_END)]"
TXT_FAIL = " [$(ANSI_RED)FAIL$(ANSI_END)]"

# Arduino tools
TOOL_ROOT = target/tools
TOOLCHAIN_ROOT = target/toolchain

# SDK
SDK_ROOT = target/sdk
SDK_PATH = $(SDK_ROOT)/hardware/system/libameba/sdk
SDK_A_PATH = $(SDK_ROOT)/hardware/variants/rtl8721d
SDK_A_FILES = $(wildcard $(SDK_A_PATH)/*.a)
SDK_LD_DIR = $(SDK_A_PATH)/linker_scripts/gcc
SDK_LD_FILES = $(wildcard $(SDK_LD_DIR)/*.ld)
PLATFORM_TXT = $(SDK_ROOT)/hardware/platform.txt
BOARDS_TXT = $(SDK_ROOT)/hardware/boards.txt

# Tooling
TOOL_PREFIX = $(TOOLCHAIN_ROOT)/bin/arm-none-eabi-
AMEBA_TOOLDIR = $(TOOL_ROOT)/tools/linux

AR = $(TOOL_PREFIX)ar
CC = $(TOOL_PREFIX)gcc
AS = $(TOOL_PREFIX)as
NM = $(TOOL_PREFIX)nm
LD = $(TOOL_PREFIX)gcc
OBJCOPY = $(TOOL_PREFIX)objcopy
OBJDUMP = $(TOOL_PREFIX)objdump
PICK = $(AMEBA_TOOLDIR)/pick
PAD = $(AMEBA_TOOLDIR)/pad
# CHKSUM = $(AMEBA_TOOLDIR)/checksum
GDB = gdb-multiarch
AMEBA_UPLOAD_TOOL = $(realpath $(AMEBA_TOOLDIR)/image_tool/amebad_image_tool)

# Files...
# These includes derived from the arduino platform.txt ...
INCLUDES =  -Ic/inc \
			-I$(SDK_PATH)/component/os/freertos \
			-I$(SDK_PATH)/component/os/freertos/freertos_v10.2.0/Source/include \
			-I$(SDK_PATH)/component/os/freertos/freertos_v10.2.0/Source/portable/GCC/ARM_CM33/non_secure \
			-I$(SDK_PATH)/component/os/os_dep/include \
			-I$(SDK_PATH)/component/soc/realtek/amebad/misc \
			-I$(SDK_PATH)/component/common/api/network/include \
			-I$(SDK_PATH)/component/common/api \
			-I$(SDK_PATH)/component/common/api/at_cmd \
			-I$(SDK_PATH)/component/common/api/platform \
			-I$(SDK_PATH)/component/common/api/wifi \
			-I$(SDK_PATH)/component/common/api/wifi/rtw_wpa_supplicant/src \
			-I$(SDK_PATH)/component/common/api/wifi/rtw_wpa_supplicant/src/crypto \
			-I$(SDK_PATH)/component/common/application \
			-I$(SDK_PATH)/component/common/media/framework \
			-I$(SDK_PATH)/component/common/example \
			-I$(SDK_PATH)/component/common/example/wlan_fast_connect \
			-I$(SDK_PATH)/component/common/mbed/api \
			-I$(SDK_PATH)/component/common/mbed/hal \
			-I$(SDK_PATH)/component/common/mbed/hal_ext \
			-I$(SDK_PATH)/component/common/mbed/targets/hal/rtl8721d \
			-I$(SDK_PATH)/component/common/network \
			-I$(SDK_PATH)/component/common/network/lwip/lwip_v2.0.2/port/realtek/freertos \
			-I$(SDK_PATH)/component/common/network/lwip/lwip_v2.0.2/src/include \
			-I$(SDK_PATH)/component/common/network/lwip/lwip_v2.0.2/src/include/lwip \
			-I$(SDK_PATH)/component/common/network/lwip/lwip_v2.0.2/port/realtek \
			-I$(SDK_PATH)/component/common/test \
			-I$(SDK_PATH)/component/soc/realtek/amebad/cmsis \
			-I$(SDK_PATH)/component/soc/realtek/amebad/fwlib \
			-I$(SDK_PATH)/component/soc/realtek/amebad/misc \
			-I$(SDK_PATH)/component/common/drivers/wlan/realtek/include \
			-I$(SDK_PATH)/component/common/drivers/wlan/realtek/src/osdep \
			-I$(SDK_PATH)/component/common/drivers/wlan/realtek/src/hci \
			-I$(SDK_PATH)/component/common/network/ssl/ssl_ram_map/rom \
			-I$(SDK_PATH)/component/common/utilities \
			-I$(SDK_PATH)/component/common/video/v4l2/inc \
			-I$(SDK_PATH)/component/common/media/rtp_codec \
			-I$(SDK_PATH)/component/common/file_system/fatfs \
			-I$(SDK_PATH)/component/common/file_system/fatfs/r0.10c/include \
			-I$(SDK_PATH)/component/common/file_system/ftl \
			-I$(SDK_PATH)/component/common/drivers/sdio/realtek/sdio_host/inc \
			-I$(SDK_PATH)/component/common/audio \
			-I$(SDK_PATH)/component/common/drivers/i2s \
			-I$(SDK_PATH)/component/common/application/xmodem \
			-I$(SDK_PATH)/component/common/network/mDNS \
			-I$(SDK_PATH)/component/soc/realtek/amebad/fwlib/include \
			-I$(SDK_PATH)/component/soc/realtek/amebad/swlib/string \
			-I$(SDK_PATH)/component/soc/realtek/amebad/app/monitor/include \
			-I$(SDK_PATH)/component/soc/realtek/amebad/app/xmodem \
			-I$(SDK_PATH)/component/common/network/ssl/mbedtls-2.4.0/include \
			-I$(SDK_PATH)/component/common/bluetooth/realtek/sdk/board/amebad/lib \
			-I$(SDK_PATH)/component/common/bluetooth/realtek/sdk/board/amebad/src \
			-I$(SDK_PATH)/component/common/bluetooth/realtek/sdk/board/amebad/src/vendor_cmd \
			-I$(SDK_PATH)/component/common/bluetooth/realtek/sdk/board/common/inc \
			-I$(SDK_PATH)/component/common/bluetooth/realtek/sdk/example/bt_config \
			-I$(SDK_PATH)/component/common/bluetooth/realtek/sdk/inc/app \
			-I$(SDK_PATH)/component/common/bluetooth/realtek/sdk/inc/bluetooth/gap \
			-I$(SDK_PATH)/component/common/bluetooth/realtek/sdk/inc/bluetooth/profile \
			-I$(SDK_PATH)/component/common/bluetooth/realtek/sdk/inc/bluetooth/profile/client \
			-I$(SDK_PATH)/component/common/bluetooth/realtek/sdk/inc/bluetooth/profile/server \
			-I$(SDK_PATH)/component/common/bluetooth/realtek/sdk/inc/os \
			-I$(SDK_PATH)/component/common/bluetooth/realtek/sdk/inc/platform \
			-I$(SDK_PATH)/component/common/bluetooth/realtek/sdk/inc/stack \
			-I$(SDK_PATH)/component/common/bluetooth/realtek/sdk/src/app/ble_central_client \
			-I$(SDK_PATH)/component/common/bluetooth/realtek/sdk/src/mcu/module/data_uart_cmd \
			-I$(SDK_PATH)/component/common/drivers/ir/protocol \
			-I$(SDK_PATH)/component/common/drivers/wlan/realtek/src/core/option \
			-I$(SDK_PATH)/component/common/drivers/wlan/realtek/src/hal \
			-I$(SDK_PATH)/component/common/drivers/wlan/realtek/src/hal/phydm \
			-I$(SDK_PATH)/component/common/network/ssl/mbedtls-2.4.0/include

SRCS = $(wildcard c/src/*.c)

# Output
BUILD_INFO_FILE = c/inc/build_info.h
TARGET = target/rtl8721d
OBJ_DIR = $(TARGET)/obj
OUT_DIR = $(TARGET)/bin
OBJS := $(addprefix $(OBJ_DIR)/,$(patsubst %.c,%.o,$(SRCS)))
DEPS := $(addprefix $(OBJ_DIR)/,$(patsubst %.c,%.d,$(SRCS)))
RUST_OUT_PATH = target/thumbv8m.main-none-eabihf/debug/
RUST_OBJ_NAME = rtl8721d-rust
RUST_OBJ = $(OBJ_DIR)/$(RUST_OBJ_NAME).a
BSP_PATH = $(TOOL_ROOT)/bsp/image/PMU_bins/NONE
BSP_FILENAMES = km0_boot_all.bin km4_boot_all.bin km0_image2_all.bin
BSP_SRC_FILES := $(addprefix $(BSP_PATH)/,$(BSP_FILENAMES))
BSP_TGT_FILES := $(addprefix $(OUT_DIR)/,$(BSP_FILENAMES))
IMGTOOL_BIN := $(TOOL_ROOT)/bsp/image/imgtool_flashloader_amebad.bin

# Compiler options
CFLAGS = $(shell grep -e '^compiler.c.flags' $(PLATFORM_TXT) | cut -d '=' -f 2-)
CFLAGS += $(shell grep -e '^compiler.c.elf.flags' $(PLATFORM_TXT) | cut -d '=' -f 2-)
CFLAGS += $(shell grep -e '^compiler.c.elf.extra_flags' $(PLATFORM_TXT) | cut -d '=' -f 2-)
CFLAGS += $(shell grep -e '^compiler.ameba.c.flags' $(PLATFORM_TXT) | cut -d '=' -f 2- | sed 's/{compiler.arduino.c.include}//g')
CFLAGS += $(shell grep -e '^ameba_rtl8721d.build.extra_flags' $(BOARDS_TXT) | cut -d '=' -f 2- | sed 's/{build.usb_flags}//g')

LFLAGS = -L$(TOOLCHAIN_ROOT)/lib/ -L$(SDK_LD_DIR) -T$(SDK_LD_FILES)
LFLAGS += -O2 -march=armv8-m.main+dsp -mthumb -mcmse -mfloat-abi=hard -mfpu=fpv5-sp-d16 -nostartfiles -specs nosys.specs -Wl,--gc-sections \
		  -Wl,-Map=$(OUT_DIR)/$(BIN_NAME).map -Wl,--cref -Wl,--build-id=none -Wl,-wrap,strcat -Wl,-wrap,strchr -Wl,-wrap,strcmp \
		  -Wl,-wrap,strncmp -Wl,-wrap,strcpy -Wl,-wrap,strncpy -Wl,-wrap,strlen -Wl,-wrap,strnlen -Wl,-wrap,strncat -Wl,-wrap,strpbrk \
		  -Wl,-wrap,strstr -Wl,-wrap,strtok -Wl,-wrap,strsep -Wl,-wrap,strtoll -Wl,-wrap,strtoul -Wl,-wrap,strtoull -Wl,-wrap,atoi -Wl,-wrap,malloc \
		  -Wl,-wrap,free -Wl,-wrap,realloc -Wl,-wrap,memcmp -Wl,-wrap,memcpy -Wl,-wrap,memmove -Wl,-wrap,memset -Wl,--no-enum-size-warning -Wl,--warn-common \
		  -Wl,--allow-multiple-definition

all: build $(BSP_TGT_FILES) prepare_image

OUT_FILE_PREFIX = $(OUT_DIR)/$(BIN_NAME)
NMAP_FILE = $(OUT_FILE_PREFIX).nmap
ASM_FILE = $(OUT_FILE_PREFIX).asm
AXF_FILE = $(OUT_FILE_PREFIX).axf

.PHONY: prepare_image
prepare_image: build $(BSP_TGT_FILES)
	@ echo -n generating $(NMAP_FILE)...
	@ $(NM) $(AXF_FILE) | sort > $(NMAP_FILE)
	@ echo $(TXT_OK)

	@ echo -n "generating $(ASM_FILE)..."
	@ $(OBJDUMP) -d $(AXF_FILE) > $(ASM_FILE)
	@ echo $(TXT_OK)

	@ echo  preparing output binaries...

	@ $(OBJCOPY) -j .ram_image2.entry -j .ram_image2.text -j .ram_image2.data -Obinary $(AXF_FILE) $(OUT_DIR)/ram_2.r.bin
	@ $(OBJCOPY) -j .xip_image2.text  -Obinary $(AXF_FILE) $(OUT_DIR)/xip_image2.bin
	@ $(OBJCOPY) -j .psram_image2.text -j .psram_image2.data -Obinary $(AXF_FILE) $(OUT_DIR)/psram_2.r.bin

	@ RAM_IMAGE2_START=$$(grep '__ram_image2_text_start__' $(NMAP_FILE) | awk '{print "0x"$$1}') ;  \
	RAM_IMAGE2_END=$$(grep '__ram_image2_text_end__' $(NMAP_FILE) | awk '{print "0x"$$1}') ;  \
	$(PICK) $$RAM_IMAGE2_START $$RAM_IMAGE2_END $(OUT_DIR)/ram_2.r.bin $(OUT_DIR)/ram_2.bin raw ; \
	$(PICK) $$RAM_IMAGE2_START $$RAM_IMAGE2_END $(OUT_DIR)/ram_2.bin $(OUT_DIR)/ram_2.p.bin

	@ FLASH_START=$$(grep '__flash_text_start__' $(NMAP_FILE) | awk '{print "0x"$$1}') ; \
	FLASH_END=$$(grep '__flash_text_end__' $(NMAP_FILE) | awk '{print "0x"$$1}') ; \
	$(PICK) $$FLASH_START $$FLASH_END $(OUT_DIR)/xip_image2.bin $(OUT_DIR)/xip_image2.p.bin

	@ PSRAM_IMAGE2_START=$$(grep '__psram_image2_text_start__' $(NMAP_FILE) | awk '{print "0x"$$1}') ; \
	PSRAM_IMAGE2_END=$$(grep '__psram_image2_text_end__' $(NMAP_FILE) | awk '{print "0x"$$1}') ; \
	$(PICK) $$PSRAM_IMAGE2_START $$PSRAM_IMAGE2_END $(OUT_DIR)/psram_2.r.bin $(OUT_DIR)/psram_2.bin raw ; \
	$(PICK) $$PSRAM_IMAGE2_START $$PSRAM_IMAGE2_END $(OUT_DIR)/psram_2.bin $(OUT_DIR)/psram_2.p.bin

	@ cat $(OUT_DIR)/xip_image2.p.bin $(OUT_DIR)/ram_2.p.bin $(OUT_DIR)/psram_2.p.bin > $(OUT_DIR)/km4_image2_all.bin
	@ $(PAD) $(OUT_DIR)/km4_image2_all.bin 4096
	@ cat $(OUT_DIR)/km0_image2_all.bin $(OUT_DIR)/km4_image2_all.bin > $(OUT_DIR)/km0_km4_image2.bin
	@ rm -f $(OUT_DIR)/km0_image2_all.bin $(OUT_DIR)/km4_image2_all.bin $(OUT_DIR)/psram* $(OUT_DIR)/ram* $(OUT_DIR)/xip*

	@ echo ...$(TXT_OK)

.PHONY: prep
prep:
	@echo -n "generating build information..."
	@mkdir -p $(OBJ_DIR)
	@mkdir -p $(OUT_DIR)
	@echo \#define UTS_VERSION \"`date +%Y/%m/%d-%T`\" > $(BUILD_INFO_FILE)
	@echo \#define RTL_FW_COMPILE_TIME \"`date +%Y/%m/%d-%T`\" >> $(BUILD_INFO_FILE)
	@echo \#define RTL_FW_COMPILE_DATE \"`date +%Y%m%d`\" >> $(BUILD_INFO_FILE)
	@echo \#define RTL_FW_COMPILE_BY \"`id -u -n`\" >> $(BUILD_INFO_FILE)
	@echo \#define RTL_FW_COMPILE_HOST \"`$(HOSTNAME_APP)`\" >> $(BUILD_INFO_FILE)
	@echo \#define RTL_FW_COMPILE_DOMAIN \"\" >> $(BUILD_INFO_FILE)
	@echo \#define RTL_FW_COMPILER \"gcc `$(CC) $(CFLAGS) -dumpversion | tr --delete '\r'`\" >> $(BUILD_INFO_FILE)
	@echo $(TXT_OK)

# Compile
$(OBJS): $(OBJ_DIR)/%.o : %.c
	@mkdir -p $(dir $@)
	@echo -n "building $@..."
	@$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@
	@echo $(TXT_OK)

$(RUST_OBJ):
	@echo -n "building $(RUST_OBJ)..."
	@cargo build 2> /dev/null
	@cp -f $(RUST_OUT_PATH)/librust8720.a $(RUST_OBJ)
	@cp -f $(RUST_OUT_PATH)/librust8720.d $(OBJ_DIR)/$(RUST_OBJ_NAME).d
	@echo $(TXT_OK)

$(BSP_TGT_FILES):
	@echo -n "copying $@..."
	@cp -f $(BSP_PATH)/$(notdir $@) $@
	@echo $(TXT_OK)

build: prep $(OBJS) $(RUST_OBJ)
	@echo -n "linking..."
	@$(LD) \
		$(LFLAGS) \
		-o $(OUT_DIR)/$(BIN_NAME).axf \
		-Wl,--start-group $(OBJS) $(RUST_OBJ) $(SDK_A_FILES) -Wl,--end-group \
		-lm
	@echo $(TXT_OK)

rebuild: clean build

.PHONY: upload
upload:
	cp -f $(IMGTOOL_BIN) $(OUT_DIR)
	cd $(OUT_DIR) && $(AMEBA_UPLOAD_TOOL) $(UPLOAD_PORT)

.PHONY: clean
clean:
	@echo -n "cleaning..."
	@rm -rf $(TARGET)
	@rm -f $(OBJS) $(RUST_OBJ) $(patsubst %.o,%.d,$(OBJS)) $(BUILD_INFO_FILE)
	@echo $(TXT_OK)

.PHONY: clean_all
clean_all: clean
	@cargo clean

-include $(DEPS)
