# Note: gitpod/workspace-base image references older version of CMake, it's necessary to install newer one
FROM  gitpod/workspace-base
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8

# Set users
ARG CONTAINER_USER=gitpod
ARG CONTAINER_GROUP=gitpod
ARG TOOLCHAIN_VERSION=1.60.0.1

# Install dependencies
RUN sudo install-packages git curl gcc ninja-build libudev-dev \
  python3 python3-pip libusb-1.0-0 libssl-dev pkg-config libtinfo5 clang \
  && pip3 install websockets==10.2

USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}

# Install toolchain with extra crates
ARG INSTALL_RUST_TOOLCHAIN=install-rust-toolchain.sh
ENV PATH=${PATH}:/home/${CONTAINER_USER}/.cargo/bin:/home/${CONTAINER_USER}/.cargo/bin:/home/${CONTAINER_USER}/opt/bin
ADD --chown=${CONTAINER_USER}:${CONTAINER_GROUP} \
  https://github.com/esp-rs/rust-build/releases/download/v${TOOLCHAIN_VERSION}/${INSTALL_RUST_TOOLCHAIN} \
  /home/${CONTAINER_USER}/${INSTALL_RUST_TOOLCHAIN}

# Add newer version of CMake
ADD --chown=${CONTAINER_USER}:${CONTAINER_GROUP} \
  https://github.com/Kitware/CMake/releases/download/v3.23.1/cmake-3.23.1-linux-x86_64.sh \
  /home/${CONTAINER_USER}/cmake-install.sh
RUN chmod a+x /home/gitpod/cmake-install.sh \
  && mkdir -p /home/gitpod/opt \
  && ./cmake-install.sh --prefix=/home/gitpod/opt --skip-license

RUN chmod a+x ${INSTALL_RUST_TOOLCHAIN} \
  && ./${INSTALL_RUST_TOOLCHAIN} \
    --extra-crates "ldproxy cargo-espflash espmonitor bindgen" \
    --clear-cache "YES" --export-file /home/gitpod/export-rust.sh \
  && mkdir -p .espressif/frameworks/ \
  && git clone --branch "release/v4.4" -q --depth 1 --shallow-submodules \
    --recursive https://github.com/espressif/esp-idf.git \
    .espressif/frameworks/esp-idf-v4.4 \
  && .espressif/frameworks/esp-idf-v4.4/install.sh esp32s2 \
  && .espressif/frameworks/esp-idf-v4.4/install.sh esp32s3 \
  && rm -rf .espressif/dist \
  && rm -rf .espressif/frameworks/esp-idf-v4.4/docs
