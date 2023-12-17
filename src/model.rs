use std::collections::HashMap;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

time::serde::format_description!(hhmm_format, Time, "[hour]:[minute]");

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub devices: IndexMap<String, Device>,
    pub parameters: IndexMap<String, Parameter>,
}

#[derive(Debug)]
pub struct State {
    pub devices: HashMap<String, Device>,
    pub device_by_address: HashMap<Address, (String, Op)>,
    pub parameters: HashMap<String, Parameter>,
    pub parameter_by_address: HashMap<u16, String>,
}

impl State {
    pub fn new(devices: HashMap<String, Device>, parameters: HashMap<String, Parameter>) -> Self {
        let device_by_address = devices
            .iter()
            .flat_map(|(name, device)| {
                [
                    (device.get, (name.clone(), Op::Get)),
                    (device.set, (name.clone(), Op::Set)),
                    (device.answer, (name.clone(), Op::Answer)),
                ]
            })
            .map(|(name, x)| (name, x))
            .collect::<HashMap<_, _>>();
        let parameter_by_address = parameters
            .iter()
            .map(|(name, parameter)| (parameter.info_number, name.clone()))
            .collect::<HashMap<_, _>>();
        Self {
            devices,
            device_by_address,
            parameters,
            parameter_by_address,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub device_type: DeviceType,
    pub get: Address,
    pub set: Address,
    pub answer: Address,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    HeatGeneator,
    HeatingCircuit,
    HeatingCircuitModule,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Address(pub u32, pub u8, pub u8, pub u8);

#[derive(Debug, Copy, Clone, Serialize)]
pub enum Op {
    Get,
    Set,
    Answer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterMeta {
    name: String,
}

pub enum CanPayloadError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoolParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FloatParameter {
    pub factor: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<Vec<u16>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeRange {
    #[serde(with = "hhmm_format")]
    pub start: time::Time,
    #[serde(with = "hhmm_format")]
    pub end: time::Time,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeRangeParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<TimeRange>,
}

fn is_false(v: &bool) -> bool {
    !v
}

fn default_false() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub info_number: u16,
    #[serde(skip_serializing_if = "is_false", default = "default_false")]
    pub big_endian: bool,
    #[serde(flatten)]
    pub r#type: ParameterType,
}

#[derive(Debug, Serialize, Deserialize, AsRefStr)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ParameterType {
    Bool(BoolParameter),
    Int(IntParameter),
    Float(FloatParameter),
    Enum(EnumParameter),
    TimeRange(TimeRangeParameter),
}
