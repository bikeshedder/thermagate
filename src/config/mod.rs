use std::{net::SocketAddr, time::Duration};

use figment::{
    providers::{Format, Toml},
    Figment,
};
use nrg_hass::config::HomeAssistantConfig;
use nrg_mqtt::config::MqttConfig;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationSeconds};

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
    pub fn load(config_file: &str) -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Toml::string(DEFAULT_CONFIG))
            .merge(Toml::file(config_file))
            .extract()
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

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryConfig {
    #[serde_as(as = "DurationSeconds")]
    pub interval: Duration,
    pub params: Vec<QueryParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParam {
    pub device: Device,
    pub param: ParamName,
}
