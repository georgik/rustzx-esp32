#!/bin/bash

# Gitpod and VsCode Codespaces tasks do not source the user environment
if [ "${USER}" == "gitpod" || "${CODESPACE_NAME}" != ""]; then
    export CURRENT_PROJECT=/workspace/rustzx-esp32
    which idf.py >/dev/null || {
        source ~/export-rust.sh > /dev/null 2>&1
    }
else
    export CURRENT_PROJECT=~/workspace
fi

cargo +esp build --target xtensa-esp32-espidf --release --features "esp32_ili9341"
