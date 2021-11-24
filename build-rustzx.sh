#!/bin/bash

# Build and flash using cargo-espflash
cargo +esp-1.56.0.1 espflash /dev/tty.usbserial-13201 --target xtensa-esp32s2-espidf --release --features "kaluga_ili9341 native"

