#!/bin/bash

#cargo +esp espflash --target xtensa-esp32s2-espidf --release --features "kaluga_ili9341" --monitor
#cargo +esp build --target xtensa-esp32s2-espidf --release --features "kaluga_ili9341"
#cargo +esp build --target xtensa-esp32s3-espidf --release --features "esp32s3_usb_otg"
cargo +esp build --target xtensa-esp32-espidf --release --features "esp32_ili9341"
#cargo +esp build --target riscv32imc-esp-espidf --release --features "esp32c3_ili9341"

