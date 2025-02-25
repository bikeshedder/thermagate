use std::fmt::Debug;

use super::string::MultilingualStr;

pub trait Enum: Into<&'static str> + From<i16> + Sync + Debug {
    const VARIANTS: &'static [EnumVariant];
}

pub struct EnumVariant {
    pub value: i16,
    pub code: &'static str,
    pub label: MultilingualStr,
}
