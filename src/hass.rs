use nrg_hass::{
    config::HomeAssistantConfig,
    models::{
        device::Device as HassDevice, device_class::DeviceClass, entity_category::EntityCategory,
        select::Select, sensor::Sensor, state_class::StateClass, switch::Switch,
        unit::UnitOfMeasurement,
    },
};

use crate::{
    can::device::Device as CanDevice,
    catalog::param::{Param, ParamType},
    model::unit::Unit,
};

/// Home Assistant device representing the gateway itself. All CAN bus devices
/// are linked to it via `via_device`.
pub fn make_hass_gateway_device(hass: &HomeAssistantConfig) -> HassDevice {
    let mut b = HassDevice::builder();
    b.identifiers(vec![hass.object_id.clone()])
        .name(hass.name.clone())
        .manufacturer("Thermagate")
        .model("CAN bus gateway")
        .sw_version(env!("CARGO_PKG_VERSION"));
    b.build().unwrap()
}

/// Home Assistant device representing a single CAN bus device (heat
/// generator, heating circuit, outdoor unit, ...). Entities of the same CAN
/// bus device are grouped together and the device is linked to the gateway.
pub fn make_hass_device(device: CanDevice, hass: &HomeAssistantConfig) -> HassDevice {
    let mut b = HassDevice::builder();
    b.identifiers(vec![format!(
        "{}_{}",
        hass.object_id,
        device.to_string().to_lowercase()
    )])
    .name(format!("{} {}", hass.name, device))
    .manufacturer("ROTEX / Daikin")
    .model(device_model(device))
    .via_device(hass.object_id.clone());
    b.build().unwrap()
}

fn device_model(device: CanDevice) -> &'static str {
    let name = device.name();
    if name.starts_with("HCM") {
        "Heating circuit module"
    } else if name.starts_with("HC") {
        "Heating circuit"
    } else if name.starts_with("HG") {
        "Heat generator"
    } else {
        match device {
            CanDevice::Outdoor => "Outdoor unit",
            CanDevice::RoCon | CanDevice::RoCon2 => "RoCon control panel",
            CanDevice::G1 => "RoCon G1 gateway",
            CanDevice::TG => "Thermagate",
            _ => "Unknown device",
        }
    }
}

/// State class of a sensor. Counters (energy, runtime) are monotonically
/// increasing, physical values are measurements. Without a state class Home
/// Assistant keeps no long-term statistics and energy sensors can not be used
/// in the energy dashboard.
fn state_class(param: &Param) -> Option<StateClass> {
    match param.unit()? {
        Unit::KiloWattHours | Unit::Hours => Some(StateClass::TotalIncreasing),
        Unit::DegCelsius
        | Unit::KiloWatt
        | Unit::LitersPerHour
        | Unit::Bar
        | Unit::Percent
        | Unit::Ampere => Some(StateClass::Measurement),
        // Used for configuration values (hysteresis, delays) only
        Unit::Kelvin | Unit::Minutes | Unit::Seconds => None,
    }
}

/// Diagnostic sensor of the gateway device publishing the Thermagate version.
/// It makes sure the gateway device exists in Home Assistant.
pub fn make_hass_gateway_sensor(hass: &HomeAssistantConfig, topic_prefix: &str) -> Sensor {
    let mut b = Sensor::builder();
    b.unique_id(format!("{}.gateway.version", hass.object_id))
        .object_id(format!("{}_gateway_version", hass.object_id))
        // Home Assistant prefixes the device name ("Thermagate")
        .name("Version")
        .state_topic(format!("{}/gateway/version", topic_prefix))
        .entity_category(EntityCategory::Diagnostic)
        .device(make_hass_gateway_device(hass));
    b.build().unwrap()
}

