use std::fmt;
use std::marker::PhantomData;
use std::num::ParseIntError;

use indexmap::IndexMap;
use lazy_static::lazy_static;
use regex::Regex;
use serde::de::Unexpected;
use serde::{Deserialize, Deserializer};
use time::macros::format_description;
use time::Time;

use crate::model;

pub trait FromHexStr
where
    Self: Sized,
{
    fn from_hex_str(s: &str) -> Result<Self, ParseIntError>;
}

impl FromHexStr for u8 {
    fn from_hex_str(s: &str) -> Result<u8, ParseIntError> {
        u8::from_str_radix(s, 16)
    }
}

impl FromHexStr for u32 {
    fn from_hex_str(s: &str) -> Result<u32, ParseIntError> {
        u32::from_str_radix(s, 16)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HexStr<T: FromHexStr>(pub T);

impl<'de, T: FromHexStr> Deserialize<'de> for HexStr<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HexStringVisitor<T: FromHexStr>(PhantomData<T>);

        impl<'de, T: FromHexStr> serde::de::Visitor<'de> for HexStringVisitor<T> {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    "a string containing an integer encoded as hexadecimal string prefixed with '0x'",
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.len() < 3 {
                    return Err(E::invalid_value(Unexpected::Str(v), &self));
                }
                if !v.starts_with("0x") {
                    return Err(E::invalid_value(Unexpected::Str(v), &self));
                }
                T::from_hex_str(&v[2..]).map_err(|_| E::invalid_value(Unexpected::Str(v), &self))
            }
        }

        let visitor = HexStringVisitor(PhantomData::<T>::default());

        deserializer.deserialize_str(visitor).map(HexStr)
    }
}

fn deserialize_f32_str<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    struct FloatStringVisitor;
    impl<'de> serde::de::Visitor<'de> for FloatStringVisitor {
        type Value = f32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing a floating point number")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.parse::<f32>()
                .map_err(|_| E::invalid_value(Unexpected::Str(v), &self))
        }
    }
    deserializer.deserialize_str(FloatStringVisitor)
}

#[derive(Debug, Deserialize)]
pub struct U16Str(#[serde(deserialize_with = "deserialize_u16_str")] pub u16);

fn deserialize_u16_str<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<u16>().map_err(serde::de::Error::custom)
}

#[derive(Debug, Deserialize)]
pub struct F32Str(#[serde(deserialize_with = "deserialize_f32_str")] pub f32);

fn deserialize_bool_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<bool>().map_err(serde::de::Error::custom)
}

