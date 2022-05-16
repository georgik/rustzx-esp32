#!/bin/bash

cargo +esp build --target xtensa-esp32-espidf --release --features "esp32_ili9341"
