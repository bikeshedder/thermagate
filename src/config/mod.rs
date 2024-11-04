use std::{net::SocketAddr, time::Duration};

use nrg_hass::config::HomeAssistantConfig;
use nrg_mqtt::config::MqttConfig;
use serde::{Deserialize, Serialize};

use crate::can::{device::Device, params::ParamName};

pub const DEFAULT_CONFIG: &str = include_str!("default.toml");

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub language: Language,
    pub http: HttpConfig,
    pub can: CanConfig,
    pub mqtt: MqttConfig,
    pub hass: HomeAssistantConfig,
    pub query: QueryConfig,
}

impl Config {
    pub fn load(config_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(config::Config::builder()
            .add_source(config::File::from_str(
                DEFAULT_CONFIG,
                config::FileFormat::Toml,
            ))
            .add_source(config::File::new(config_file, config::FileFormat::Toml))
            .build()?
            .try_deserialize::<Self>()?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    DE,
    EN,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpConfig {
    pub listen: SocketAddr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanConfig {
    pub interface: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryConfig {
    pub interval: Duration,
    pub params: Vec<QueryParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParam {
    pub device: Device,
    pub param: ParamName,
}
