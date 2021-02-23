# rtl8720dn

A working project for the rtl8720dn/BW16 using Rust.

## Notes

### Links

- [Board used](https://www.aliexpress.com/item/1005001768102716.html)
- [rtl8720dn data sheet](https://files.seeedstudio.com/products/102110419/Basic%20documents/00014457-UM0401-RTL872xD-Datasheet-v1.7_205016.pdf)

### Serial flash wiring

The demo board has different labels for the top & bottom, making working with it on a breadboard painful.
Included top labels for reference.

| function | Blue Pill (BMP) | RTL8720DN   |
| -------- | --------------- | ----------- |
| 5V       | 5V              | VIN         |
| GND      | GND             | GND         |
| TX -> RX | B6 TX           | LOG_RX/GE1  |
| RX -> TX | B7 RX           | LOG_TX/GA4  |

### SWD Wiring

| function | Blue Pill (BMP) | RTL8720DN   |
| -------- | --------------- | ----------- |
| 5V       | 5V              | VIN         |
| GND      | GND             | GND         |
| SWDIO    | A13 SWDIO       | ???         |
| SWCLK    | A14 SWCLK       | ???         |
| SWOUT    | A3              | ???         |

### Blue Pill BMP reference table

Refer [here](https://github.com/blacksphere/blackmagic/tree/master/src/platforms/swlink) for up to date information.

|  Function   | PIN   | STM8S-DISCO | BLUEPILL    |
| ----------- | ----- | ----------- | ----------- |
|  JTMS/SWDIO |  PA13 |   CN5/5     |  P2/2       |
|  JTCK/SWCLK |  PA14 |   CN5/4     |  P2/3       |
|  JTDI       |  PA15 |   CN5/6     |  P4/11 (38) |
|  JTDO       |  PB3  |   CN5/3     |  P4/10 (39) |
|  SRST       |  PB4  |   CN5/8     |  P4/9  (40) |
|  UART1_TX   |  PB6  |   CN7/4     |  P4/7  (42) |
|  UART1_RX   |  PB7  |   CN7/2     |  P4/6  (43) |
|  SWO/RX2    |  PA3  |   NA(*1)    |  P3/7  (13) |
