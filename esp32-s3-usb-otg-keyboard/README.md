| Supported Targets | ESP32-S2 | ESP32-S3 |
| ----------------- | -------- | -------- |

# USB HID Class example
This example implements a basic USB Host HID Class Driver, and demonstrates how to use the driver to communicate with USB HID devices (such as Keyboard and Mouse or both) on the ESP32-S2/S3. Currently, the example only supports the HID boot protocol which should be present on most USB Mouse and Keyboards. The example will continuously scan for the connection of any HID Mouse or Keyboard, and attempt to fetch HID reports from those devices once connected. To quit the example (and shut down the HID driver), users can GPIO0 to low (i.e., pressing the "Boot" button on most ESP dev kits).


### Hardware Required
* Development board with USB capable ESP SoC (ESP32-S2/ESP32-S3)
* **USB DEV port**: Use this for power supply to the device and connected USB Host devices. Connect a USB cable here for power.
* **USB-UART port**: Use this only for programming and monitoring via a separate USB cable.
* USB OTG Cable

### Common Pin Assignments

If your board doesn't have a USB A connector connected to the dedicated GPIOs, 
you may have to DIY a cable and connect **D+** and **D-** to the pins listed below.

```
ESP BOARD    USB CONNECTOR (type A)
                   --
                  | || VCC
[GPIO19]  ------> | || D-
[GPIO20]  ------> | || D+
                  | || GND
                   --

### Power Configuration for ESP32-S3-USB-OTG Board

This project is specifically configured for the ESP32-S3-USB-OTG development board with optimal power routing:

#### Power Setup
1. **USB DEV Port (USB-A Male)**: Connect your main power USB cable here
   - Powers the ESP32-S3 board
   - Automatically routes power to USB Host port for connected devices
   - This single cable powers everything!

2. **USB-UART Port (USB-C)**: Connect a separate programming cable here
   - Used only for flashing firmware and serial monitoring
   - Cannot power USB Host devices (hardware limitation)

3. **USB HOST Port (USB-A Female)**: Connect your USB keyboard/mouse here
   - Receives power from USB DEV port automatically
   - No separate power cable needed

#### GPIO Power Control (Automatic)
The firmware automatically configures:
- **GPIO12** (DEV_VBUS_EN): Routes power from USB DEV to USB HOST
- **GPIO17** (IDEV_LIMIT_EN): Enables 500mA current limiting
- **GPIO18** (USB_SEL): Selects USB DEV port for data routing

#### Alternative Power Modes
- **Battery Mode**: Can power USB Host from battery via boost converter (GPIO13)
- **External Power**: Use separate 5V supply if needed

### Build and Flash

Build the project and flash it to the board, then run monitor tool to view serial output:

```
idf.py -p PORT flash monitor
```

The example serial output will be the following:

```
I (198) example: HID HOST example
I (598) example: Interface number 0, protocol Mouse
I (598) example: Interface number 1, protocol Keyboard

Mouse
X: 000883       Y: 000058       |o| |
Keyboard
qwertyuiop[]\asdfghjkl;'zxcvbnm,./
Mouse
X: 000883       Y: 000058       | |o|
```

Where every keyboard key printed as char symbol if it is possible and a Hex value for any other key. 

#### Keyboard input data
Keyboard input data starts with the word "Keyboard" and every pressed key is printed to the serial debug.
Left or right Shift modifier is also supported. 

```
Keyboard
Hello, ESP32 USB HID Keyboard is here!
```

#### Mouse input data 
Mouse input data starts with the word "Mouse" and has the following structure. 
```
Mouse
X: -00343   Y: 000183   | |o|
     |            |      | |
     |            |      | +- Right mouse button pressed status ("o" - pressed, " " - not pressed)
     |            |      +--- Left mouse button pressed status ("o" - pressed, " " - not pressed)
     |            +---------- Y relative coordinate of the cursor 
     +----------------------- X relative coordinate of the cursor 
```
