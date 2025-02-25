use std::{fmt::Debug, str::FromStr};

use rust_decimal::Decimal;

use crate::model::{
    r#enum::{Enum, EnumVariant},
    string::MultilingualStr,
    time::Time,
    time_range::TimeRange,
    unit::Unit,
    value::Value,
};

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

#[derive(Debug)]
pub struct TimeRangeParam {
    pub id: u16,
    pub name: &'static str,
    pub label: MultilingualStr,
    pub read: bool,
    pub write: bool,
    pub default: Option<TimeRange>,
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
    fn decode_any(&self, data: [u8; 2]) -> Option<Value>;
    fn encode_str(&self, s: &str) -> Option<[u8; 2]> {
        // FIXME remove this default implementation
        todo!();
    }
}

pub trait DecodeParam: Param {
    type Value;
    fn decode(&self, data: [u8; 2]) -> Option<Self::Value>;
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
    fn decode_any(&self, data: [u8; 2]) -> Option<Value> {
        self.decode(data).map(Value::Bool)
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
    fn decode_any(&self, data: [u8; 2]) -> Option<Value> {
        self.decode(data).map(Value::I8)
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
    fn decode_any(&self, data: [u8; 2]) -> Option<Value> {
        self.decode(data).map(Value::I16)
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
    fn decode_any(&self, data: [u8; 2]) -> Option<Value> {
        self.decode(data).map(Value::Dec)
    }
    fn encode_str(&self, s: &str) -> Option<[u8; 2]> {
        let mut dec = Decimal::from_str(s).unwrap();
        if dec.scale() != self.scale {
            panic!("Invalid scale");
        }
        dec.set_scale(0).unwrap();
        let v: u16 = dec.try_into().unwrap();
        Some([(v >> 8) as u8, v as u8])
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
    fn decode_any(&self, data: [u8; 2]) -> Option<Value> {
        decode_i8(data).map(|v| Value::Enum8(v, Some(T::from(v.into()).into())))
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
    fn decode_any(&self, data: [u8; 2]) -> Option<Value> {
        decode_i16(data).map(|v| Value::Enum16(v, Some(T::from(v).into())))
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
    fn decode_any(&self, data: [u8; 2]) -> Option<Value> {
        self.decode(data).map(Value::Time)
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
    fn decode_any(&self, data: [u8; 2]) -> Option<Value> {
        self.decode(data).map(Value::TimeRange)
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
