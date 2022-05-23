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


if [ "${USER}" == "gitpod" ];then
    gp_url=$(gp url 9012)
    echo "gp_url=${gp_url}"
    export WOKWI_HOST=${gp_url:8}
elif [ "${CODESPACE_NAME}" != "" ];then
    export WOKWI_HOST=${CODESPACE_NAME}-9012.githubpreview.dev
fi

export ESP_BOARD="esp32"
export ESP_ELF="rustzx-esp32"
export WOKWI_PROJECT_ID="331440829570744915"
if [ "${ESP_BOARD}" == "esp32c3" ]; then
    export ESP_ARCH="riscv32imc-esp-espidf"
    export WOKWI_PROJECT_ID="330910629554553426"
elif [ "${ESP_BOARD}" == "esp32s2" ]; then
    export WOKWI_PROJECT_ID="330831847505265234"
    export ESP_ARCH="xtensa-esp32s2-espidf"
else
    export WOKWI_PROJECT_ID="331440829570744915"
    export ESP_ARCH="xtensa-esp32-espidf"
fi

wokwi-server --chip ${ESP_BOARD} --id ${WOKWI_PROJECT_ID} ${CURRENT_PROJECT}/target/${ESP_ARCH}/${BUILD_MODE}/${ESP_ELF}
