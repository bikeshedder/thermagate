use internment::Intern;
use rust_decimal::Decimal;

use crate::model::{
    r#enum::EnumVariant, string::MultilingualString, time::Time, time_range::TimeRange, unit::Unit,
};

#[derive(Debug)]
pub struct Param {
    pub id: u16,
    pub name: Intern<String>,
    pub label: MultilingualString,
    pub read: bool,
    pub write: bool,
    pub r#type: ParamType,
}

impl Param {
    pub fn unit(&self) -> Option<Unit> {
        match &self.r#type {
            ParamType::I8(p) => p.unit,
            ParamType::I16(p) => p.unit,
            ParamType::Dec(p) => p.unit,
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ParamType {
    Bool(BoolParam),
    I8(I8Param),
    I16(I16Param),
    Dec(DecParam),
    Enum8(EnumParam<u8>),
    Enum16(EnumParam<u16>),
    Time(TimeParam),
    TimeRange(TimeRangeParam),
}

#[derive(Debug)]
pub struct BoolParam {
    pub default: Option<bool>,
}

#[derive(Debug)]
pub struct I8Param {
    pub default: Option<i8>,
    pub unit: Option<Unit>,
    pub min: Option<i8>,
    pub max: Option<i8>,
}

#[derive(Debug)]
pub struct I16Param {
    pub default: Option<i16>,
    pub unit: Option<Unit>,
    pub min: Option<i16>,
    pub max: Option<i16>,
}

#[derive(Debug)]
pub struct DecParam {
    pub default: Option<Decimal>,
    pub unit: Option<Unit>,
    pub scale: u32,
    pub min: Option<Decimal>,
    pub max: Option<Decimal>,
}

#[derive(Debug)]
pub struct EnumParam<T: Clone> {
    pub default: Option<EnumVariant<T>>,
    pub variants: Vec<EnumVariant<T>>,
}

#[derive(Debug)]
pub struct TimeParam {
    pub default: Option<Time>,
}

#[derive(Debug)]
pub struct TimeRangeParam {
    pub default: Option<TimeRange>,
}
