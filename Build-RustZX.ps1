
[CmdletBinding()]
param (
    [ArgumentCompleter( { return [array](rustup toolchain list) })]
    [ValidateScript({[array](rustup toolchain list).Contains($_)},
    ErrorMessage="Toolchain {0} is not installed. Please install the toolchain using scripts from https://github.com/esp-rs/rust-build")]
    [String]
    $ToolchainName = 'esp-1.56.0.1',
    [String]
    [ValidateSet("xtensa-esp32-espidf", "xtensa-esp32s2-espidf", "xtensa-esp32s3-espidf", "riscv32imc-esp-espidf")]
    $Target = "xtensa-esp32s3-espidf",
    [String]
    [ValidateSet("esp32s3_usb_otg", "esp32s3_usb_otg", "kaluga_ili9341", "kaluga_st7789")]
    $Board = 'esp32s3_usb_otg',
    [String]
    $ApplicationFile=".\target\$Target\release\rustzx-esp32",
    [String]
    $EspIdfVersion="branch:master",
    [String]
    $Port = ""
)

$ErrorActionPreference = "Stop"

"Processing configuration:"
"-ApplicationFile  = ${ApplicationFile}"
"-Board            = ${Board}"
"-EspIdfVersion    = ${EspIdfVersion}"
"-Port             = ${Port}"
"-Target           = ${Target}"
"-ToolchainName    = ${ToolchainName}"

$env:ESP_IDF_VERSION="branch:master"

if ($false -eq $Port) {
    # Requires to be executed outside of activated ESP-IDF
    cargo +$ToolchainName build --target $Target --release --features "${Board} native"
} else {
    # Build and flash directly using `cargo install cargo-espflash --git https://github.com/jessebraham/espflash.git --branch fixes/partition-table
    cargo +$ToolchainName espflash $Port --target $Target --release --features "${Board} native"
}
