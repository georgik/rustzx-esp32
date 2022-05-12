# RustZX for ESP32 - experimental version

Goal of the project: Run ZX Spectrum on ESP32

HW: ESP32 with ILI9341

## Build using GitPod

[![Open ESP32 in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/github.com/georgik/rustzx-esp32/)

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
./build-rustzx.sh
```

Run in with Wokwi simulator:

```
./run-wokwi.sh
```

## Build and flash

Build for ESP32 Using cargo-espflash for ESP32-S3 USB OTG:

```
cargo +esp-1.60.0.1 espflash --target xtensa-esp32-espidf --release --features "esp32c3_ili9341"
```


Build for ESP32-S3 USB OTG sing cargo-espflash:

```
cargo +esp-1.60.0.1 espflash --target xtensa-esp32-espidf --release --features "esp32s3_usb_otg"
```

With PowerShell:

```
.\Build-RustZX.ps1 -Target xtensa-esp32s2-espidf -Board kaluga_ili9341 -Port COM23
```


## HW Setup

### Display connection

| ESP32-DevKitS-V1.1 | ILI9341       |
|--------------------|---------------|
| GND                | GND           |
| 3.3V               | 3.3V          |
| RST                | GPIO4         |
| CLK                | GPIO18        |
| D_C                | GPIO2         |
| CS                 | GPIO15        |
| MOSI               | GPIO23        |
| MISO               | not connected |

## References

- Rust code for ESP32 based on https://github.com/ivmarkov/rust-esp32-std-demo
- RustZX wrapper code reused from https://github.com/pacmancoder/rustzx
