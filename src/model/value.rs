use std::fmt;

use rust_decimal::Decimal;
use serde::Serialize;

use super::{time::Time, time_range::TimeRange};

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Value {
    Bool(bool),
    I8(i8),
    I16(i16),
    Dec(Decimal),
    Enum8(u8, Option<String>),
    Enum16(u16, Option<String>),
    TimeRange(TimeRange),
    Time(Time),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(v) => write!(f, "{}", v),
            Value::I8(v) => write!(f, "{}", v),
            Value::I16(v) => write!(f, "{}", v),
            Value::Dec(v) => write!(f, "{}", v),
            Value::Enum8(v, n) => {
                if let Some(n) = n {
                    write!(f, "{}", n)
                } else {
                    write!(f, "{}", v)
                }
            }
            Value::Enum16(v, n) => {
                if let Some(n) = n {
                    write!(f, "{}", n)
                } else {
                    write!(f, "{}", v)
                }
            }
            Value::TimeRange(v) => write!(f, "{}", v),
            Value::Time(v) => write!(f, "{}", v),
        }
    }
}
