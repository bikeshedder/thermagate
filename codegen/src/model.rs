use std::{collections::BTreeMap, str::FromStr, sync::LazyLock};

use regex::Regex;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::{macros::format_description, Time};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct Document {
    #[serde(flatten)]
    pub params: BTreeMap<String, Param>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub label: Label,
    #[serde(flatten)]
    pub r#type: Type,
    #[serde(default)]
    pub read: bool,
    pub write: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Label {
    pub de: String,
    pub en: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Type {
    Bool(BoolParam),
    I8(I8Param),
    I16(I16Param),
    Dec(DecParam),
    Enum8(EnumParam),
    Enum16(EnumParam),
    TimeRange(TimeRangeParam),
    Time(TimeParam),
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BoolParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct I8Param {
    pub unit: Option<Unit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<i8>,
    pub min: Option<i8>,
    pub max: Option<i8>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct I16Param {
    pub unit: Option<Unit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i16>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecParam {
    pub unit: Option<Unit>,
    pub scale: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Decimal>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumParam {
    pub r#enum: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<u16>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Unit {
    #[serde(rename = "C")]
    DecCelsius,
    #[serde(rename = "K")]
    Kelvin,
    #[serde(rename = "h")]
    Hours,
    #[serde(rename = "kW")]
    KiloWatt,
    #[serde(rename = "kWh")]
    KiloWattHours,
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "%")]
    Percent,
    #[serde(rename = "l/h")]
    LitersPerHour,
    #[serde(rename = "A")]
    Ampere,
    #[serde(rename = "min")]
    Minutes,
    #[serde(rename = "s")]
    Seconds,
}

impl Unit {
    pub fn as_str(&self) -> &str {
        match self {
            Unit::DecCelsius => "C",
            Unit::Kelvin => "K",
            Unit::Hours => "h",
            Unit::KiloWatt => "kW",
            Unit::KiloWattHours => "kWh",
            Unit::Bar => "bar",
            Unit::Percent => "%",
            Unit::LitersPerHour => "l/h",
            Unit::Ampere => "A",
            Unit::Minutes => "min",
            Unit::Seconds => "s",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeRangeParam {
    pub default: Option<TimeRange>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: Time,
    pub end: Time,
}

impl ToString for TimeRange {
    fn to_string(&self) -> String {
        let format = format_description!("[hour]:[minute]");
        format!(
            "{}-{}",
            self.start.format(&format).unwrap(),
            self.end.format(&format).unwrap(),
        )
    }
}

impl FromStr for TimeRange {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let format = format_description!("[hour]:[minute]");
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(\d{2}:\d{2})-(\d{2}:\d{2})").unwrap());
        let c = RE.captures(s).ok_or(())?;
        Ok(Self {
            start: Time::parse(&c[1], format).unwrap(),
            end: Time::parse(&c[2], format).unwrap(),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeParam {
    pub default: Option<Time>,
}
