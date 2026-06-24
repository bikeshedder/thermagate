use internment::Intern;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MultilingualString {
    pub de: Intern<String>,
    pub en: Intern<String>,
}
