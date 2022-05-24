#!/usr/bin/env bash

set -e

if [ "${USER}" == "gitpod" ]; then
    export CURRENT_PROJECT=/workspace/rustzx-esp32
elif [ "${CODESPACE_NAME}" != "" ]; then
    export CURRENT_PROJECT=/workspaces/rustzx-esp32
else
    export CURRENT_PROJECT=~/workspace
fi

BUILD_MODE=""
case "$1" in
    ""|"release")
        bash build-rustzx.sh
        BUILD_MODE="release"
        ;;
    "debug")
        bash build-rustzx.sh debug
        BUILD_MODE="debug"
        ;;
    *)
        echo "Wrong argument. Only \"debug\"/\"release\" arguments are supported"
        exit 1;;
esac


export ESP_ELF="rustzx-esp32"
export ESP_BOARD="esp32"
if [ "${ESP_BOARD}" == "esp32c3" ]; then
    export ESP_ARCH="riscv32imc-esp-espidf"
elif [ "${ESP_BOARD}" == "esp32s2" ]; then
    export ESP_ARCH="xtensa-esp32s2-espidf"
else
    export ESP_ARCH="xtensa-esp32-espidf"
fi

web-flash --chip ${ESP_BOARD} ${CURRENT_PROJECT}/target/${ESP_ARCH}/${BUILD_MODE}/${ESP_ELF}
