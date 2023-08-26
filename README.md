# Altherma Gateway

## Installation instructions

### Configure CAN hat

Edit `/boot/config.txt`

```
# Enable Waveshare RS485 CAN HAT (B)
# https://www.waveshare.com/wiki/RS485_CAN_HAT_%28B%29
dtparam=spi=on
dtoverlay=mcp2515-can0,oscillator=16000000,interrupt=25,,spimaxfrequency=1000000
dtoverlay=sc16is752-spi1,int_pin=24
```

Create `/etc/network/interfaces.d/can0`

```
auto can0
iface can0 can static
    bitrate 20000
```
