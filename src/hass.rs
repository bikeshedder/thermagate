use std::num::NonZeroU32;

use rumqttc::AsyncClient;
use serde::Serialize;

///
#[derive(Serialize)]
pub enum UnitOfMeasurement {
    #[serde(alias = "°C")]
    TempCelsius,
}

/// https://www.home-assistant.io/integrations/sensor.mqtt/#availability
#[derive(Serialize)]
pub struct Availability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,
    pub topic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

/// https://www.home-assistant.io/integrations/sensor.mqtt/#availability_mode
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityMode {
    All,
    Any,
    Latest,
}

#[derive(Serialize)]
pub struct Device {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<(String, String)>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hw_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_area: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sw_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_device: Option<String>,
}

/// https://www.home-assistant.io/integrations/sensor/#device-class
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceClass {
    // Apparent power in VA.
    ApparentPower,
    /// Air Quality Index (unitless).
    Aqi,
    /// Atmospheric pressure in cbar, bar, hPa, inHg, kPa, mbar, Pa or psi
    AtmosphericPressure,
    /// Percentage of battery that is left in %
    Battery,
    /// Carbon Dioxide in CO2 (Smoke) in ppm
    CarbonDioxide,
    /// Carbon Monoxide in CO (Gas CNG/LPG) in ppm
    CarbonMonoxide,
    /// Current in A, mA
    Current,
    /// Data rate in bit/s, kbit/s, Mbit/s, Gbit/s, B/s, kB/s, MB/s, GB/s, KiB/s, MiB/s or GiB/s
    DataRate,
    /// Data size in bit, kbit, Mbit, Gbit, B, kB, MB, GB, TB, PB, EB, ZB, YB, KiB, MiB, GiB, TiB, PiB, EiB, ZiB or YiB
    DataSize,
    /// Date string (ISO 8601)
    Date,
    /// Generic distance in km, m, cm, mm, mi, yd, or in
    Distance,
    /// Duration in d, h, min, or s
    Duration,
    /// Energy in Wh, kWh, MWh, MJ, or GJ
    Energy,
    /// Stored energy in Wh, kWh, MWh, MJ, or GJ
    EnergyStorage,
    /// Has a limited set of (non-numeric) states
    Enum,
    /// Frequency in Hz, kHz, MHz, or GHz
    Frequency,
    /// Gasvolume in m³, ft³ or CCF
    Gas,
    /// Percentage of humidity in the air in %
    Humidity,
    /// The current light level in lx
    Illuminance,
    /// Irradiance in W/m² or BTU/(h⋅ft²)
    Irradiance,
    /// Percentage of water in a substance in %
    Moisture,
    /// The monetary value (ISO 4217)
    Monetary,
    /// Concentration of Nitrogen Dioxide in µg/m³
    NitrogenDioxide,
    /// Concentration of Nitrogen Monoxide in µg/m³
    NitrogenMonoxide,
    /// Concentration of Nitrous Oxide in µg/m³
    NitrousOxide,
    /// Concentration of Ozone in µg/m³
    Ozone,
    /// Concentration of particulate matter less than 1 micrometer in µg/m³
    Pm1,
    /// Concentration of particulate matter less than 2.5 micrometers in µg/m³
    Pm25,
    /// Concentration of particulate matter less than 10 micrometers in µg/m³
    Pm10,
    /// Power factor (unitless), unit may be None or %
    PowerFactor,
    /// Power in W or kW
    Power,
    /// Accumulated precipitation in cm, in or mm
    Precipitation,
    /// Precipitation intensity in in/d, in/h, mm/d or mm/h
    PrecipitationIntensity,
    /// Pressure in Pa, kPa, hPa, bar, cbar, mbar, mmHg, inHg or psi
    Pressure,
    /// Reactive power in var
    ReactivePower,
    /// Signal strength in dB or dBm
    SignalStrength,
    /// Sound pressure in dB or dBA
    SoundPressure,
    /// Generic speed in ft/s, in/d, in/h, km/h, kn, m/s, mph or mm/d
    Speed,
    /// Concentration of sulphur dioxide in µg/m³
    SulphurDioxide,
    /// Temperature in °C, °F or K
    Temperature,
    /// Datetime object or timestamp string (ISO 8601)
    Timestamp,
    /// Concentration of volatile organic compounds in µg/m³
    VolatileOrganicCompounds,
    /// Ratio of volatile organic compounds in ppm or ppb
    VolatileOrganicCompoundsParts,
    /// Voltage in V, mV
    Voltage,
    /// Generic volume in L, mL, gal, fl. oz., m³, ft³, or CCF
    Volume,
    /// Generic stored volume in L, mL, gal, fl. oz., m³, ft³, or CCF
    VolumeStorage,
    /// Water consumption in L, gal, m³, ft³, or CCF
    Water,
    /// Generic mass in kg, g, mg, µg, oz, lb, or st
    Weight,
    /// Wind speed in ft/s, km/h, kn, m/s, or mph
    WindSpeed,
}

/// https://developers.home-assistant.io/docs/core/entity/#generic-properties
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityCategory {
    Config,
    Diagnostic,
}

#[derive(Serialize)]
#[repr(u8)]
pub enum Qos {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce,
}

/// https://developers.home-assistant.io/docs/core/entity/sensor/#available-state-classes
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StateClass {
    Measurement,
    Total,
    TotalIncreasing,
}

/// https://www.home-assistant.io/integrations/sensor.mqtt/
#[derive(Serialize, Default)]
pub struct Sensor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availabilty: Option<Vec<Availability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<AvailabilityMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class: Option<DeviceClass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_after: Option<NonZeroU32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_update: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_reset_value_template: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_display_precision: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_class: Option<StateClass>,
    pub state_topic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measurement: Option<UnitOfMeasurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Sensor {
    pub async fn register(
        &self,
        client: &AsyncClient,
        hass_discovery_prefix: &str,
        device_prefix: &str,
    ) -> Result<(), rumqttc::ClientError> {
        let json = serde_json::to_string(&self).unwrap();
        client
            .publish(
                format!("{hass_discovery_prefix}/sensor/{device_prefix}/HCM2_cAUSSENTEMP/config"),
                rumqttc::QoS::AtLeastOnce,
                true,
                json,
            )
            .await
    }

    pub async fn value(
        &self,
        client: &AsyncClient,
        payload: &str,
    ) -> Result<(), rumqttc::ClientError> {
        client
            .publish(&self.state_topic, rumqttc::QoS::AtLeastOnce, true, payload)
            .await
    }
}
