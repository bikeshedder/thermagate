//! This module is responsible for loading the csv files containing the
//! all sorts of things which are loaded at application startup but never
//! change. e.g. parameters, enums, pages, etc.

use enums_csv::read_enums;
use error::CatalogError;
use param::Param;
use params_csv::read_params;
use rust_embed::Embed;

pub mod enums_csv;
pub mod error;
pub mod param;
pub mod params_csv;

#[derive(Embed)]
#[folder = "data"]
pub struct Data;

pub struct Catalog {
    pub params: Vec<Param>,
}

impl Catalog {
    pub fn load() -> Result<Self, CatalogError> {
        // FIXME add support for loading catalog from disk
        let enums = read_enums(
            &*Data::get("enums.csv")
                .expect("File not found data/enums.csv")
                .data,
        )?;
        let params = read_params(
            &*Data::get("params.csv")
                .expect("File not found: data/params.csv")
                .data,
            enums,
        )?;
        Ok(Self { params })
    }
    pub fn param_by_id(&self, id: u16) -> Option<&Param> {
        // TODO replace this by a HashMap lookup
        self.params.iter().find(|p| p.id == id)
    }
    pub fn param_by_name(&self, name: &str) -> Option<&Param> {
        // TODO replace this by a HashMap lookup
        self.params.iter().find(|p| &*p.name == name)
    }
}
