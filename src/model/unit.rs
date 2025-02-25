use strum::EnumMessage;

#[derive(Debug, Copy, Clone, EnumMessage)]
pub enum Unit {
    #[strum(message = "°C")]
    DegCelsius,
    #[strum(message = "K")]
    Kelvin,
    #[strum(message = "kW")]
    KiloWatt,
    #[strum(message = "kWh")]
    KiloWattHours,
    #[strum(message = "l/h")]
    LitersPerHour,
    #[strum(message = "bar")]
    Bar,
    #[strum(message = "h")]
    Hours,
    #[strum(message = "%")]
    Percent,
    #[strum(message = "A")]
    Ampere,
    #[strum(message = "min")]
    Minutes,
    #[strum(message = "s")]
    Seconds,
}
