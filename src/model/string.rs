use internment::Intern;

#[derive(Debug, Clone)]
pub struct MultilingualString {
    pub de: Intern<String>,
    pub en: Intern<String>,
}
