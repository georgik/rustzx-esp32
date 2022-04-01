#!/bin/bash

#PORT=`ls /dev/tty.usbserial-*01 | head -n 1`
# Build and flash using cargo-espflash
#echo "Using port: ${PORT}"
#cargo +esp espflash --target xtensa-esp32s2-espidf --release --features "kaluga_ili9341 native" ${PORT} --monitor
#cargo +esp build --target xtensa-esp32s2-espidf --release --features "kaluga_ili9341 native"
cargo +esp build --target xtensa-esp32s3-espidf --release --features "esp32s3_usb_otg native"

