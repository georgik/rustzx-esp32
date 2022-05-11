# RustZX for ESP32-C3

Goal of the project: Run ZX Spectrum on ESP32

Hardware: ESP32-C3 and ILI9341 display

## Build using GitPod

[![Open ESP32-C3 in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/github.com/georgik/rustzx-esp32/tree/target/esp32c3)

```
cargo build --release
```

### Other targets

- [![Open ESP32 in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/github.com/georgik/rustzx-esp32/) - ESP32 (Xtensa) - branch: [main](https://github.com/georgik/rustzx-esp32/)
- [![Open ESP32-S2 in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/github.com/georgik/rustzx-esp32/tree/target/esp32s2) - ESP32-S2 (Xtensa) - branch: [target/esp32s2](https://github.com/georgik/rustzx-esp32/tree/target/esp32s2)
- [![Open ESP32-S2 in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/github.com/georgik/rustzx-esp32/tree/target/esp32s3) - ESP32-S3 (Xtensa) - branch: [target/esp32s3](https://github.com/georgik/rustzx-esp32/tree/target/esp32s3)
- [![Open ESP32-S2 in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/github.com/georgik/rustzx-esp32/tree/target/esp32s3) - ESP32-C3 (RISC-V) - branch: [target/esp32c3](https://github.com/georgik/rustzx-esp32/tree/target/esp32c3)

## Build

Open in VS Code with Dev Container support.

Run:

```
cargo build --release
```

Run in with Wokwi simulator:

```
./run-wokwi.sh
```

## Build on local machine

Install rust nightly toolchain with LLVM and use export-rust to configure `LIBCLANG_PATH` using `export-rust.sh`:

```
curl -L -O https://raw.githubusercontent.com/esp-rs/rust-build/feature/small-llvm/install-rust-toolchain.sh
chmod a+x install-rust-toolchain.sh
./install-rust-toolcha.sh \
    --extra-crates "cargo-espflash ldproxy espmonitor" \
    --build-target "esp32c3" \
    --export-file export-rust.sh
source export-rust.sh
```

Build and flash

```
cargo espflash
```

## References

- Rust code for ESP32 based on https://github.com/ivmarkov/rust-esp32-std-demo
- RustZX wrapper code reused from https://github.com/pacmancoder/rustzx
