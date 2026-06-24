use std::fmt::Debug;

use internment::Intern;
use serde::Serialize;

use super::string::MultilingualString;

#[derive(Clone, Debug, Serialize)]
pub struct EnumVariant<T: Clone> {
    pub value: T,
    pub code: Intern<String>,
    pub label: MultilingualString,
}
