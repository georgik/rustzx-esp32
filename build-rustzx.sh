#!/bin/bash

# Gitpod tasks do not source the user environment
if [ "${USER}" == "gitpod" ]; then
    source /home/gitpod/export-rust.sh > /dev/null 2>&1
    export IDF_TOOLS_PATH=/home/gitpod/.espressif
    source /home/gitpod/.espressif/frameworks/esp-idf-v4.4/export.sh > /dev/null 2>&1
    export CURRENT_PROJECT=rustzx-esp32
fi

cargo +esp build --target xtensa-esp32-espidf --release --features "esp32_ili9341"
#cargo +esp espflash --target xtensa-esp32s2-espidf --release --features "kaluga_ili9341" --monitor
#cargo +esp build --target xtensa-esp32s2-espidf --release --features "kaluga_ili9341"
#cargo +esp build --target xtensa-esp32s3-espidf --release --features "esp32s3_usb_otg"
#cargo +esp build --target riscv32imc-esp-espidf --release --features "esp32c3_ili9341"