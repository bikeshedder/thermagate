use std::{
    fmt::{self, Debug},
    str::FromStr,
};

use rust_decimal::Decimal;
use serde::Serialize;
use strum::EnumMessage;

use super::r#enum::{Enum, EnumVariant};

#[derive(Debug, Default)]
pub struct BoolParam {
    pub id: u16,
    pub name: &'static str,
    pub label: MultilingualStr,
    pub read: bool,
    pub write: bool,
    pub default: Option<bool>,
}

#[derive(Debug, Default)]
pub struct I8Param {
    pub id: u16,
    pub name: &'static str,
    pub label: MultilingualStr,
    pub read: bool,
    pub write: bool,
    pub default: Option<i8>,
    pub unit: Option<Unit>,
    pub min: Option<i8>,
    pub max: Option<i8>,
}

#[derive(Debug, Default)]
pub struct I16Param {
    pub id: u16,
    pub name: &'static str,
    pub label: MultilingualStr,
    pub read: bool,
    pub write: bool,
    pub default: Option<i16>,
    pub unit: Option<Unit>,
    pub min: Option<i16>,
    pub max: Option<i16>,
}

#[derive(Debug)]
pub struct DecParam {
    pub id: u16,
    pub name: &'static str,
    pub label: MultilingualStr,
    pub read: bool,
    pub write: bool,
    pub default: Option<Decimal>,
    pub unit: Option<Unit>,
    pub scale: u32,
    pub min: Option<Decimal>,
    pub max: Option<Decimal>,
}

#[derive(Debug)]
pub struct Enum8Param<T> {
    pub id: u16,
    pub name: &'static str,
    pub label: MultilingualStr,
    pub read: bool,
    pub write: bool,
    pub default: Option<T>,
}

#[derive(Debug)]
pub struct Enum16Param<T: Enum> {
    pub id: u16,
    pub name: &'static str,
    pub label: MultilingualStr,
    pub read: bool,
    pub write: bool,
    pub default: Option<T>,
}

