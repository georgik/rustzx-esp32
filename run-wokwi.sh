#!/usr/bin/env bash

set -e

pip3 install websockets==10.2

cargo +esp espflash save-image app.bin --target xtensa-esp32-espidf --features "esp32_ili9341"

find target -name bootloader.bin -exec cp {} . \;
find target -name partition-table.bin -exec cp {} . \;

# cd ~/esp32-wokwi-gitpod-websocket-server/

# ESP32 board
export ESP_BOARD="esp32"
export WOKWI_PROJECT_ID="331440829570744915"
export ESP_BOOTLOADER_OFFSET="0x1000"
export ESP_PARTITION_TABLE_OFFSET="0x8000"
export ESP_APP_OFFSET="0x10000"

# if [ "$HOME" == "/home/esp" ]; then
#     path="$HOME/workspace"
# else
#     path="/workspace/${CURRENT_PROJECT}"
# fi
# cd $path
# sleep 2
python3  ~/esp32-wokwi-gitpod-websocket-server/server.py

