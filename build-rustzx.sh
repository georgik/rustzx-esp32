#!/bin/bash

# Gitpod and VsCode Codespaces tasks do not source the user environment
if [ "${USER}" == "gitpod" ]; then
    which idf.py >/dev/null || {
        source ~/export-rust.sh > /dev/null 2>&1
    }
elif [ "${CODESPACE_NAME}" != "" ]; then
    which idf.py >/dev/null || {
        source ~/export-rust.sh > /dev/null 2>&1
    }
fi


case "$1" in
    ""|"release")
        cargo +esp build --target xtensa-esp32-espidf --release --features "esp32_ili9341"
        ;;
    "debug")
        cargo +esp build --target xtensa-esp32-espidf --features "esp32_ili9341"
        ;;
    *)
        echo "Wrong argument. Only \"debug\"/\"release\" arguments are supported"
        exit 1;;
esac
