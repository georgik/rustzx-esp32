# RustZX for ESP32

Rust Bare Metal implementation of ZX Spectrum for ESP32.
The project is still work in progress.

Hardware (working):
- ZX Spectrum with USB keyboard over ESP-NOW (wireless)
  - [ESP32-S3-BOX](https://github.com/espressif/esp-box) as main emulator unit with display
  - [ESP32-S3-USB-OTG](https://github.com/espressif/esp-bsp/tree/master/bsp/esp32_s3_usb_otg) as USB keyboard to ESP-NOW converter (wireless) (ESP-IDF)
- ZX Spectrum PS/2 keyboard over UART (wired)
  - [M5Stack CoreS3](https://shop.m5stack.com/products/m5stack-cores3-esp32s3-lotdevelopment-kit) as main emulator unit with display
  - [ESP32-C3 DevKit](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/hw-reference/esp32c3/user-guide-devkitm-1.html) as PS/2 or USB COMBO keyboard converter to serial

Hardware (work-in-progress):
- [ESP32 C3 DevKit RUST](https://github.com/esp-rs/esp-rust-board) - app is not yet optimized for low memory

## ZX Spectrum with USB keyboard over ESP-NOW (wireless)

### Assembly

#### Assembly of the keyboard

- plug USB keyboard to ESP32-S3-USB-OTG USB HOST connector
- plug USB power suply to ESP32-S3-USB-OTG USB DEV connector
- plug mini-USB connector to port for flashing

#### Flashing keyboard

- use ESP-IDF 5.2
```
cd esp32-s3-usb-otg-keyboard
idf.py build flash monitor
```
- code is based on [ESP-IDF USB HID example](https://github.com/espressif/esp-idf/tree/master/examples/peripherals/usb/host/hid)

### Assembly of the main part

- connect ESP32-S3-BOX with USB-C to computer and flash the application

## Software setup

- use [espup](https://github.com/esp-rs/espup) to install Rust toolchain for Xtensa (ESP32-S3)
```
espup install
```
- use [espflash](https://github.com/esp-rs/espflash) to flash binaries
```
cargo install espflash
```
- download a `.tap` file from Speccy archives and store it to `data/hello.tap`

## Run

Flash and monitor the application:
```
cd esp32-s3-box
cargo run --release
```

## ZX Spectrum PS/2 keyboard over UART (wired)

### Assembly

#### Assmebly of the keyboard

- flash ESP32-C3 which should serve as keyboard converter from [PS/2 or USB to serial](https://georgik.rocks/how-to-connect-usb-and-ps-2-keyboards-to-esp32-with-rust/)
```
git clone https://github.com/georgik/ps2keyboard-esp32c3.git --branch feature/serial-converter
cd ps2keyboard-esp32c3
cargo run --release
```
- take PS/2 keyboard and wire it to ESP32-C3 according to [PS/2 ESP32-C3 circuit](https://github.com/georgik/ps2keyboard-esp32c3/tree/feature/serial-converter?tab=readme-ov-file#circuit)
- in case of USB keyboard you can skip PS/2 connector and wire[PS/2 ESP32-C3 circuit](https://github.com/georgik/ps2keyboard-esp32c3/tree/feature/serial-converter?tab=readme-ov-file#circuit) to USB connector using the schematics from [USB to PS2 convertor](https://www.instructables.com/USB-to-PS2-convertor/)
- recommendation: use [wire wrapping](https://youtu.be/L-463vchW0o?si=MtQrXpbTJznikXSJ) to connect parts

### Assembly of the main part

- connect ESP32-C3 keyboard converter and M5Stack CoreS3
```
GPIO4 RX (ESP32-C3 KB) - GPIO17 TX or T at Grove Port C (M5Stack CoreS3)
GPIO5 TX (ESP32-C3 KB) - GPIO18 RX or R at Grove Port C (M5Stack CoreS3)
GND (ESP32-C3 KB) - GND (M5Stack CoreS3)
```

## Software setup

- use [espup](https://github.com/esp-rs/espup) to install Rust toolchain for Xtensa (ESP32-S3)
```
espup install
```
- use [espflash](https://github.com/esp-rs/espflash) to flash binaries
```
cargo install espflash
```
- download a `.tap` file from Speccy archives and store it to `data/hello.tap`

## Run

Flash and monitor the application:
```
cd m5stack-cores3
cargo run --release
```

Hit enter to load the tape included in the memory.

## References

- RustZX wrapper code reused from https://github.com/pacmancoder/rustzx
- [ESP-IDF USB HID example](https://github.com/espressif/esp-idf/tree/master/examples/peripherals/usb/host/hid)
- [Rust ESP-NOW](https://github.com/esp-rs/esp-wifi)
- older RustZX-ESP32 based with std on [v1.0.0-archive](https://github.com/georgik/rustzx-esp32/tree/v1.0.0-archive)
