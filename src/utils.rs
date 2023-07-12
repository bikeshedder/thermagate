use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
};

use serde::de::DeserializeOwned;

#[derive(Debug, thiserror::Error)]
pub enum ReadTomlError {
    #[error("I/O error")]
    IO(#[from] io::Error),
    #[error("Toml parsing failed")]
    Toml(#[from] toml::de::Error)
}

pub fn read_toml<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T, ReadTomlError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let toml = io::read_to_string(reader)?;
    let data = toml::from_str(&toml)?;
    Ok(data)
}
