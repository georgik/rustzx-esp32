#!/usr/bin/env bash

set -e

cargo +esp espflash save-image app.bin --target xtensa-esp32s2-espidf --release --features "esp32s2_usb_otg native"
python3 ~/esp32-wokwi-gitpod-websocket-server/server.py
