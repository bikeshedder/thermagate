use thiserror::Error;

#[derive(Error, Debug)]
pub enum CatalogError {
    #[error("CSV error: {0}")]
    CSV(#[from] csv::Error),

    #[error("Unknown enum: {0}")]
    UnknownEnum(String),

    #[error("Unknown enum variant: {0}")]
    UnknownEnumVariant(String),

    #[error("Invalid value for 8 bit enum: {0}")]
    InvalidEnum8Value(u16),

    #[error("Invalid pages.json: {0}")]
    InvalidPagesJson(serde_json::Error),

    #[error("Unknown device: {0}")]
    UnknownDevice(String),

    #[error("Unknown parameter: {0}")]
    UnknownParameter(String),
}
