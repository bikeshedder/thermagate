use std::fmt::Debug;

use super::string::MultilingualString;

#[derive(Clone, Debug)]
pub struct EnumVariant<T: Clone> {
    pub value: T,
    pub code: String,
    pub label: MultilingualString,
}
