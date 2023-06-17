use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rotex::{self, HexStr, RotexData};

#[derive(Debug)]
pub struct State {
    pub devices: HashMap<String, Device>,
    pub device_by_address: HashMap<Address, (String, Op)>,
    pub parameters: HashMap<String, Parameter>,
    pub parameter_by_address: HashMap<u16, String>,
}

#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub device_type: DeviceType,
    pub get: Address,
    pub set: Address,
    pub answer: Address,
}

#[derive(Debug)]
pub enum DeviceType {
    HeatGeneator,
    HeatingCircuit,
    HeatingCircuitModule,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Address(pub u32, pub u8, pub u8, pub u8);

impl Address {
    pub fn from_rotex_data(data: &(HexStr<u32>, HexStr<u8>, HexStr<u8>, HexStr<u8>)) -> Self {
        Self(data.0 .0, data.1 .0, data.2 .0, data.3 .0)
    }
}

impl Device {
    pub fn from_rotex_data(data: &rotex::Device, device_type: DeviceType) -> Self {
        Self {
            name: data.name.clone(),
            device_type,
            get: Address::from_rotex_data(&data.get),
            set: Address::from_rotex_data(&data.set),
            answer: Address::from_rotex_data(&data.answer),
        }
    }
}

#[derive(Debug)]
pub enum Op {
    Get,
    Set,
    Answer,
}

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
        let parameters = data
            .parameters
            .iter()
            .map(|p| {
                (
                    p.name.clone(),
                    match &p.r#type {
                        crate::rotex::ParameterType::Float { factor, .. } => Parameter::Float {
                            meta: ParameterMeta {
                                name: p.name.clone(),
                            },
                            factor: factor.0,
                            value: None,
                        },
                        _ => Parameter::Float {
                            meta: ParameterMeta {
                                name: p.name.clone(),
                            },
                            factor: 1.0,
                            value: None,
                        },
                    },
                )
            })
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterMeta {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Parameter {
    Float {
        #[serde(flatten)]
        meta: ParameterMeta,
        value: Option<f32>,
        factor: f32,
    },
}
