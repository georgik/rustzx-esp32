#!/usr/bin/env bash

set -e

pip3 install websockets==10.2

cargo +esp espflash save-image app.bin --target riscv32imc-esp-espidf --release --features "esp32c3_ili9341"

find target -name bootloader.bin -exec cp {} . \;
find target -name partition-table.bin -exec cp {} . \;

cd ~/esp32-wokwi-gitpod-websocket-server/

# ESP32-C3 - https://wokwi.com/projects/330910629554553426
export WOKWI_PROJECT_ID="330910629554553426"
export ESP_BOOTLOADER_OFFSET="0x0000"
export ESP_PARTITION_TABLE_OFFSET="0x8000"
export ESP_APP_OFFSET="0x10000"

python3 server.py