pub fn make_hass_sensor(
    device: CanDevice,
    param: &Param,
    hass: &HomeAssistantConfig,
    topic_prefix: &str,
) -> Sensor {
    let mut b = Sensor::builder();
    b.unique_id(format!(
        "{}.{}.{:04x}",
        hass.object_id,
        device.name().to_lowercase(),
        param.id
    ));
    b.object_id(format!(
        "{}_{}_{}",
        hass.object_id,
        device.name().to_lowercase(),
        param.name.to_lowercase()
    ));
    // Home Assistant prefixes the device name ("Thermagate HG1"), so the
    // resulting friendly name stays "Thermagate HG1 <label>" as before.
    b.name(param.label.de.to_string());
    b.state_topic(format!("{}/{}/{}", topic_prefix, device.name(), param.name));
    b.device(make_hass_device(device, hass));
    if let Some(state_class) = state_class(param) {
        b.state_class(state_class);
    }
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

pub fn make_hass_select(device: CanDevice, param: &Param) -> Option<Select> {
    let options: Vec<String> = match &param.r#type {
        ParamType::Enum8(p) => p.variants.iter().map(|v| (*v.code).clone()).collect(),
        ParamType::Enum16(p) => p.variants.iter().map(|v| (*v.code).clone()).collect(),
        _ => return None,
    };
    let mut b = Select::builder();
    b.unique_id(format!(
        "{}.{}.{:04x}",
        "altherma",
        device.name().to_lowercase(),
        param.id
    ));
    b.object_id(format!(
        "{}_{}_{}",
        "altherma",
        device.name().to_lowercase(),
        param.name.to_lowercase()
    ));
    b.name(format!("Altherma {} {}", device.name(), param.label.de));
    b.options(options);
    b.state_topic(format!("altherma/{}/{}", device.name(), param.name));
    b.command_topic(format!("altherma/{}/{}/set", device.name(), param.name));
    Some(b.build().unwrap())
}

pub fn make_hass_switch(device: CanDevice, param: &Param) -> Switch {
    let mut b = Switch::builder();
    b.unique_id(format!(
        "{}.{}.{:04x}",
        "altherma",
        device.name().to_lowercase(),
        param.id
    ));
    b.object_id(format!(
        "{}_{}_{}",
        "altherma",
        device.name().to_lowercase(),
        param.name.to_lowercase()
    ));
    b.name(format!("Altherma {} {}", device.name(), param.label.de));
    b.state_topic(format!("altherma/{}/{}", device.name(), param.name));
    b.command_topic(format!("altherma/{}/{}/set", device.name(), param.name));
    b.build().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::Catalog;

    fn hass() -> HomeAssistantConfig {
        HomeAssistantConfig {
            discovery_prefix: "homeassistant".into(),
            object_id: "thermagate".into(),
            name: "Thermagate".into(),
        }
    }

    fn sensor_json(device: CanDevice, param: &str) -> serde_json::Value {
        let catalog = Catalog::load().unwrap();
        let param = catalog.param_by_name(param).unwrap();
        serde_json::to_value(make_hass_sensor(device, param, &hass(), "thermagate")).unwrap()
    }

    #[test]
    fn test_sensor_ids_unchanged() {
        let json = sensor_json(CanDevice::HG1, "energy_hp_total");
        assert_eq!(json["unique_id"], "thermagate.hg1.0930");
        assert_eq!(json["object_id"], "thermagate_hg1_energy_hp_total");
        assert_eq!(json["state_topic"], "thermagate/HG1/energy_hp_total");
        // Home Assistant prepends the device name "Thermagate HG1"
        assert_eq!(json["name"], "Energie WP gesamt");
    }

    #[test]
    fn test_sensor_device() {
        let json = sensor_json(CanDevice::Outdoor, "status_compressor");
        assert_eq!(json["device"]["identifiers"][0], "thermagate_outdoor");
        assert_eq!(json["device"]["name"], "Thermagate Outdoor");
        assert_eq!(json["device"]["model"], "Outdoor unit");
        assert_eq!(json["device"]["via_device"], "thermagate");
    }

    #[test]
    fn test_sensor_state_class() {
        let energy = sensor_json(CanDevice::HG1, "energy_hp_total");
        assert_eq!(energy["state_class"], "total_increasing");
        let runtime = sensor_json(CanDevice::HG1, "runtime_compressor");
        assert_eq!(runtime["state_class"], "total_increasing");
        let temp = sensor_json(CanDevice::HG1, "outside_temp");
        assert_eq!(temp["state_class"], "measurement");
        let mode = sensor_json(CanDevice::HG1, "operating_mode");
        assert!(mode.get("state_class").is_none());
    }

    #[test]
    fn test_gateway_sensor() {
        let json = serde_json::to_value(make_hass_gateway_sensor(&hass(), "thermagate")).unwrap();
        assert_eq!(json["device"]["identifiers"][0], "thermagate");
        assert_eq!(json["device"]["sw_version"], env!("CARGO_PKG_VERSION"));
        assert_eq!(json["entity_category"], "diagnostic");
        assert_eq!(json["name"], "Version");
        assert_eq!(json["state_topic"], "thermagate/gateway/version");
    }
}
