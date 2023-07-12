use std::collections::{HashMap, BTreeMap};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_hex::{SerHex, StrictPfx};
use time::macros::format_description;

use crate::rotex::{self, HexStr, RotexData};

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
    pub parameters: HashMap<String, ParameterType>,
    pub parameter_by_address: HashMap<u16, String>,
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

#[derive(Debug)]
pub enum Op {
    Get,
    Set,
    Answer,
}

/*
impl State {
    pub fn from_rotex_data(data: &RotexData) -> Self {
        let devices = data
            .heat_generators
            .iter()
            .map(|d| Device::from_rotex_data(d, DeviceType::HeatGeneator))
            .chain(
                data.heating_circuits
                    .iter()
                    .map(|d| Device::from_rotex_data(d, DeviceType::HeatingCircuit)),
            )
            .chain(
                data.heating_circuit_modules
                    .iter()
                    .map(|d| Device::from_rotex_data(d, DeviceType::HeatingCircuit)),
            )
            .map(|d| (d.name.clone(), d))
            .collect::<HashMap<_, _>>();
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
        let parameter_by_address = data
            .parameters
            .iter()
            .map(|p| (p.info_number.as_u16(), p.name.clone()))
            .collect::<HashMap<_, _>>();
        Self {
            devices,
            device_by_address,
            parameters,
            parameter_by_address,
        }
    }
}
 */

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub info_number: u16,
    #[serde(skip_serializing_if = "is_false")]
    pub big_endian: bool,
    #[serde(flatten)]
    pub r#type: ParameterType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ParameterType {
    Bool(BoolParameter),
    Int(IntParameter),
    Float(FloatParameter),
    Enum(EnumParameter),
    TimeRange(TimeRangeParameter),
}
