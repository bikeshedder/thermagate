use std::fmt;
use std::io::BufReader;
use std::marker::PhantomData;
use std::num::ParseIntError;
use std::path::Path;
use std::{fs::File, io};

use serde::de::Unexpected;
use serde::{Deserialize, Deserializer};
use thiserror::Error;

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

#[derive(Debug)]
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

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("input output error")]
    IO(#[from] io::Error),
    #[error("deserialization error")]
    Deserialize(#[from] serde_json::Error),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RotexData {
    pub parameters: Vec<Parameter>,
    pub heat_generators: Vec<Device>,
    pub heating_circuits: Vec<Device>,
    pub heating_circuit_modules: Vec<Device>,
}

impl RotexData {
    pub fn read(filename: &Path) -> Result<Self, LoadError> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader)?;
        Ok(data)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub name: String,
    pub get: (HexStr<u32>, HexStr<u8>, HexStr<u8>, HexStr<u8>),
    pub set: (HexStr<u32>, HexStr<u8>, HexStr<u8>, HexStr<u8>),
    pub answer: (HexStr<u32>, HexStr<u8>, HexStr<u8>, HexStr<u8>),
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

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum ParameterType {
    #[serde(alias = "bool")]
    Bool {
        default: Option<BoolStr>,
    },
    #[serde(alias = "int")]
    Int {
        #[serde(default = "BoolStr::false_value")]
        big_endian: BoolStr,
        min: Option<U16Str>,
        max: Option<U16Str>,
        default: Option<U16Str>,
        #[serde(alias = "transferDelay")]
        transfer_delay: Option<U16Str>,
        #[serde(alias = "delayedByValue")]
        delayed_by_value: Option<Vec<U16Str>>,
    },
    #[serde(alias = "float")]
    Float {
        #[serde(default = "BoolStr::false_value")]
        big_endian: BoolStr,
        transfer_threshold: Option<F32Str>,
        factor: F32Str,
        min: Option<F32Str>,
        max: Option<F32Str>,
        default: Option<F32Str>,
    },
    #[serde(alias = "enum")]
    Enum {
        #[serde(default = "BoolStr::false_value")]
        big_endian: BoolStr,
        r#in: Option<Vec<U16Str>>,
    },
    TimeRange {
        default: Option<String>,
    },
}
