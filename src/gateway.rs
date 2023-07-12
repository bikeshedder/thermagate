use std::{collections::HashMap, sync::Arc};

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::{web::run_server, can, model::{Device, Parameter, State}, utils::read_toml};

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

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
