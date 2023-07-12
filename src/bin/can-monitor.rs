use std::{collections::HashMap, sync::Arc};

use altherma_gateway::{can::BusDriver, utils::read_toml};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use altherma_gateway::model::{Device, Parameter, State};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let devices: HashMap<String, Device> = read_toml("data/devices.toml")?;
    let parameters: HashMap<String, Parameter> = read_toml("data/parameters.toml")?;

    let state = Arc::new(State::new(devices, parameters));

    let mut driver = BusDriver::new("can0", state);
    while let Some(frame) = driver.recv().await {
        println!("{frame:?}");
    }

    Ok(())
}
