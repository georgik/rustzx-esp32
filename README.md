# RustZX for ESP32-C3

Goal of the project: Run ZX Spectrum on ESP32

Hardware: ESP32-C3 and ILI9341 display

![RustZX-ESP32](docs/rustzx-esp32-ili9341.png)

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

## HW Setup

### Display connection

| ILI9341  | ESP32-C3-DevKit-RUST-1    | Cable color |
|----------|---------------------------|-------------|
| GND      | GND                       | black       |
| 3.3V     | 3.3V                      | red         |
| RST      | GPIO10                    | orange      |
| CLK      | GPIO6                     | yellow      |
| D_C      | GPIO3                     | green       |
| CS       | GPIO2                     | blue        |
| MOSI     | GPIO7                     | purple      |
| MISO     | not connected             | -           |


Wokwi related project: https://wokwi.com/projects/330910629554553426 - [diagram.json](docs/diagram.json)

## References

- Rust code for ESP32 based on https://github.com/ivmarkov/rust-esp32-std-demo
- RustZX wrapper code reused from https://github.com/pacmancoder/rustzx
