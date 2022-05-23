# RustZX for ESP32 - experimental version

Goal of the project: Run ZX Spectrum on ESP32

HW: ESP32 with ILI9341

![RustZX-ESP32](docs/rustzx-esp32-ili9341.png)

## Build using GitPod

[![Open ESP32 in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/github.com/georgik/rustzx-esp32/)

### Run simulation

```
./run-wokwi.sh
```

### Build the artifact

```
cargo build --release
```

### Other targets

- [![Open ESP32 in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/github.com/georgik/rustzx-esp32/) - ESP32 (Xtensa) - branch: [main](https://github.com/georgik/rustzx-esp32/)
- [![Open ESP32-S2 in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/github.com/georgik/rustzx-esp32/tree/target/esp32s2) - ESP32-S2 (Xtensa) - branch: [target/esp32s2](https://github.com/georgik/rustzx-esp32/tree/target/esp32s2)
- [![Open ESP32-S2 in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/github.com/georgik/rustzx-esp32/tree/target/esp32s3) - ESP32-S3 (Xtensa) - branch: [target/esp32s3](https://github.com/georgik/rustzx-esp32/tree/target/esp32s3)
- [![Open ESP32-S2 in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/github.com/georgik/rustzx-esp32/tree/target/esp32c3) - ESP32-C3 (RISC-V) - branch: [target/esp32c3](https://github.com/georgik/rustzx-esp32/tree/target/esp32c3)


## Build

Open in VS Code with Dev Container support.

- Terminal approach:

    ```
    ./build-rustzx.sh  [debug | release]
    ```
    > If no argument is passed, `release` will be used as default


- [Devcontainers] UI approach:

    The default build task is already set to build the project, and it can be used
    in VsCode and Gitpod:
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Build Task` command.
    - `Terminal`-> `Run Build Task` in the menu.
    - With `Ctrl-Shift-B` or `Cmd-Shift-B`.
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build`.
    - From UI: Press `Build` on the left side of the Status Bar.

## Build and flash
- Terminal approach:

    ```
    ./flash.sh [debug | release]
    ```
    > If no argument is passed, `release` will be used as default

- [Devcontainers] UI approach:
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build & Flash`.
    - From UI: Press `Build & Flash` on the left side of the Status Bar.
- Other:


Build for ESP32 Using cargo-espflash for ESP32-S3 USB OTG:

```
cargo +esp-1.60.0.1 espflash --target xtensa-esp32-espidf --release --features "esp32c3_ili9341"
```


Build for ESP32-S3 USB OTG sing cargo-espflash:

```
cargo +esp-1.60.0.1 espflash --target xtensa-esp32-espidf --release --features "esp32s3_usb_otg"
```

With PowerShell:

```
.\Build-RustZX.ps1 -Target xtensa-esp32s2-espidf -Board kaluga_ili9341 -Port COM23
```
### With devcontainers
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build & Flash`.
    - From UI: Press `Build & Flash` on the left side of the Status Bar.

## Build and run Wokwi simulation

- Terminal approach:

    ```
    ./run-wokwi.sh [debug | release]
    ```
    > If no argument is passed, `release` will be used as default

- [Devcontainers] UI approach:

    The default test task is already set to build the project, and it can be used
    in VsCode and Gitpod:
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Test Task` command
    - With `Ctrl-Shift-,` or `Cmd-Shift-,`
        > Note: This Shortcut is not available in Gitpod by default.
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build & Run Wokwi`.
    - From UI: Press `Build & Run Wokwi` on the left side of the Status Bar.

### Debuging with Wokwi

Wokwi offers debugging with GDB.

- [Devcontainers] Terminal approach:
    ```
    $HOME/.espressif/tools/xtensa-esp32-elf/esp-2021r2-patch3-8.4.0/xtensa-esp32-elf/bin/xtensa-esp32-elf-gdb target/xtensa-esp32-espidf/debug/rustzx-esp32 -ex "target remote localhost:9333"
    ```
- [Devcontainers] UI approach:

    Debug using with VsCode or Gitpod is also possible:
    1. Run the Wokwi Simulation in `debug` profile
        > Note that the simulation will pause if the browser tab is on the background
    2. Go to `Run and Debug` section of the IDE (`Ctrl-Shift-D or Cmd-Shift-D`)
    3. Start Debugging (`F5`)
    4. Choose the proper user:
        - `esp` when using VsCode or GitHub Codespaces
        - `gitpod` when using Gitpod

## HW Setup

### Display connection

| ILI9341 |  ESP32-DevKitS-V1.1 | Cable color |
----------|---------------------|-------------|
| GND     | GND                 | black       |
| 3.3V    | 3.3V                | red         |
| RST     | GPIO4               | orange      |
| CLK     | GPIO18              | yellow      |
| D_C     | GPIO2               | green       |
| CS      | GPIO15              | blue        |
| MOSI    | GPIO23              | purple      |
| MISO    | not connected       | -           |


Wokwi related project: https://wokwi.com/projects/331440829570744915 - [diagram.json](docs/diagram.json)

## References

- Rust code for ESP32 based on https://github.com/ivmarkov/rust-esp32-std-demo
- RustZX wrapper code reused from https://github.com/pacmancoder/rustzx
