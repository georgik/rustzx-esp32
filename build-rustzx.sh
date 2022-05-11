#!/bin/bash

cargo +nightly build --target riscv32imc-esp-espidf --release --features "esp32c3_ili9341"

