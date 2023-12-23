use nrg_hass::models::{device_class::DeviceClass, sensor::Sensor, unit::UnitOfMeasurement};

use crate::can::{
    device::Device as CanDevice,
    param::{Param as CanParam, Unit as CanUnit},
};

pub fn make_hass_sensor(device: CanDevice, param: &dyn CanParam) -> Sensor {
    let mut b = Sensor::builder();
    b.object_id(format!(
        "{}_{}_{}",
        "altherma_gateway",
        device.name().to_lowercase(),
        param.name().to_lowercase()
    ));
    b.name(format!("Altherma {} {}", device.name(), param.label().de));
    b.state_topic(format!(
        "altherma-gateway/{}/{}",
        device.name(),
        param.name()
    ));
    if let Some(unit) = param.unit() {
        match unit {
            CanUnit::DegCelsius => b
                .device_class(DeviceClass::Temperature)
                .unit_of_measurement(UnitOfMeasurement::TempCelsius),
            CanUnit::KiloWatt => b
                .device_class(DeviceClass::Power)
                .unit_of_measurement(UnitOfMeasurement::Watt),
            CanUnit::KiloWattHours => b
                .device_class(DeviceClass::Energy)
                .unit_of_measurement(UnitOfMeasurement::WattHours),
            CanUnit::LitersPerHour => b.unit_of_measurement(UnitOfMeasurement::CubicMetersPerHour),
            CanUnit::Bar => b
                .device_class(DeviceClass::Pressure)
                .unit_of_measurement(UnitOfMeasurement::Bar),
            CanUnit::Hours => b.unit_of_measurement(UnitOfMeasurement::Hours),
            CanUnit::Percent => b.unit_of_measurement(UnitOfMeasurement::Percentage),
        };
    }
    b.build().unwrap()
}
