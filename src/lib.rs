use std::time::Duration;

pub mod can;
pub mod commands;
pub mod config;
pub mod hass;
pub mod model;
pub mod utils;
pub mod web;

pub const RECONNECT_DELAY: Duration = Duration::from_secs(5);
