use std::{collections::HashMap, sync::Arc};

use serde::Serialize;
use tokio::sync::{broadcast, Mutex};

use crate::{
    can::{self, BusFrame, Value},
    config::Config,
    data::{DEVICES, PARAMETERS},
    model::{Device, Op, Parameter, State},
    utils::read_toml_str,
    web::run_server,
};

#[derive(Serialize, Clone)]
pub enum Param {
    Loading,
    Value(Option<Value>),
}

#[derive(Clone)]
pub struct Params(pub Arc<Mutex<HashMap<String, HashMap<String, Param>>>>);

pub async fn cmd(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let devices: HashMap<String, Device> = read_toml_str(DEVICES)?;
    let parameters: HashMap<String, Parameter> = read_toml_str(PARAMETERS)?;

    let state = Arc::new(State::new(devices, parameters));

    let mut can = can::BusDriver::new(&config.can.interface, state);

    let (tx, _) = broadcast::channel::<BusFrame>(16);

    let params: Params = Params(Arc::new(Mutex::new(HashMap::new())));

    tokio::spawn({
        let tx = tx.clone();
        let params = params.clone();
        async move {
            while let Some(frame) = can.recv().await {
                match frame.op {
                    Op::Get => {
                        params
                            .0
                            .lock()
                            .await
                            .entry(frame.device.clone())
                            .or_default()
                            .entry(frame.parameter.clone())
                            .or_insert(Param::Loading);
                    }
                    Op::Answer => {
                        params
                            .0
                            .lock()
                            .await
                            .entry(frame.device.clone())
                            .or_default()
                            .insert(frame.parameter.clone(), Param::Value(frame.value.clone()));
                    }
                    Op::Set => {}
                }
                let _ = tx.send(frame);
            }
        }
    });

    run_server(config.http.listen, params, tx).await;

    Ok(())
}
