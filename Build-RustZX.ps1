
[CmdletBinding()]
param (
    [Parameter()]
    [String]
    $ToolchainName = 'esp-1.56.0.1',
    [String]
    [ValidateSet("xtensa-esp32-espidf", "xtensa-esp32s2-espidf", "xtensa-esp32s3-espidf", "riscv32imc-esp-espidf")]
    $Target = "xtensa-esp32s2-espidf",
    [String]
    [ValidateSet("esp32s3_usb_otg", "esp32s3_usb_otg", "kaluga")]
    $Board = 'esp32s2_usb_otg',
    [string]
    $ApplicationFile=".\target\$Target\release\rustzx-esp32"
)

$ErrorActionPreference = "Stop"

"Processing configuration:"
"-Board            = ${Board}"
"-Target           = ${Target}"
"-ToolchainName    = ${ToolchainName}"
"-ApplicationFile  = ${ApplicationFile}"

# Requires to be executed outside of activated ESP-IDF
cargo +$ToolchainName build --target $Target --release --features "${Board} native"


# if (-Not (Test-Path -Path $ApplicationFile -PathType Leaf)) {
#     "$ApplicatioFile does not exist. Build the application"
# }

# esptool.py --chip esp32s2 elf2image --flash_size 2MB $ApplicationFile -o out
# esptool.py --chip esp32s2 write_flash 0x10000 .\out
# espmonitor.exe COM9

