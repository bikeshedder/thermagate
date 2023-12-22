use std::{collections::HashMap, sync::Arc};

use serde::Serialize;
use tokio::sync::{broadcast, Mutex};

use crate::{
    can::{
        driver::{CanDriver, ReceivedMessage},
        message::MessageType,
        param::AnyValue,
    },
    config::Config,
    web::run_server,
};

#[derive(Serialize, Clone)]
pub enum Param {
    Loading,
    Value(Option<AnyValue>),
}

#[derive(Clone)]
pub struct Params(pub Arc<Mutex<HashMap<String, HashMap<&'static str, Param>>>>);

pub async fn cmd(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let can = CanDriver::new(&config.can.interface);
    let params: Params = Params(Arc::new(Mutex::new(HashMap::new())));
    tokio::spawn(recv_params(params.clone(), can.rx()));
    run_server(config.http.listen, params, can).await;
    Ok(())
}

async fn recv_params(params: Params, mut rx: broadcast::Receiver<ReceivedMessage>) {
    while let Ok(message) = rx.recv().await {
        let Some((param, value)) = message.decode_param() else {
            continue;
        };
        match message.r#type {
            MessageType::Request => {
                params
                    .0
                    .lock()
                    .await
                    .entry(message.receiver.to_string())
                    .or_default()
                    .entry(param.name())
                    .or_insert(Param::Loading);
            }
            MessageType::Response => {
                params
                    .0
                    .lock()
                    .await
                    .entry(message.sender.to_string())
                    .or_default()
                    .insert(param.name(), Param::Value(value));
            }
            // Ignore other message types
            _ => {}
        }
    }
}
