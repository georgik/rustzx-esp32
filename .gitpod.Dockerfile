# Note: gitpod/workspace-base image references older version of CMake, it's necessary to install newer one
FROM gitpod/workspace-base
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8

# Set users
ARG CONTAINER_USER=gitpod
ARG CONTAINER_GROUP=gitpod
ARG TOOLCHAIN_VERSION=1.60.0.1

# Install dependencies
RUN sudo install-packages git curl gcc ninja-build libudev-dev \
  python3 python3-pip libusb-1.0-0 libssl-dev pkg-config libtinfo5 clang

USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}

# Install toolchain with extra crates
ARG INSTALL_RUST_TOOLCHAIN=install-rust-toolchain.sh
ENV PATH=${PATH}:/home/${CONTAINER_USER}/.cargo/bin:/home/${CONTAINER_USER}/opt/bin

# Use LLVM installer
# Official: https://github.com/esp-rs/rust-build/releases/download/v${TOOLCHAIN_VERSION}/${INSTALL_RUST_TOOLCHAIN}
ADD --chown=${CONTAINER_USER}:${CONTAINER_GROUP} \
  https://raw.githubusercontent.com/esp-rs/rust-build/feature/small-llvm/install-rust-toolchain.sh \
  /home/${CONTAINER_USER}/${INSTALL_RUST_TOOLCHAIN}

RUN chmod a+x ${INSTALL_RUST_TOOLCHAIN} \
  && ./${INSTALL_RUST_TOOLCHAIN} \
    --extra-crates "cargo-espflash ldproxy espmonitor" \
    --clear-cache "YES" \
    --build-target "esp32c3" \
    --export-file /home/${CONTAINER_USER}/export-rust.sh \
  && mkdir -p .espressif/frameworks/ \
  && git clone --branch "release/v4.4" -q --depth 1 --shallow-submodules \
    --recursive https://github.com/espressif/esp-idf.git \
    .espressif/frameworks/esp-idf-v4.4 \
  && python3 .espressif/frameworks/esp-idf-v4.4/tools/idf_tools.py install cmake ninja \
  && .espressif/frameworks/esp-idf-v4.4/install.sh esp32c3 \
  && rm -rf .espressif/dist \
  && rm -rf .espressif/frameworks/esp-idf-v4.4/docs \
  && rm -rf .espressif/frameworks/esp-idf-v4.4/examples \
  && rm -rf .espressif/frameworks/esp-idf-v4.4/tools/esp_app_trace \
  && rm -rf .espressif/frameworks/esp-idf-v4.4/tools/test_idf_size \
  && git clone https://github.com/georgik/esp32-wokwi-gitpod-websocket-server.git
