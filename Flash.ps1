esptool.py --chip esp32s2 elf2image --flash_size 2MB .\rust-esp32-std-demo -o out;  esptool.py --chip esp32s2 write_flash 0x10000 .\out; espmonitor.exe COM5
