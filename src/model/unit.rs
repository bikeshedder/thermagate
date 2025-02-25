use serde::Deserialize;
use strum::EnumMessage;

#[derive(Debug, Copy, Clone, EnumMessage, Deserialize, PartialEq, Eq)]
pub enum Unit {
    #[serde(rename = "C")]
    #[strum(message = "°C")]
    DegCelsius,
    #[serde(rename = "K")]
    #[strum(message = "K")]
    Kelvin,
    #[serde(rename = "kW")]
    #[strum(message = "kW")]
    KiloWatt,
    #[serde(rename = "kWh")]
    #[strum(message = "kWh")]
    KiloWattHours,
    #[serde(rename = "l/h")]
    #[strum(message = "l/h")]
    LitersPerHour,
    #[serde(rename = "bar")]
    #[strum(message = "bar")]
    Bar,
    #[serde(rename = "h")]
    #[strum(message = "h")]
    Hours,
    #[serde(rename = "%")]
    #[strum(message = "%")]
    Percent,
    #[serde(rename = "A")]
    #[strum(message = "A")]
    Ampere,
    #[serde(rename = "min")]
    #[strum(message = "min")]
    Minutes,
    #[serde(rename = "s")]
    #[strum(message = "s")]
    Seconds,
}
