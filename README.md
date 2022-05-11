# RustZX for ESP32 - experimental version

Goal of the project: Run ZX Spectrum on ESP32

HW: ESP32 OTG USB with ST7789 display


## References

- Rust code for ESP32 based on https://github.com/ivmarkov/rust-esp32-std-demo
- RustZX wrapper code reused from https://github.com/pacmancoder/rustzx

## Build using GitPod

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/github.com/georgik/rustzx-esp32/tree/target/esp32c3)

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

Build for ESP32-C3 Using cargo-espflash for ESP32-S3 USB OTG:

```
cargo +esp-1.60.0.1 espflash --target xtensa-esp32s3-espidf --release --features "esp32c3_ili9341"
```


Build for ESP32-S3 USB OTG sing cargo-espflash:

```
cargo +esp-1.60.0.1 espflash --target xtensa-esp32s3-espidf --release --features "esp32s3_usb_otg"
```

With PowerShell:

```
.\Build-RustZX.ps1 -Target xtensa-esp32s2-espidf -Board kaluga_ili9341 -Port COM23
```
