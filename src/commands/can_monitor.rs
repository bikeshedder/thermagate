use std::{collections::HashMap, sync::Arc};

use crate::can::BusDriver;
use crate::config::Config;
use crate::data::{DEVICES, PARAMETERS};
use crate::model::{Device, Parameter, State};
use crate::utils::read_toml_str;

pub async fn cmd(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let devices: HashMap<String, Device> = read_toml_str(DEVICES)?;
    let parameters: HashMap<String, Parameter> = read_toml_str(PARAMETERS)?;

    let state = Arc::new(State::new(devices, parameters));

    let mut driver = BusDriver::new(&config.can.interface, state);
    while let Some(frame) = driver.recv().await {
        println!("{frame:?}");
    }

    Ok(())
}
