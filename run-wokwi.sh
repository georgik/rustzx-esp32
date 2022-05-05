#!/usr/bin/env bash

set -e

cargo +esp espflash save-image app.bin --target xtensa-esp32s2-espidf --release --features "esp32s2_usb_otg native"

find target -name bootloader.bin -exec cp {} . \;
find target -name partition-table.bin -exec cp {} . \;

cd ~/esp32-wokwi-gitpod-websocket-server/
python3 server.py


