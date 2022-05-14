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

cargo +esp espflash save-image app.bin --merge --target xtensa-esp32-espidf --features "esp32_ili9341"

# ESP32 board
export ESP_BOARD="esp32"
export WOKWI_PROJECT_ID="331440829570744915"
ESP_ARCH=""
if [ "${ESP_BOARD}" == "esp32c3" ]; then
    ESP_ARCH="riscv32imc-esp-espidf"
else
    ESP_ARCH="xtensa-esp32-espidf"
fi
export ESP_ARCH=${ESP_ARCH}

python3  ~/esp32-wokwi-gitpod-websocket-server/server.py