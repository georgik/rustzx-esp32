# RustZX for ESP32 - experimental version

Goal of the project: Run ZX Spectrum on ESP32

HW: ESP32 OTG USB with ST7789 display


## References

- Rust code for ESP32 based on https://github.com/ivmarkov/rust-esp32-std-demo
- RustZX wrapper code reused from https://github.com/pacmancoder/rustzx

## Build

Open in VS Code with Dev Container support.

Run:

```
./build-rustzx.sh
```

## Build and flash

Using cargo-espflash:

```
cargo +esp-1.56.0.1 espflash /dev/tty.usbserial-110 --target xtensa-esp32s2-espidf --release --features "esp32s2_usb_otg native"
```

With PowerShell:

```
.\Build-RustZX.ps1 -Target xtensa-esp32s2-espidf -Board kaluga_ili9341 -Port COM23
```
