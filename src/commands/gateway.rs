use std::{collections::HashMap, sync::Arc};

use crate::{web::run_server, can, model::{Device, Parameter, State}, utils::read_toml};

pub async fn cmd() -> Result<(), Box<dyn std::error::Error>> {
    let devices: HashMap<String, Device> = read_toml("data/devices.toml")?;
    let parameters: HashMap<String, Parameter> = read_toml("data/parameters.toml")?;

    let state = Arc::new(State::new(devices, parameters));

    let mut can = can::BusDriver::new("can0", state);

    tokio::spawn(async move {
        while let Some(frame) = can.recv().await {
            println!("{frame:?}");
        }
    });

    run_server().await;

    Ok(())
}
