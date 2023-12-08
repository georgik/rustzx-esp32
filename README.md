# RustZX for ESP32

Rust Bare Metal implementation of ZX Spectrum for ESP32.
The project is still work in progress.

Hardware (working):
- [M5Stack CoreS3](https://shop.m5stack.com/products/m5stack-cores3-esp32s3-lotdevelopment-kit)
- USB-to-Serial converter serving as keyboard input

Hardware (work-in-progress):
- [ESP32 C3 DevKit RUST](https://github.com/esp-rs/esp-rust-board) - app is not yet optimized for low memory

### Assembly

- connect USB-to-Serial converter and M5Stack CoreS3, this will serve as keyboard input
```
TX - GPIO17
RX - GPIO18
GND - GND
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
- download a `.tap` file from Speccy archives and store it to `test.tap`
```
cd m5stack-cores3
cargo build --release
```

## Run

Flash and monitor the application:
```
cargo run --release
```

Connect to serial console for keyboard simulation:
```
screen /dev/tty.usbserial-.... 115200
```

Screen command won't echo your input. Hit enter to load the tape included in the memory.

## References

- RustZX wrapper code reused from https://github.com/pacmancoder/rustzx
- older RustZX-ESP32 based with std on [v1.0.0-archive](https://github.com/georgik/rustzx-esp32/tree/v1.0.0-archive)
