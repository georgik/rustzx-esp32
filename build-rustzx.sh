#!/bin/bash

# Gitpod tasks do not source the user environment
if [ "${USER}" == "gitpod" ]; then
    source /home/gitpod/export-rust.sh > /dev/null 2>&1
    export IDF_TOOLS_PATH=/home/gitpod/.espressif
    source /home/gitpod/.espressif/frameworks/esp-idf-v4.4/export.sh > /dev/null 2>&1
    export CURRENT_PROJECT=rustzx-esp32
fi

cargo +esp build --target xtensa-esp32s2-espidf --release --features "esp32s2_ili9341"
