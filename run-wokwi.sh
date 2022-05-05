#!/usr/bin/env bash

set -e

pip3 install websockets==10.2

cargo +esp espflash save-image app.bin --target xtensa-esp32s2-espidf --release --features "kaluga_ili9341 native"

find target -name bootloader.bin -exec cp {} . \;
find target -name partition-table.bin -exec cp {} . \;

cd ~/esp32-wokwi-gitpod-websocket-server/
export WOKWI_PROJECT_ID="330831847505265234"
python3 server.py

