use std::fmt;
use std::io::BufReader;
use std::path::Path;
use std::{fs::File, io};

use serde::de::Unexpected;
use serde::{Deserialize, Deserializer};
use thiserror::Error;

fn deserialize_u8_hex<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    struct HexStringVisitor;
    impl<'de> serde::de::Visitor<'de> for HexStringVisitor {
        type Value = u8;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(
                "a string containing a u8 encoded as hexadecimal string prefixed with '0x'",
            )
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(0)
            //serde_json::from_str(v).map_err(E::custom)
        }
    }
    deserializer.deserialize_str(HexStringVisitor)
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

fn deserialize_option_bool_str<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        Ok(Some(s.parse::<bool>().map_err(serde::de::Error::custom)?))
    } else {
        Ok(None)
    }
}

fn deserialize_bool_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<bool>().map_err(serde::de::Error::custom)
}

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("input output error")]
    IO(#[from] io::Error),
    #[error("deserialization error")]
    Deserialize(#[from] serde_json::Error),
}

#[derive(Debug, Deserialize)]
pub struct RotexData {
    pub parameters: Vec<Parameter>,
}

impl RotexData {
    pub fn read(filename: &Path) -> Result<Self, LoadError> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader)?;
        Ok(data)
    }
}

fn bool_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(alias = "infoNumber")]
    pub info_number: InfoNumber,
    #[serde(flatten)]
    pub r#type: ParameterType,
    #[serde(default, deserialize_with = "deserialize_bool_str")]
    pub writeable: bool,
    #[serde(default = "bool_true", deserialize_with = "deserialize_bool_str")]
    pub display: bool,
    #[serde(alias = "waterCircuit", default, deserialize_with = "deserialize_bool_str")]
    pub water_circuit: bool,
}

#[derive(Debug, Deserialize)]
pub struct InfoNumber {
    #[serde(alias = "byteHigh", deserialize_with = "deserialize_u8_hex")]
    pub byte_high: u8,
    #[serde(alias = "byteLow", deserialize_with = "deserialize_u8_hex")]
    pub byte_low: u8,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ParameterType {
    #[serde(alias = "bool")]
    Bool {
        #[serde(deserialize_with = "deserialize_option_bool_str")]
        default: Option<bool>,
    },
    #[serde(alias = "int")]
    Int {
        #[serde(alias = "bigEndian", default, deserialize_with = "deserialize_bool_str")]
        big_endian: bool,
        min: Option<u16>,
        max: Option<u16>,
        default: Option<u16>,
        #[serde(alias = "transferDelay")]
        transfer_delay: Option<u16>,
        #[serde(alias = "delayedByValue")]
        delayed_by_value: Option<Vec<u16>>,
    },
    #[serde(alias = "float")]
    Float {
        #[serde(alias = "bigEndian", default, deserialize_with = "deserialize_bool_str")]
        big_endian: bool,
        #[serde(alias = "transferThreshold", deserialize_with = "deserialize_f32_str")]
        transfer_threshold: f32,
        #[serde(alias = "transferThreshold", deserialize_with = "deserialize_f32_str")]
        factor: f32,
        min: Option<f32>,
        max: Option<f32>,
        default: Option<f32>,
    },
    #[serde(alias = "enum")]
    Enum {
        #[serde(alias = "bigEndian", default, deserialize_with = "deserialize_bool_str")]
        big_endian: bool,
        r#in: Option<Vec<u8>>,
    },
    TimeRange {
        default: String,
    },
}