#[derive(Debug)]
pub struct TimeParam {
    pub id: u16,
    pub name: &'static str,
    pub label: MultilingualStr,
    pub read: bool,
    pub write: bool,
    pub default: Option<Time>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Time {
    hour: u8,
    minute: u8,
}

impl Time {
    pub const fn new(hour: u8, minute: u8) -> Option<Self> {
        if hour > 24 {
            return None;
        }
        if !matches!(minute, 0 | 15 | 30 | 45) {
            return None;
        }
        if hour == 24 && minute != 0 {
            return None;
        }
        Some(Self { hour, minute })
    }
    #[track_caller]
    pub const fn new_const(hour: u8, minute: u8) -> Self {
        match Self::new(hour, minute) {
            Some(time) => time,
            None => panic!("Invalid time"),
        }
    }
    pub fn hour(&self) -> u8 {
        self.hour
    }
    pub fn minute(&self) -> u8 {
        self.minute
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

#[derive(Debug)]
pub struct TimeRangeParam {
    pub id: u16,
    pub name: &'static str,
    pub label: MultilingualStr,
    pub read: bool,
    pub write: bool,
    pub default: Option<TimeRange>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TimeRange {
    pub start: Time,
    pub end: Time,
}

impl fmt::Display for TimeRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct MultilingualStr {
    pub de: &'static str,
    pub en: &'static str,
}

pub trait Param: Sync + Debug {
    fn id(&self) -> u16;
    fn name(&self) -> &'static str;
    fn label(&self) -> MultilingualStr;
    fn unit(&self) -> Option<Unit> {
        None
    }
    fn choices(&self) -> Option<&'static [EnumVariant]> {
        None
    }
    fn decode_any(&self, data: [u8; 2]) -> Option<AnyValue>;
    fn encode_str(&self, s: &str) -> Option<[u8; 2]> {
        // FIXME remove this default implementation
        todo!();
    }
}

pub trait DecodeParam: Param {
    type Value;
    fn decode(&self, data: [u8; 2]) -> Option<Self::Value>;
}

#[derive(Debug, Clone, Serialize)]
pub enum AnyValue {
    Bool(bool),
    I8(i8),
    I16(i16),
    Dec(Decimal),
    Enum8(i8, Option<&'static str>),
    Enum16(i16, Option<&'static str>),
    TimeRange(TimeRange),
    Time(Time),
}

impl fmt::Display for AnyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnyValue::Bool(v) => write!(f, "{}", v),
            AnyValue::I8(v) => write!(f, "{}", v),
            AnyValue::I16(v) => write!(f, "{}", v),
            AnyValue::Dec(v) => write!(f, "{}", v),
            AnyValue::Enum8(v, n) => {
                if let Some(n) = n {
                    write!(f, "{}", n)
                } else {
                    write!(f, "{}", v)
                }
            }
            AnyValue::Enum16(v, n) => {
                if let Some(n) = n {
                    write!(f, "{}", n)
                } else {
                    write!(f, "{}", v)
                }
            }
            AnyValue::TimeRange(v) => write!(f, "{}", v),
            AnyValue::Time(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumMessage)]
pub enum Unit {
    #[strum(message = "°C")]
    DegCelsius,
    #[strum(message = "K")]
    Kelvin,
    #[strum(message = "kW")]
    KiloWatt,
    #[strum(message = "kWh")]
    KiloWattHours,
    #[strum(message = "l/h")]
    LitersPerHour,
    #[strum(message = "bar")]
    Bar,
    #[strum(message = "h")]
    Hours,
    #[strum(message = "%")]
    Percent,
    #[strum(message = "A")]
    Ampere,
    #[strum(message = "min")]
    Minutes,
    #[strum(message = "s")]
    Seconds,
}

impl Param for BoolParam {
    fn id(&self) -> u16 {
        self.id
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn label(&self) -> MultilingualStr {
        self.label
    }
    fn decode_any(&self, data: [u8; 2]) -> Option<AnyValue> {
        self.decode(data).map(AnyValue::Bool)
    }
}

impl DecodeParam for BoolParam {
    type Value = bool;
    fn decode(&self, data: [u8; 2]) -> Option<Self::Value> {
        decode_i16(data).map(|v| v != 0)
    }
}

impl Param for I8Param {
    fn id(&self) -> u16 {
        self.id
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn label(&self) -> MultilingualStr {
        self.label
    }
    fn unit(&self) -> Option<Unit> {
        self.unit
    }
    fn decode_any(&self, data: [u8; 2]) -> Option<AnyValue> {
        self.decode(data).map(AnyValue::I8)
    }
}

impl DecodeParam for I8Param {
    type Value = i8;
    fn decode(&self, data: [u8; 2]) -> Option<Self::Value> {
        decode_i8(data)
    }
}

impl Param for I16Param {
    fn id(&self) -> u16 {
        self.id
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn label(&self) -> MultilingualStr {
        self.label
    }
    fn unit(&self) -> Option<Unit> {
        self.unit
    }
    fn decode_any(&self, data: [u8; 2]) -> Option<AnyValue> {
        self.decode(data).map(AnyValue::I16)
    }
}

impl DecodeParam for I16Param {
    type Value = i16;
    fn decode(&self, data: [u8; 2]) -> Option<Self::Value> {
        decode_i16(data)
    }
}

impl Param for DecParam {
    fn id(&self) -> u16 {
        self.id
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn label(&self) -> MultilingualStr {
        self.label
    }
    fn unit(&self) -> Option<Unit> {
        self.unit
    }
    fn decode_any(&self, data: [u8; 2]) -> Option<AnyValue> {
        self.decode(data).map(AnyValue::Dec)
    }
    fn encode_str(&self, s: &str) -> Option<[u8; 2]> {
        let mut dec = Decimal::from_str(s).unwrap();
        if dec.scale() != self.scale {
            panic!("Invalid scale");
        }
        dec.set_scale(0).unwrap();
        let v: u16 = dec.try_into().unwrap();
        Some([(v >> 8) as u8, (v >> 0) as u8])
    }
}

impl DecodeParam for DecParam {
    type Value = Decimal;
    fn decode(&self, data: [u8; 2]) -> Option<Self::Value> {
        decode_i16(data).map(|v| Decimal::new(v.into(), self.scale))
    }
}

impl<T> Param for Enum8Param<T>
where
    T: Enum,
{
    fn id(&self) -> u16 {
        self.id
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn label(&self) -> MultilingualStr {
        self.label
    }
    fn choices(&self) -> Option<&'static [EnumVariant]> {
        Some(T::VARIANTS)
    }
    fn decode_any(&self, data: [u8; 2]) -> Option<AnyValue> {
        decode_i8(data).map(|v| AnyValue::Enum8(v, Some(T::from(v.into()).into())))
    }
}

impl<T> DecodeParam for Enum8Param<T>
where
    T: Enum,
{
    type Value = T;
    fn decode(&self, data: [u8; 2]) -> Option<Self::Value> {
        decode_i8(data).map(|v| v.into()).map(From::from)
    }
}

impl<T> Param for Enum16Param<T>
where
    T: Enum,
{
    fn id(&self) -> u16 {
        self.id
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn label(&self) -> MultilingualStr {
        self.label
    }
    fn choices(&self) -> Option<&'static [EnumVariant]> {
        Some(T::VARIANTS)
    }
    fn decode_any(&self, data: [u8; 2]) -> Option<AnyValue> {
        decode_i16(data).map(|v| AnyValue::Enum16(v, Some(T::from(v).into())))
    }
}

impl<T> DecodeParam for Enum16Param<T>
where
    T: Enum,
{
    type Value = T;
    fn decode(&self, data: [u8; 2]) -> Option<Self::Value> {
        decode_i16(data).map(From::from)
    }
}

impl Param for TimeParam {
    fn id(&self) -> u16 {
        self.id
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn label(&self) -> MultilingualStr {
        self.label
    }
    fn decode_any(&self, data: [u8; 2]) -> Option<AnyValue> {
        self.decode(data).map(AnyValue::Time)
    }
}

impl DecodeParam for TimeParam {
    type Value = Time;
    fn decode(&self, data: [u8; 2]) -> Option<Self::Value> {
        Time::new(data[1] / 4, (data[1] % 4) * 15)
    }
}

impl Param for TimeRangeParam {
    fn id(&self) -> u16 {
        self.id
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn label(&self) -> MultilingualStr {
        self.label
    }
    fn decode_any(&self, data: [u8; 2]) -> Option<AnyValue> {
        self.decode(data).map(AnyValue::TimeRange)
    }
}

impl DecodeParam for TimeRangeParam {
    type Value = TimeRange;
    fn decode(&self, data: [u8; 2]) -> Option<Self::Value> {
        if data[0] == 0x80 {
            return None;
        }
        // FIXME better handling if start/end don't parse. At least do some logging or
        // mark the value as errnous.
        let start = Time::new(data[0] / 4, (data[0] % 4) * 15)?;
        let end = Time::new(data[1] / 4, (data[1] % 4) * 15)?;
        Some(TimeRange { start, end })
    }
}

fn decode_i8(data: [u8; 2]) -> Option<i8> {
    let v = data[0] as i8;
    if v == i8::MIN {
        None
    } else {
        Some(v)
    }
}

fn decode_i16(data: [u8; 2]) -> Option<i16> {
    let v = ((data[0] as i16) << 8) + (data[1] as i16);
    if v == i16::MIN {
        None
    } else {
        Some(v)
    }
}

#[test]
fn test_decparam_encode_str() {
    use super::params::MAX_FEED_TEMP;
    assert_eq!(MAX_FEED_TEMP.encode_str("25.0"), Some([0x00, 0xFA]));
    assert_eq!(MAX_FEED_TEMP.encode_str("25.5"), Some([0x00, 0xFF]));
    assert_eq!(MAX_FEED_TEMP.encode_str("25.6"), Some([0x01, 0x00]));
}
