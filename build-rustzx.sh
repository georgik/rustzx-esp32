#!/bin/bash

# Gitpod tasks do not source the user environment
if [ "${USER}" == "gitpod" ]; then
    which idf.py >/dev/null || {
        source /home/gitpod/export-rust.sh > /dev/null 2>&1
        export IDF_TOOLS_PATH=/home/gitpod/.espressif
        source /home/gitpod/.espressif/frameworks/esp-idf-release-v4.4/export.sh
    }
fi

cargo build --target xtensa-esp32c3-espidf --release --features "esp32c3_ili9341"
