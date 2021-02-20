# We keep this here to allow caching the SDK download since it is rather large
FROM buildpack-deps:buster AS sdk-download
RUN curl -sL "https://github.com/ambiot/ambd_sdk/archive/master.zip" > sdk.zip
RUN mkdir -p /rtlsdk && cd /rtlsdk && unzip /sdk.zip

# Build image
FROM buildpack-deps:buster AS builder
COPY --from=sdk-download /rtlsdk/ambd_sdk-master/ /ambd_sdk/

RUN apt-get update && apt-get install -yy \
    libclang1 \
    clang \
    gcc-arm-none-eabi \
    gdb-multiarch \
    openocd

RUN useradd -m -d /home/build -s /bin/bash -G users build && mkdir -p /code && chown build:build /code
USER build
WORKDIR /home/build/

RUN curl https://sh.rustup.rs -sSf > rustup.sh && \
    sh rustup.sh -y --default-toolchain nightly -t thumbv8m.main-none-eabihf -c rust-src && \
    rm rustup.sh

ENV PATH=/home/build/.cargo/bin:$PATH

RUN cargo install bindgen && \
    mkdir -p /code && \
    rustup default nightly-x86_64-unknown-linux-gnu

WORKDIR /code
