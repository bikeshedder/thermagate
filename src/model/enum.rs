use std::fmt::Debug;

use internment::Intern;

use super::string::MultilingualString;

#[derive(Clone, Debug)]
pub struct EnumVariant<T: Clone> {
    pub value: T,
    pub code: Intern<String>,
    pub label: MultilingualString,
}
