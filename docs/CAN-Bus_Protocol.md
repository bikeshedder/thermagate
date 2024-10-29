# CAN-Bus Protocol

## Message IDs

The Daikin/ROTEX uses the CAN-Bus in a **non-standard** way. The message ID is not used to identify the message type and sending node but just the device sending the message.

The actual message type is then encoded as part of the first byte of the message data.

## Data encoding

The 7 `data` bytes of every CAN-bus frame are encoded the following way:

- `data[0]` stores the first (MSB) 4 bits of the recipient address followed by 4 bits of the message type:
  `data[0] = (recipient_address >> 3) & 0b11110000 | message_type & 0b00001111`
- `data[1]` contains a padding bit followed by the remaining 7 bits (LSB) of the recipient address
  `data[1] = recipient_address & 0b01111111`
- `data[2]` contains the parameter ID. The parameter ID `0xFA` denotes that the parameter ID is a 16 bit integer and the next two bytes are used to store the parameter ID instead.

Depending on `data[2]` the remaining 4 bytes are either used to just encode the value or a two byte parameter ID followed by 2 bytes for the value.

If `data[2]` is less than `0xFA`:

- `data[3]` contains the MSB of the value
- `data[4]` contains the LSB of the value
- `data[5]` is unused
- `data[6]` is unused

If `data[3]` is `0xFA`:

- `data[3]` contains the MSB of the parameter ID
- `data[4]` contains the LSB of the parameter ID
- `data[5]` contains the MSB of the value
- `data[6]` contains the LSB of the value

**Implementation note:** Parameter IDs less than `0xFA` can be encoded in two ways. Either by using a parameter ID of `0xFA` and specifying the parameter ID in `data[3]` and `data[4]` or by writing it directly into `data[2]`. The G1 gateway always uses the `0xFA` encoding even for parameters that could be encoded with a single byte parameter ID.

## Message types

- `0x0` (0) Set
- `0x1` (1) Request
- `0x2` (2) Response
- `0x6` (6) Ping
- `0x7` (7) Pong

The `Ping` and `Pong` message types are just an educated guess. When monitoring the CAN-Bus traffic they typically appear in succession. It is possible that they mean something completely different.

## Device addresses

- `0x10a` RoCon
- `0x180-0x187` HG (Heating Generator) 1-8
- `0x300-0x30f` HC (Heating Circuit) 1-16
- `0x379` HC Broadcast
- `0x500` Outdoor
- `0x600-0x60f` HCM (Heating Circuit Module) 1-16
- `0x679` HCM Broadcast
- `0x69d` G1 (Internet Gateway)

When writing data on the CAN-Bus care must be taken not to use a device address that is used by any other device. This project uses `0x666` as device address. 😈

Other device addresses can sometimes be seen on the bus but their purpose is entirely unknown.

In my current heat pump installation (Altherma ECH2O with an additional mixer unit) I can see a single Heating Generator (HG1), two Heating Circuits (HC1, HC2), two Heating Circuit Modules (HC1, HC2) and sometimes a message from the Outdoor address as well.

## Parameter IDs

The list of known parameters is long and can be found in the source code of this project.

The most reliable way to figure out parameter IDs and their meanings is to use the CAN-bus monitor. Just interact with the RoCon interface of the indoor unit and compare the data on the display with the bus messages. That way it is easy to map out the parameters.

## Quirks

- The G1 gateway requests the `SOFTWARE_NUMBER` (0x0199) to detect the existence of devices.
- The G1 gateway requests basically all known parameters from all devices even if they never report that parameter.
- Parameters that should be unique are reported by multiple devices with varying values. e.g. the `AUSSENTEMP` (0x000c, outdoor temp.) parameter is reported by `HG1`, `HCM1` and `HCM2`. `HG` reports a value for that parameter that varies sligthly from the `HCM` devices which doesn't seam to make a lot of sense.

## A word of caution!

The RoCon interface is very prone to crashing. Never send messages to it or use its device id for sending messages. If doing so the RoCon interface can freeze, crash and/or reset itself to factory defaults. Gladly all configuration except for the date and time seams to be stored outside the `RoCon` interface so after crashing it that's the only setup that needs to be done again.

I've heard from other people that it is possible to brick your HPSU if you're not careful. The entire protocol and the meaning of parameters was deduced from the bus messages, the G1 Gateway source code and other projects interfacing the HPSU via the can bus.

## Acknowledgements

- The [Rotex HPSU configuration](https://github.com/crycode-de/ioBroker.canbus/blob/master/well-known-messages/configs/rotex-hpsu.md) included in the [ioBroker.canbus](https://github.com/crycode-de/ioBroker.canbus/) repository helped me a lot to decipher the message format.
- The [fhemHPSU](https://github.com/ahermann86/fhemHPSU) project contains an exhaustive list of parameters.
- The `data.json` included in the `G1 Gateway` software contains a large number of parameters. This list is very incomplete and contains a few errors but is a good starting point for experimentation: `/var/local/rocon-g1/node_modules/rotex-control-module/data/data.json`

The mentioned sources were only used for initial experimentation. The `params.csv` and `enums.csv` files which are distributed in this repository were reverse engineered from the communication seen of an actual `ETSXB16P50D` and `EPRA14DAW1` device. It does not contain the same typos and errors of the original Rocon G1 device and is not based on its source code in any way.
