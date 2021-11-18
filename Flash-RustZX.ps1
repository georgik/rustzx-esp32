[CmdletBinding()]
param (
    [Parameter()]
    [String]
    $ToolchainName = 'esp-1.56.0.1',
    [String]
    [ValidateSet("xtensa-esp32-espidf", "xtensa-esp32s2-espidf", "xtensa-esp32s3-espidf", "riscv32imc-esp-espidf")]
    $Target = "xtensa-esp32s3-espidf",
    [String]
    [ValidateSet("esp32", "esp32s2", "esp32s3")]
    $Chip = "esp32s3",
    [String]
    [ValidateSet("esp32s3_usb_otg", "esp32s3_usb_otg", "kaluga")]
    $Board = 'esp32s3_usb_otg',
    [String]
    $ApplicationFile=".\target\$Target\release\rustzx-esp32",
    [String]
    $Port = "COM10"
)

$ErrorActionPreference = "Stop"

"Processing configuration:"
"-Board            = ${Board}"
"-Target           = ${Target}"
"-Chip             = ${Chip}"
"-ToolchainName    = ${ToolchainName}"
"-ApplicationFile  = ${ApplicationFile}"


$ErrorActionPreference = "Stop"

if (-Not (Test-Path -Path $ApplicationFile -PathType Leaf)) {
    "$ApplicatioFile does not exist. Build the application"
}

# Requires to be executed in activated ESP-IDF
rm out
esptool.py --chip ${Chip} elf2image --flash_size 4MB $ApplicationFile -o out
esptool.py --chip ${Chip} write_flash 0x10000 .\out
espmonitor.exe ${Port}

