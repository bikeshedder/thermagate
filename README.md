# <img src="logo.svg" height="64px" style="height: 1.5em; vertical-align: bottom"> Thermagate

This software is designed to work for the **Daikin Altherma 3 H HT ECH<sub>2</sub>O (ETSXB16P50D)**. It should be mostly compatible with other Daikin and ROTEX heat pump models that utilize a CAN bus for communication purposes.

Daikin heat pumps that don't use a CAN bus are **NOT** supported. If your heat pump does have built-in WiFi it is a newer/different model and uses a completely different communication technology.

## Disclaimer

**⚠️ This software is provided "as is," without any warranty, including merchantability, fitness for a particular purpose, or noninfringement. The authors are not liable for any claims, damages, or liabilities. Use at your own risk.**

This is an independent project, unaffiliated with Daikin Industries, Ltd., or any of its affiliates. All trademarks are the property of their respective holders, and no affiliation or endorsement is implied.

This software is an original work, developed independently without reliance on or use of any third-party or proprietary code, intellectual property, or trade secrets. Any resemblance to other software is coincidental.

## Prerequisites

This projects requires just a CAN bus interface that is connected to your indoor unit with a bitrate of 20000. This project was designed as a single binary that can be copied either to a Raspberry PI or used as an alternative software on the RoCon G1 Gateway hardware.

While this software is useful on its own it really shines when combined with a MQTT Broker (e.g. [Mosquitto](https://mosquitto.org/)) and [Home Assistant](https://www.home-assistant.io/) for visualization and control purposes.

## Installation instructions

As of now no binary release exists, yet. You need to build the code yourself:

1. Clone this repository
2. Install Rust following the [installation instructions](https://www.rust-lang.org/learn/get-started) from rust-lang.org.
3. Run `cargo build --release` to build the binary or `cargo run` to run command directly.

If you do want to build on your local workstation and just create a binary that can be run on a Raspberry PI or similar device you can also use [`cross`](https://crates.io/crates/cross) for cross compiling the binary.

1. `cargo install cross`
2. `cross build --release --target armv7-unknown-linux-musleabihf`
3. Copy the binary `target/armv7-unknown-linux-musleabihf/release/thermagate` to the target device.

## Configuration file

The `thermagate` command comes with a `default-config` command that will print the default configuration. You can pipe that output to a `config.toml` and change it as needed:

```
thermagate default-config > config.toml
```

**👉 You probably want to change the `listen` address to something more relaxed such as `[::]:3000` (IPv6) or `0.0.0.0:3000` (IPv4) so it can be reached from the outside and modify the `[mqtt]` settings.**

The default configuration file can also be found in the sources: [`src/config/default.toml`](./src/config/default.toml)

## Uploading the software to the RoCon G1 Gateway

Please see [ROTEX RoCon G1 Gateway](./docs/ROTEX_RoCon_G1_Gateway.md) on how to gain access to that device.

While the gateway software can be installed parallel to the existing gateway software I highly recommend replacing the system entirely for reasons outlined in the document.

If you want to keep the existing gateway software for future reference you can also open the BeagleBone Black case and replace the SD card. This is also an option if some future update provided by Daikin disables the login shell or changes the password of the root user.

## Raspberry PI 4 configuration

I use a Raspberry PI 4 with a Waveshare RS485 CAN HAT (B). The following instructions are only meant when using the very same hardware.

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

After rebooting the Raspberry and hooking it up to the CAN-bus of the heat pump you should be able to run the Thermagate software with its default configuration.
