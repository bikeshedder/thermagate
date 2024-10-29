use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::serial::SerialConfig;

pub const DEFAULT_CONFIG: &str = include_str!("default.toml");

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub http: HttpConfig,
    pub can: CanConfig,
    pub mqtt: MqttConfig,
    pub hass: HassConfig,
    pub serial: SerialConfig,
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
pub struct HttpConfig {
    pub listen: SocketAddr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanConfig {
    pub interface: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MqttConfig {
    pub device_id: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HassConfig {
    pub mqtt_discovery_prefix: String,
    pub device_prefix: String,
}
