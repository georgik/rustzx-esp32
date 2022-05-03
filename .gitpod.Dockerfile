
FROM gitpod/workspace-full
ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8

# Set users
ARG CONTAINER_USER=gitpod
ARG CONTAINER_GROUP=gitpod
ARG TOOLCHAIN_VERSION=1.60.0.1
ARG NIGHTLY_VERSION=nightly

# Install dependencies
RUN sudo apt-get update \
  && sudo apt-get install -y vim nano git curl gcc ninja-build cmake libudev-dev \
  python3 python3-pip libusb-1.0-0 libssl-dev pkg-config libtinfo5 clang \
  && sudo apt-get clean -y && sudo rm -rf /var/lib/apt/lists/* /tmp/library-scripts \
  && sudo pip3 install websockets==10.2 \

USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}

# Install toolchain with extra crates
ARG INSTALL_RUST_TOOLCHAIN=install-rust-toolchain.sh
ENV PATH=${PATH}:/home/${CONTAINER_USER}/.cargo/bin:/home/${CONTAINER_USER}/.cargo/bin
ADD --chown=${CONTAINER_USER}:${CONTAINER_GROUP} \
  https://github.com/esp-rs/rust-build/releases/download/v${TOOLCHAIN_VERSION}/${INSTALL_RUST_TOOLCHAIN} \
  /home/${CONTAINER_USER}/${INSTALL_RUST_TOOLCHAIN}
RUN chmod a+x ${INSTALL_RUST_TOOLCHAIN} \
  && ./${INSTALL_RUST_TOOLCHAIN} \
    --extra-crates "ldproxy cargo-generate cargo-espflash espmonitor bindgen" \
    --clear-cache "YES" --export-file /home/${CONTAINER_USER}/export-rust.sh \
  && rustup component add rust-src --toolchain ${NIGHTLY_VERSION} \
  && rustup target add riscv32i-unknown-none-elf \
  && mkdir -p .espressif/frameworks/ \
  && git clone --branch "release/v4.4" -q --depth 1 --shallow-submodules \
    --recursive https://github.com/espressif/esp-idf.git \
    .espressif/frameworks/esp-idf-v4.4 \
  && .espressif/frameworks/esp-idf-v4.4/install.sh esp32s2 esp32s3 \
  && rm -rf .espressif/dist

CMD [ "/bin/bash", \
  "-c", "source /home/gitpod/export-rust.sh; export IDF_TOOLS_PATH=/home/gitpod/.espressif; source /home/gitpod/.espressif/frameworks/esp-idf-v4.4/export.sh; /bin/bash"]
