use std::fmt;
use std::io::BufReader;
use std::{fs::File, io};
use std::path::Path;

use serde::{Deserialize, Deserializer};
use thiserror::Error;

fn deserialize_u8_hex<'de, D>(deserializer: D) -> Result<u8, D::Error> where D: Deserializer<'de> {
    struct HexStringVisitor;
    impl<'de> serde::de::Visitor<'de> for HexStringVisitor {
        type Value = u8;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing a u8 encoded as hexadecimal string prefixed with '0x'")
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

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("input output error")]
    IO(#[from] io::Error),
    #[error("deserialization error")]
    Deserialize(#[from] serde_json::Error),
}

#[derive(Debug, Deserialize)]
pub struct RotexData {
    pub parameters: Vec<Parameter>
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
    #[serde(alias="infoNumber")]
    pub info_number: InfoNumber,
    #[serde(flatten)]
    pub r#type: ParameterType,
    #[serde(default)]
    pub writeable: bool,
    #[serde(alias="transferThreshold", default="bool_true")]
    pub display: bool,
    #[serde(alias="waterCircuit", default)]
    pub water_circuit: bool,
}

#[derive(Debug, Deserialize)]
pub struct InfoNumber {
    #[serde(alias="byteHigh", deserialize_with="deserialize_u8_hex")]
    pub byte_high: u8,
    #[serde(alias="byteLow", deserialize_with="deserialize_u8_hex")]
    pub byte_low: u8,
}

#[derive(Debug, Deserialize)]
pub enum ParameterType {
    #[serde(alias="bool")]
    Bool {
        default: bool
    },
    #[serde(alias="int")]
    Int {
        #[serde(alias="bigEndian")]
        big_endian: bool,
        min: Option<u16>,
        max: Option<u16>,
        default: Option<u16>,
        #[serde(alias="transferDelay")]
        transfer_delay: u16,
        #[serde(alias="delayedByValue")]
        delayed_by_value: Vec<u16>,
    },
    #[serde(alias="float")]
    Float {
        #[serde(alias="bigEndian")]
        big_endian: Option<bool>,
        #[serde(alias="transferThreshold")]
        transfer_threshold: bool,
        factor: u32,
        min: Option<f32>,
        max: Option<f32>,
        default: Option<f32>,
    },
    #[serde(alias="enum")]
    Enum {
        #[serde(alias="bigEndian")]
        big_endian: Option<bool>,
        r#in: Vec<u8>,
    },
    TimeRange {
        default: String,
    }
}
