#!/usr/bin/env bash

set -e

pip3 install websockets==10.2

cargo +esp espflash save-image app.bin --target riscv32imc-esp-espidf --release --features "esp32c3_ili9341 native"

find target -name bootloader.bin -exec cp {} . \;
find target -name partition-table.bin -exec cp {} . \;

cd ~/esp32-wokwi-gitpod-websocket-server/

# ESP32S2 board
#export WOKWI_PROJECT_ID="330831847505265234"
# ESP32C3
export WOKWI_PROJECT_ID="330910629554553426"

python3 server.py