#[derive(Debug, Deserialize)]
pub struct BoolStr(#[serde(deserialize_with = "deserialize_bool_str")] pub bool);

impl BoolStr {
    fn true_value() -> Self {
        Self(true)
    }
    fn false_value() -> Self {
        Self(false)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RotexData {
    pub parameters: Vec<Parameter>,
    pub heat_generators: Vec<Device>,
    pub heating_circuits: Vec<Device>,
    pub heating_circuit_modules: Vec<Device>,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Address(HexStr<u32>, HexStr<u8>, HexStr<u8>, HexStr<u8>);

impl From<Address> for model::Address {
    fn from(val: Address) -> Self {
        model::Address(val.0 .0, val.1 .0, val.2 .0, val.3 .0)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub name: String,
    pub get: Address,
    pub set: Address,
    pub answer: Address,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    pub info_number: InfoNumber,
    #[serde(flatten)]
    pub r#type: ParameterType,
    #[serde(default = "BoolStr::false_value")]
    pub writeable: BoolStr,
    #[serde(default = "BoolStr::true_value")]
    pub display: BoolStr,
    #[serde(default = "BoolStr::false_value")]
    pub water_circuit: BoolStr,
    #[serde(default = "BoolStr::false_value")]
    big_endian: BoolStr,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoNumber {
    pub byte_high: HexStr<u8>,
    pub byte_low: HexStr<u8>,
}

impl InfoNumber {
    pub fn as_u16(&self) -> u16 {
        ((self.byte_high.0 as u16) << 8) + (self.byte_low.0 as u16)
    }
}

#[derive(Debug)]
pub struct TimeRange {
    pub start: Time,
    pub end: Time,
}

lazy_static! {
    static ref TIME_RANGE_REGEX: Regex =
        Regex::new(r"^((?:[01]\d|2[0-3]):(?:00|15|30|45))-((?:[01]\d|2[0-3]):(?:00|15|30|45))$")
            .unwrap();
}

impl<'de> Deserialize<'de> for TimeRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeRangeVisitor;

        impl<'de> serde::de::Visitor<'de> for TimeRangeVisitor {
            type Value = TimeRange;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string containing a time range in the format HH:MM-HH:MM")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let Some(captures) = TIME_RANGE_REGEX.captures(v) else {
                    return Err(E::invalid_value(Unexpected::Str(v), &self));
                };
                let (Some(start), Some(end)) = (captures.get(1), captures.get(2)) else {
                    return Err(E::invalid_value(Unexpected::Str(v), &self));
                };
                let time_format = format_description!("[hour]:[minute]");
                let start = Time::parse(start.as_str(), time_format)
                    .map_err(|_| E::invalid_value(Unexpected::Str(v), &self))?;
                let end = Time::parse(end.as_str(), time_format)
                    .map_err(|_| E::invalid_value(Unexpected::Str(v), &self))?;
                Ok(TimeRange { start, end })
            }
        }

        let visitor = TimeRangeVisitor;

        deserializer.deserialize_str(visitor)
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum ParameterType {
    Bool {
        default: Option<BoolStr>,
    },
    Int {
        min: Option<U16Str>,
        max: Option<U16Str>,
        default: Option<U16Str>,
        #[serde(alias = "transferDelay")]
        transfer_delay: Option<U16Str>,
        #[serde(alias = "delayedByValue")]
        delayed_by_value: Option<Vec<U16Str>>,
    },
    Float {
        transfer_threshold: Option<F32Str>,
        factor: F32Str,
        min: Option<F32Str>,
        max: Option<F32Str>,
        default: Option<F32Str>,
    },
    Enum {
        r#in: Option<Vec<U16Str>>,
    },
    TimeRange {
        default: Option<TimeRange>,
    },
}

impl From<RotexData> for model::Data {
    fn from(val: RotexData) -> Self {
        let devices = val
            .heat_generators
            .iter()
            .map(|d| (d, model::DeviceType::HeatGeneator))
            .chain(
                val.heating_circuits
                    .iter()
                    .map(|d| (d, model::DeviceType::HeatingCircuit)),
            )
            .chain(
                val.heating_circuit_modules
                    .iter()
                    .map(|d| (d, model::DeviceType::HeatingCircuit)),
            )
            .map(|(d, device_type)| {
                (
                    d.name.clone(),
                    model::Device {
                        device_type,
                        get: d.get.into(),
                        set: d.set.into(),
                        answer: d.answer.into(),
                    },
                )
            })
            .collect::<IndexMap<_, _>>();
        let parameters = val
            .parameters
            .iter()
            .map(|p| {
                (
                    p.name.clone(),
                    model::Parameter {
                        info_number: p.info_number.as_u16(),
                        big_endian: p.big_endian.0,
                        r#type: match &p.r#type {
                            ParameterType::Bool { default } => {
                                model::ParameterType::Bool(model::BoolParameter {
                                    default: default.as_ref().map(|v| v.0),
                                })
                            }
                            ParameterType::Int {
                                min,
                                max,
                                default,
                                transfer_delay: _,
                                delayed_by_value: _,
                            } => model::ParameterType::Int(model::IntParameter {
                                min: min.as_ref().map(|v| v.0),
                                max: max.as_ref().map(|v| v.0),
                                default: default.as_ref().map(|v| v.0),
                            }),
                            ParameterType::Float {
                                transfer_threshold: _,
                                factor,
                                min,
                                max,
                                default,
                            } => model::ParameterType::Float(model::FloatParameter {
                                factor: factor.0,
                                min: min.as_ref().map(|v| v.0),
                                max: max.as_ref().map(|v| v.0),
                                default: default.as_ref().map(|v| v.0),
                            }),
                            ParameterType::Enum { r#in } => {
                                model::ParameterType::Enum(model::EnumParameter {
                                    r#in: r#in.as_ref().map(|v| v.iter().map(|w| w.0).collect()),
                                })
                            }
                            ParameterType::TimeRange { default } => {
                                model::ParameterType::TimeRange(model::TimeRangeParameter {
                                    default: default.as_ref().map(|v| model::TimeRange {
                                        start: v.start,
                                        end: v.end,
                                    }),
                                })
                            }
                        },
                    },
                )
            })
            .collect::<IndexMap<_, _>>();
        model::Data {
            devices,
            parameters,
        }
    }
}
