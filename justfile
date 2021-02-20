ARDUINO_SDK_DOWNLOAD := "https://github.com/ambiot/ambd_arduino/raw/master/Arduino_package/release/ameba_d-3.0.7-v02.tar.gz"
ARDUINO_TOOLS_DOWNLOAD := "https://github.com/ambiot/ambd_arduino/raw/master/Arduino_package/release/ameba_d_tools_linux-1.0.5.tar.gz"
ARDUINO_TOOLCHAIN_DOWNLOAD := "https://github.com/ambiot/ambd_arduino/raw/master/Arduino_package/release/ameba_d_toolchain_linux_64-1.0.1.tar.bz2"
COMPLETE_SDK_DOWNLOAD := "https://github.com/ambiot/ambd_sdk/archive/master.zip"
TOOL_DIR := "tools"
SDK_DIR := "tools/sdk/"
TOOLCHAIN_DIR := "tools/toolchain/"
TOOL_BIN_DIR := "tools/bin/"
COMPLETE_SDK_DIR := "tools/complete-sdk/"
TXT_OK := " [OK]"

@bindings:
    util/bindgen.sh
    echo -n "{{TXT_OK}}"

build:
    make clean
    cargo build
    make all

upload: build
    make upload

upload-screen: upload
    screen /dev/ttyACM1 115200

rustup:
    rustup toolchain add nightly
    rustup target add x86_64-unknown-linux-gnu
    rustup target add thumbv8m.main-none-eabihf
    rustup default nightly-x86_64-unknown-linux-gnu

pack-libs:
    ar -rcT target/lib-rtl8721d.a \
        target/sdk/hardware/variants/rtl8721d/*.a \
        target/rtl8721d/obj/c/src/main.o \
        target/toolchain/arm-none-eabi/lib/v8-m.main/fpu/fpv5-sp-d16/*.a \
        target/toolchain/arm-none-eabi/lib/v8-m.main/fpu/fpv5-sp-d16/*.o

@download-sdk:
	mkdir -p {{SDK_DIR}} {{TOOLCHAIN_DIR}} {{TOOL_BIN_DIR}}

	echo -n downloading toolchain...
	curl -sL "{{ARDUINO_TOOLCHAIN_DOWNLOAD}}" > "{{TOOL_DIR}}/toolchain.tar.bz2"
	echo "{{TXT_OK}}"
	echo -n extracting toolchain to {{TOOLCHAIN_DIR}}...
	tar -xjf "{{TOOL_DIR}}/toolchain.tar.bz2" -C "{{TOOLCHAIN_DIR}}" --strip-components=1
	echo "{{TXT_OK}}"

	echo -n downloading tools...
	curl -sL "{{ARDUINO_TOOLS_DOWNLOAD}}" > "{{TOOL_DIR}}/tools.tar.gz"
	echo "{{TXT_OK}}"
	echo -n extracting tools to {{TOOL_BIN_DIR}}...
	tar -xzf {{TOOL_DIR}}/tools.tar.gz -C "{{TOOL_BIN_DIR}}" --strip-components=1
	echo "{{TXT_OK}}"

	echo -n downloading sdk...
	curl -sL "{{ARDUINO_SDK_DOWNLOAD}}" > "{{TOOL_DIR}}/ambd_sdk.tar.gz"
	echo "{{TXT_OK}}"
	echo -n extracting sdk to {{SDK_DIR}}...
	tar -xzf {{TOOL_DIR}}/ambd_sdk.tar.gz -C "{{SDK_DIR}}"
	echo "{{TXT_OK}}"

@download-complete-sdk:
    mkdir -p {{COMPLETE_SDK_DIR}}
    echo "downloading complete sdk (this may take awhile)..."
    curl -L "{{COMPLETE_SDK_DOWNLOAD}}" > "{{TOOL_DIR}}/sdk-complete.zip"
    echo "{{TXT_OK}}"
    echo -n extracting sdk to {{COMPLETE_SDK_DIR}}...
    cd "{{COMPLETE_SDK_DIR}}" && unzip ../sdk-complete.zip
    echo "{{TXT_OK}}"
