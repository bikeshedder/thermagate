use nrg_hass::models::{
    device_class::DeviceClass, select::Select, sensor::Sensor, switch::Switch,
    unit::UnitOfMeasurement,
};

use crate::can::{
    device::Device as CanDevice,
    param::{BoolParam, Enum16Param, Param as CanParam, Unit as CanUnit},
    r#enum::Enum,
};

pub fn make_hass_sensor(device: CanDevice, param: &dyn CanParam) -> Sensor {
    let mut b = Sensor::builder();
    b.unique_id(format!(
        "{}.{}.{:04x}",
        "altherma",
        device.name().to_lowercase(),
        param.id()
    ));
    b.object_id(format!(
        "{}_{}_{}",
        "thermagate",
        device.name().to_lowercase(),
        param.name().to_lowercase()
    ));
    b.name(format!("Thermagate {} {}", device.name(), param.label().de));
    b.state_topic(format!("thermagate/{}/{}", device.name(), param.name()));
    if let Some(unit) = param.unit() {
        match unit {
            CanUnit::DegCelsius => b
                .device_class(DeviceClass::Temperature)
                .unit_of_measurement(UnitOfMeasurement::TempCelsius),
            CanUnit::Kelvin => b
                .device_class(DeviceClass::Temperature)
                .unit_of_measurement(UnitOfMeasurement::TempKelvin),
            CanUnit::KiloWatt => b
                .device_class(DeviceClass::Power)
                .unit_of_measurement(UnitOfMeasurement::KiloWatt),
            CanUnit::KiloWattHours => b
                .device_class(DeviceClass::Energy)
                .unit_of_measurement(UnitOfMeasurement::KiloWattHours),
            CanUnit::LitersPerHour => b.unit_of_measurement(UnitOfMeasurement::CubicMetersPerHour),
            CanUnit::Bar => b
                .device_class(DeviceClass::Pressure)
                .unit_of_measurement(UnitOfMeasurement::Bar),
            CanUnit::Hours => b.unit_of_measurement(UnitOfMeasurement::Hours),
            CanUnit::Percent => b.unit_of_measurement(UnitOfMeasurement::Percentage),
            CanUnit::Ampere => b.unit_of_measurement(UnitOfMeasurement::Ampere),
            CanUnit::Minutes => b.unit_of_measurement(UnitOfMeasurement::Minutes),
            CanUnit::Seconds => b.unit_of_measurement(UnitOfMeasurement::Seconds),
        };
    }
    b.build().unwrap()
}

pub fn make_hass_select<T: Enum>(device: CanDevice, param: &Enum16Param<T>) -> Select
where
    T: From<i16> + Into<&'static str> + Sync,
{
    let mut b = Select::builder();
    b.unique_id(format!(
        "{}.{}.{:04x}",
        "altherma",
        device.name().to_lowercase(),
        param.id()
    ));
    b.object_id(format!(
        "{}_{}_{}",
        "altherma",
        device.name().to_lowercase(),
        param.name().to_lowercase()
    ));
    b.name(format!("Altherma {} {}", device.name(), param.label().de));
    b.options(
        T::VARIANTS
            .iter()
            .map(|c| c.code.to_owned())
            .collect::<Vec<_>>(),
    );
    b.state_topic(format!("altherma/{}/{}", device.name(), param.name()));
    b.command_topic(format!("altherma/{}/{}/set", device.name(), param.name()));
    b.build().unwrap()
}

pub fn make_hass_switch(device: CanDevice, param: &BoolParam) -> Switch {
    let mut b = Switch::builder();
    b.unique_id(format!(
        "{}.{}.{:04x}",
        "altherma",
        device.name().to_lowercase(),
        param.id()
    ));
    b.object_id(format!(
        "{}_{}_{}",
        "altherma",
        device.name().to_lowercase(),
        param.name().to_lowercase()
    ));
    b.name(format!("Altherma {} {}", device.name(), param.label().de));
    b.state_topic(format!("altherma/{}/{}", device.name(), param.name()));
    b.command_topic(format!("altherma/{}/{}/set", device.name(), param.name()));
    b.build().unwrap()
}
