#!/usr/bin/env bash

set -e

pip3 install websockets==10.2

export ESP_ARCH="xtensa-esp32-espidf"
cargo +esp espflash save-image app.bin --target "${ESP_ARCH}" --features "esp32_ili9341"

find target/${ESP_ARCH}/debug -name bootloader.bin -exec cp {} . \;
find target/${ESP_ARCH}/debug -name partition-table.bin -exec cp {} . \;

cd ~/esp32-wokwi-gitpod-websocket-server/

# ESP32 board
export WOKWI_PROJECT_ID="331440829570744915"
export ESP_BOOTLOADER_OFFSET="0x1000"
export ESP_PARTITION_TABLE_OFFSET="0x8000"
export ESP_APP_OFFSET="0x10000"
export ELF_FILE="rustzx-esp32"

python3 server.py
