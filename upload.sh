#!/usr/bin/env bash

set -e

# Gitpod and VsCode Codespaces tasks do not source the user environment
if [ "${USER}" == "gitpod" || "${CODESPACE_NAME}" != ""]; then
    export CURRENT_PROJECT=/workspace/rustzx-esp32
    which idf.py >/dev/null || {
        source ~/export-rust.sh > /dev/null 2>&1
    }
else
    export CURRENT_PROJECT=~/workspace
fi

export ESP_ELF="rustzx-esp32"

export ESP_BOARD="esp32"
if [ "${ESP_BOARD}" == "esp32c3" ]; then
    export ESP_ARCH="riscv32imc-esp-espidf"
elif [ "${ESP_BOARD}" == "esp32s2" ]; then
    export ESP_ARCH="xtensa-esp32s2-espidf"
else
    export ESP_ARCH="xtensa-esp32-espidf"
fi

sh ./build-rustzx.sh
web-flash --chip ${ESP_BOARD} ${CURRENT_PROJECT}/target/${ESP_ARCH}/release/${ESP_ELF}