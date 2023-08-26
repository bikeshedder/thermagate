use std::{collections::HashMap, sync::Arc};

use crate::config::Config;
use crate::{can::BusDriver, utils::read_toml};
use crate::model::{Device, Parameter, State};

pub async fn cmd(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let devices: HashMap<String, Device> = read_toml("data/devices.toml")?;
    let parameters: HashMap<String, Parameter> = read_toml("data/parameters.toml")?;

    let state = Arc::new(State::new(devices, parameters));

    let mut driver = BusDriver::new(&config.can.interface, state);
    while let Some(frame) = driver.recv().await {
        println!("{frame:?}");
    }

    Ok(())
}
