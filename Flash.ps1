
$ErrorActionPreference = "Stop"

esptool.py --chip esp32s2 elf2image --flash_size 2MB .\target\xtensa-esp32s2-espidf\release\rust-esp32-std-demo -o out
esptool.py --chip esp32s2 write_flash 0x10000 .\out
espmonitor.exe COM5

