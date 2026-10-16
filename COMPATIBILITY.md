# Compatibility

Thermagate talks to the RoCon controller of ROTEX / Daikin heat pumps via the
CAN bus. Most parameters are shared between models, but some are missing or
behave differently depending on the indoor unit, the controller software and
the installed options. This list collects setups that are known to work.

**Please add your setup** via pull request or in an issue. The most useful
details are listed in [Adding your setup](#adding-your-setup) below.

## Overview

| Indoor unit | Controller software | Gateway hardware | Thermagate | Reported by |
| --- | --- | --- | --- | --- |
| Daikin Altherma 3 H HT ECH<sub>2</sub>O (ETSXB16P50D) | ? | Raspberry Pi 4 + Waveshare RS485 CAN HAT (B) | development | @bikeshedder |
| 2 × Daikin Altherma EHSX16P50BB (≈ ROTEX HPSU compact 516 h/c) | RoCon BM1 366.04 | Raspberry Pi 5 + MCP2515 CAN board | 0.2.0 (Docker) | @thorhansen79 ([#2](https://github.com/bikeshedder/thermagate/issues/2)) |
| ROTEX HPSU compact 508 H/C DB (V5.2, 8 kW) | RoCon BE/BM 366.04 G, B1/U1 367.02 A | ROTEX RoCon G1 (BeagleBone Black) | 0.2.2 | @hennr ([#2](https://github.com/bikeshedder/thermagate/issues/2)) |

All setups use a CAN bitrate of 20 kbit/s.

## Details

### Daikin Altherma 3 H HT ECH<sub>2</sub>O (ETSXB16P50D)

The system Thermagate was developed with.

- **Gateway:** Raspberry Pi 4 with Waveshare RS485 CAN HAT (B), see
  [README](README.md#raspberry-pi-4-configuration)
- **Controller software:** ?
- **Options:** probably a mixer module (RoCon M1), as the default configuration
  queries `HC2` / `HCM2`

### Daikin Altherma EHSX16P50BB

Two units, the Daikin version of the ROTEX HPSU compact (closest ROTEX model:
HPSU compact 516 h/c).

- **Controller:** RoCon BM1, software 366.04
- **Gateway:** Raspberry Pi 5 with MCP2515 CAN board, Thermagate 0.2.0 in Docker
- **Usage:** values via MQTT in Home Assistant; parameters are written from
  Home Assistant automations as well (setpoints, modes, heating curve, DHW)
- **Options:** no mixer module (`HC2` / `HCM2` parameters do not answer)

### ROTEX HPSU compact 508 H/C DB

- **Indoor unit:** HPSU compact 508 H/C DB, article no. 141575-01, label on the
  control housing: "HPSU compact V5.2 / 8kW"
- **Outdoor unit:** 8 kW (RRLQ…CAV3 series)
- **Controller software:** labels on the control housing "SW RoCon BE/BM-366-04 G"
  and "SW RoCon B1/U1-367-02 A"; display (Info): "SwNr Regler 366 Rev. 00",
  "SwNr B1/U1 367 Rev. 02 A"; Thermagate reads `version_major` = 366,
  `version_minor` = 4 from `HG1`
- **Options:** electrical backup heater (3 × 3 kW), no mixer module, no RoCon U1
- **Gateway:** ROTEX RoCon G1, i.e. BeagleBone Black Rev C with a cape whose
  EEPROM identifies it as "DAIKIN CANBUS CAPE" (manufacturer "ROTEXHeatingSyst",
  part number `BB-BONE-SERL-01`). The cape uses a TI ISO1050 isolated CAN
  transceiver on DCAN1 (P9.24 / P9.26). Runs Debian 13 (kernel 6.12) from SD card
  with the overlay `BB-CAN1-00A0.dtbo`; the interface shows up as `can0`.
- **Verified:** all values that are also shown on the RoCon display match
  (temperatures, pressure, flow, mode, energy counters, runtimes).
- **Not available** on this unit (no valid value): `energy_electrical`
  (`0xc2fa`), `pwm_pump` at the `Outdoor` device, `t_au` (optional outdoor
  sensor not installed).
- **Notes:**
  - `energy_hot_water` includes the energy of the backup heater
    (`energy_ext_hot_water`), `energy_hp_total` ("Energie erzeugt") contains the
    heat pump only.
  - `buh_current_output` and the backup heater energy counters
    (`energy_ext_hot_water`, `energy_ext_heating`) are calculated from the
    configured power (`ext_power_hot_water`, `ext_power_stage_1/2`), they are
    not measured.
  - Some names do not match the display: `feed_temperature_current` (`0x000d`)
    is "T-HK" on the display, "T-WE" corresponds to `feed_temp_phx`, and
    "T-Aussen" corresponds to `average_outside_temp`.

## Gateway hardware

| Hardware | CAN interface | Notes |
| --- | --- | --- |
| Raspberry Pi 4 | Waveshare RS485 CAN HAT (B), MCP2515 | see [README](README.md#raspberry-pi-4-configuration) |
| Raspberry Pi 5 | MCP2515 CAN board | |
| ROTEX RoCon G1 | BeagleBone Black DCAN1 + ISO1050 cape | see [ROTEX RoCon G1 Gateway](docs/ROTEX_RoCon_G1_Gateway.md); boot from SD card requires holding the BOOT button (S2) while powering on, as long as the original system is still on the eMMC |

## Adding your setup

Please include as much as you know:

- Indoor unit: model and article number (type plate), ROTEX or Daikin
- Outdoor unit
- Controller software: display → Info → "SwNr Regler" / "SwNr B1/U1", or
  query `version_major` and `version_minor` at `HG1`
- Options: backup heater, mixer module (RoCon M1), room station (RoCon U1), …
- Gateway hardware and CAN interface
- Thermagate version
- Parameters that do not answer (timeout warnings in the log) or show values
  that differ from the display
