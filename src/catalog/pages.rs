use std::io;

use serde::{Deserialize, Deserializer, Serialize, de};

use super::error::CatalogError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Page {
    pub path: String,
    #[serde(default = "Vec::new")]
    pub params: Vec<PageParam>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PageParam {
    pub device: String,
    pub param: String,
}

impl<'de> Deserialize<'de> for PageParam {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.splitn(2, '.').collect();

        if parts.len() != 2 {
            return Err(de::Error::custom("Invalid format, expected 'device.param'"));
        }

        Ok(PageParam {
            device: parts[0].to_string(),
            param: parts[1].to_string(),
        })
    }
}

pub fn read_pages(reader: impl io::Read) -> Result<Vec<Page>, CatalogError> {
    let pages: Vec<Page> =
        serde_json::from_reader(reader).map_err(CatalogError::InvalidPagesJson)?;
    Ok(pages)
}
