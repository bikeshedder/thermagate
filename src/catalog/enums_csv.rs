use std::{collections::HashMap, io};

use internment::Intern;
use serde::Deserialize;

use crate::model::{r#enum::EnumVariant, string::MultilingualString};

use super::error::CatalogError;

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    pub name: String,
    pub code: String,
    pub value: u16,
    pub label_en: String,
    pub label_de: String,
}

impl Into<EnumVariant<u16>> for Row {
    fn into(self) -> EnumVariant<u16> {
        EnumVariant {
            code: self.code.into(),
            value: self.value,
            label: MultilingualString {
                de: Intern::new(self.label_de),
                en: Intern::new(self.label_en),
            },
        }
    }
}

impl TryInto<EnumVariant<u8>> for Row {
    type Error = CatalogError;
    fn try_into(self) -> Result<EnumVariant<u8>, Self::Error> {
        Ok(EnumVariant {
            code: self.code.into(),
            value: self
                .value
                .try_into()
                .map_err(|_| CatalogError::InvalidEnum8Value(self.value))?,
            label: MultilingualString {
                de: Intern::new(self.label_de),
                en: Intern::new(self.label_en),
            },
        })
    }
}

#[derive(Default)]
pub struct EnumsCsv {
    pub enums: HashMap<String, Vec<Row>>,
}

pub fn read_enums(reader: impl io::Read) -> Result<EnumsCsv, CatalogError> {
    let mut csv = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);
    let rows = csv.deserialize::<Row>();
    let mut enums_csv = EnumsCsv::default();
    for row_result in rows {
        let row = row_result?;
        enums_csv
            .enums
            .entry(row.name.clone())
            .or_insert_with(Vec::new)
            .push(row);
    }
    Ok(enums_csv)
}
