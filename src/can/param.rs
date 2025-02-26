use std::str::FromStr;

use rust_decimal::Decimal;

use crate::{
    catalog::param::{
        BoolParam, DecParam, EnumParam, I8Param, I16Param, Param, ParamType, TimeParam,
        TimeRangeParam,
    },
    model::{time::Time, time_range::TimeRange, value::Value},
};

pub trait AsCanParam {
    fn as_can_param(&self) -> &dyn CanParam;
}

pub trait CanParam {
    fn decode(&self, data: [u8; 2]) -> Option<Value>;
    fn encode_str(&self, value: &str) -> Option<[u8; 2]> {
        todo!();
    }
}

impl AsCanParam for Param {
    fn as_can_param(&self) -> &dyn CanParam {
        match &self.r#type {
            ParamType::Bool(x) => x,
            ParamType::I8(x) => x,
            ParamType::I16(x) => x,
            ParamType::Dec(x) => x,
            ParamType::Enum8(x) => x,
            ParamType::Enum16(x) => x,
            ParamType::Time(x) => x,
            ParamType::TimeRange(x) => x,
        }
    }
}

impl CanParam for Param {
    fn decode(&self, data: [u8; 2]) -> Option<Value> {
        self.as_can_param().decode(data)
    }
    fn encode_str(&self, value: &str) -> Option<[u8; 2]> {
        self.as_can_param().encode_str(value)
    }
}

impl CanParam for BoolParam {
    fn decode(&self, data: [u8; 2]) -> Option<Value> {
        decode_i16(data).map(|v| v != 0).map(Value::Bool)
    }
}

impl CanParam for I8Param {
    fn decode(&self, data: [u8; 2]) -> Option<Value> {
        decode_i8(data).map(Value::I8)
    }
}

impl CanParam for I16Param {
    fn decode(&self, data: [u8; 2]) -> Option<Value> {
        decode_i16(data).map(Value::I16)
    }
}

impl CanParam for DecParam {
    fn decode(&self, data: [u8; 2]) -> Option<Value> {
        decode_i16(data)
            .map(|v| Decimal::new(v.into(), self.scale))
            .map(Value::Dec)
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

impl CanParam for EnumParam<u8> {
    fn decode(&self, data: [u8; 2]) -> Option<Value> {
        let value = decode_u8(data);
        let code = self
            .variants
            .iter()
            .find(|v| v.value == value)
            .map(|v| v.code);
        Some(Value::Enum8(value, code))
    }
}

impl CanParam for EnumParam<u16> {
    fn decode(&self, data: [u8; 2]) -> Option<Value> {
        let value = decode_u16(data);
        let code = self
            .variants
            .iter()
            .find(|v| v.value == value)
            .map(|v| v.code);
        Some(Value::Enum16(value, code))
    }
}

impl CanParam for TimeParam {
    fn decode(&self, data: [u8; 2]) -> Option<Value> {
        Time::new(data[1] / 4, (data[1] % 4) * 15).map(Value::Time)
    }
}

impl CanParam for TimeRangeParam {
    fn decode(&self, data: [u8; 2]) -> Option<Value> {
        if data[0] == 0x80 {
            return None;
        }
        // FIXME better handling if start/end don't parse. At least do some logging or
        // mark the value as errnous.
        let start = Time::new(data[0] / 4, (data[0] % 4) * 15)?;
        let end = Time::new(data[1] / 4, (data[1] % 4) * 15)?;
        Some(Value::TimeRange(TimeRange { start, end }))
    }
}

fn decode_i8(data: [u8; 2]) -> Option<i8> {
    let v = data[0] as i8;
    if v == i8::MIN { None } else { Some(v) }
}

fn decode_i16(data: [u8; 2]) -> Option<i16> {
    let v = ((data[0] as i16) << 8) + (data[1] as i16);
    if v == i16::MIN { None } else { Some(v) }
}

fn decode_u8(data: [u8; 2]) -> u8 {
    data[0]
}

fn decode_u16(data: [u8; 2]) -> u16 {
    ((data[0] as u16) << 8) + (data[1] as u16)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::Catalog;

    #[test]
    fn test_decparam_encode_str() {
        let catalog = Catalog::load().unwrap();
        let param = catalog.param_by_name("max_feed_temp").unwrap();
        assert_eq!(param.encode_str("25.0"), Some([0x00, 0xFA]));
        assert_eq!(param.encode_str("25.5"), Some([0x00, 0xFF]));
        assert_eq!(param.encode_str("25.6"), Some([0x01, 0x00]));
    }
}
