use nrg_hass::models::{
    device_class::DeviceClass, select::Select, sensor::Sensor, switch::Switch,
    unit::UnitOfMeasurement,
};

use crate::{
    can::{
        device::Device as CanDevice,
        param::{BoolParam, Enum16Param, Param as CanParam},
    },
    model::{r#enum::Enum, unit::Unit},
};

pub fn make_hass_sensor(
    device: CanDevice,
    param: &dyn CanParam,
    device_id: &str,
    topic_prefix: &str,
) -> Sensor {
    let mut b = Sensor::builder();
    b.unique_id(format!(
        "{}.{}.{:04x}",
        device_id,
        device.name().to_lowercase(),
        param.id()
    ));
    b.object_id(format!(
        "{}_{}_{}",
        device_id,
        device.name().to_lowercase(),
        param.name().to_lowercase()
    ));
    b.name(format!("Thermagate {} {}", device.name(), param.label().de));
    b.state_topic(format!(
        "{}/{}/{}",
        topic_prefix,
        device.name(),
        param.name()
    ));
    if let Some(unit) = param.unit() {
        match unit {
            Unit::DegCelsius => b
                .device_class(DeviceClass::Temperature)
                .unit_of_measurement(UnitOfMeasurement::TempCelsius),
            Unit::Kelvin => b
                .device_class(DeviceClass::Temperature)
                .unit_of_measurement(UnitOfMeasurement::TempKelvin),
            Unit::KiloWatt => b
                .device_class(DeviceClass::Power)
                .unit_of_measurement(UnitOfMeasurement::KiloWatt),
            Unit::KiloWattHours => b
                .device_class(DeviceClass::Energy)
                .unit_of_measurement(UnitOfMeasurement::KiloWattHours),
            Unit::LitersPerHour => b.unit_of_measurement(UnitOfMeasurement::CubicMetersPerHour),
            Unit::Bar => b
                .device_class(DeviceClass::Pressure)
                .unit_of_measurement(UnitOfMeasurement::Bar),
            Unit::Hours => b.unit_of_measurement(UnitOfMeasurement::Hours),
            Unit::Percent => b.unit_of_measurement(UnitOfMeasurement::Percentage),
            Unit::Ampere => b.unit_of_measurement(UnitOfMeasurement::Ampere),
            Unit::Minutes => b.unit_of_measurement(UnitOfMeasurement::Minutes),
            Unit::Seconds => b.unit_of_measurement(UnitOfMeasurement::Seconds),
        };
    }
    b.build().unwrap()
}

pub fn make_hass_select<T>(device: CanDevice, param: &Enum16Param<T>) -> Select
where
    T: Enum + From<i16> + Into<&'static str> + Sync,
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
