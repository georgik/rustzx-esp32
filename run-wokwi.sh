#!/usr/bin/env bash

set -e

# Gitpod tasks do not source the user environment
if [ "${USER}" == "gitpod" ]; then
    source /home/gitpod/export-rust.sh > /dev/null 2>&1
    export IDF_TOOLS_PATH=/home/gitpod/.espressif
    source /home/gitpod/.espressif/frameworks/esp-idf-v4.4/export.sh > /dev/null 2>&1
    export CURRENT_PROJECT=rustzx-esp32
fi

pip3 install websockets==10.2

cargo +esp espflash save-image app.bin --target xtensa-esp32-espidf --features "esp32_ili9341"

find target -name bootloader.bin -exec cp {} . \;
find target -name partition-table.bin -exec cp {} . \;

# ESP32 board
export ESP_BOARD="esp32"
export WOKWI_PROJECT_ID="331440829570744915"
export ESP_BOOTLOADER_OFFSET="0x1000"
export ESP_PARTITION_TABLE_OFFSET="0x8000"
export ESP_APP_OFFSET="0x10000"

python3  ~/esp32-wokwi-gitpod-websocket-server/server.py