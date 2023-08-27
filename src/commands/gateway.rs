use std::{collections::HashMap, sync::Arc};

use tokio::sync::broadcast;

use crate::{
    can::{self, BusFrame},
    config::Config,
    data::{DEVICES, PARAMETERS},
    model::{Device, Parameter, State},
    utils::read_toml_str,
    web::run_server,
};

pub async fn cmd(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let devices: HashMap<String, Device> = read_toml_str(DEVICES)?;
    let parameters: HashMap<String, Parameter> = read_toml_str(PARAMETERS)?;

    let state = Arc::new(State::new(devices, parameters));

    let mut can = can::BusDriver::new(&config.can.interface, state);

    let (tx, _) = broadcast::channel::<BusFrame>(16);

    tokio::spawn({
        let tx = tx.clone();
        async move {
            while let Some(frame) = can.recv().await {
                let _ = tx.send(frame);
            }
        }
    });

    run_server(config.http.listen, tx).await;

    Ok(())
}
