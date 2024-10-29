use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
};

use serde::de::DeserializeOwned;
use tracing::warn;

#[derive(Debug, thiserror::Error)]
pub enum ReadTomlError {
    #[error("I/O error")]
    IO(#[from] io::Error),
    #[error("Toml parsing failed")]
    Toml(#[from] toml::de::Error),
}

pub fn read_toml<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T, ReadTomlError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let toml = io::read_to_string(reader)?;
    read_toml_str(&toml)
}

pub fn read_toml_str<T: DeserializeOwned>(toml: &str) -> Result<T, ReadTomlError> {
    let data = toml::from_str(toml)?;
    Ok(data)
}

pub fn warn_if_err<E: std::error::Error>(result: Result<(), E>, error_message: &str) {
    if let Err(error) = result {
        warn!("{}: {}", error_message, error);
    }
}
