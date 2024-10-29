use super::param::MultilingualStr;

pub trait Enum: Into<&'static str> + From<i16> + Sync {
    const VARIANTS: &'static [EnumVariant];
}

pub struct EnumVariant {
    pub value: i16,
    pub code: &'static str,
    pub label: MultilingualStr,
}
