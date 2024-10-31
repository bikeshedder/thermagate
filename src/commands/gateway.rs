use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

use nrg_hass::{config::HomeAssistantConfig, discovery::announce, state::publish_state};
use nrg_mqtt::config::MqttConfig;
use num_traits::ToPrimitive;
use rumqttc::AsyncClient;
use serde::Serialize;
use serde_json::json;
use tokio::sync::{broadcast, Mutex};
use tracing::debug;

use crate::{
    can::{
        driver::{CanDriver, ReceivedMessage},
        message::MessageType,
        param::{AnyValue, Unit},
    },
    config::Config,
    hass::make_hass_sensor,
    utils::warn_if_err,
    web::{run_server, ParamUpdate},
};

#[derive(Debug, Clone, Serialize)]
pub enum Param {
    Loading,
    Value(Option<AnyValue>),
}

#[derive(Clone)]
pub struct Params {
    pub values: Arc<Mutex<HashMap<String, HashMap<&'static str, Param>>>>,
    pub tx: broadcast::Sender<Arc<ParamUpdate>>,
}

pub async fn cmd(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let can = Arc::new(CanDriver::new(&config.can.interface));
    let (params_tx, _) = broadcast::channel(100); // FIXME size?
    let params: Params = Params {
        values: Arc::new(Mutex::new(HashMap::new())),
        tx: params_tx,
    };
    let (mqtt, mut mqtt_event_loop) = config.mqtt.client();
    tokio::spawn(recv_params(
        params.clone(),
        can.rx(),
        mqtt,
        config.hass.clone(),
        config.mqtt.clone(),
    ));
    tokio::spawn(async move {
        while let Ok(ev) = mqtt_event_loop.poll().await {
            debug!("{:?}", ev);
        }
    });
    tokio::spawn(query_params(can.clone()));
    run_server(&config.http, params, can).await;
    Ok(())
}

async fn query_params(_can: Arc<CanDriver>) {
    // This function is supposed to query parameters in periodic intervals.
    // As of now the gateway just listens to the CAN bus and provides access
    // to all the variables read by RoCon G1.
}

async fn recv_params(
    params: Params,
    mut rx: broadcast::Receiver<ReceivedMessage>,
    mqtt: AsyncClient,
    hass_cfg: HomeAssistantConfig,
    mqtt_cfg: MqttConfig,
) {
    while let Ok(message) = rx.recv().await {
        let Some((param, value)) = message.decode_param() else {
            continue;
        };
        if message.sender.is_other() {
            continue;
        }
        let mqtt_value = if let Some(value) = &value {
            match (&value, param.unit()) {
                (AnyValue::Bool(b), _) => json!(b),
                (AnyValue::I8(v), _) => json!(v),
                (AnyValue::I16(v), _) => json!(v),
                (AnyValue::Dec(v), Some(Unit::LitersPerHour)) => {
                    // Convert liters per hour to cubic meters per hour
                    json!(v.to_f64().map(|v| v / 1000f64))
                }
                (AnyValue::Dec(v), _) => json!(v.to_f64()),
                (AnyValue::Enum8(_, n), _) => json!(n),
                (AnyValue::Enum16(_, n), _) => json!(n),
                (AnyValue::TimeRange(v), _) => json!(v.to_string()),
                (AnyValue::Time(v), _) => json!(v.to_string()),
            }
        } else {
            json!(null)
        };
        match message.r#type {
            MessageType::Request => {
                if let Entry::Vacant(e) = params
                    .values
                    .lock()
                    .await
                    .entry(message.receiver.to_string())
                    .or_default()
                    .entry(param.name())
                {
                    e.insert(Param::Loading);
                    let sensor = make_hass_sensor(
                        message.receiver,
                        param,
                        &hass_cfg.object_id,
                        &mqtt_cfg.topic_prefix,
                    );
                    warn_if_err(
                        announce(&mqtt, &hass_cfg, &hass_cfg.object_id, &sensor).await,
                        "Failed to announce device to MQTT broker",
                    )
                }
            }
            MessageType::Response => {
                params
                    .values
                    .lock()
                    .await
                    .entry(message.sender.to_string())
                    .or_default()
                    .insert(param.name(), Param::Value(value.clone()));
                let sensor = make_hass_sensor(
                    message.sender,
                    param,
                    &hass_cfg.object_id,
                    &mqtt_cfg.topic_prefix,
                );
                warn_if_err(
                    publish_state(&mqtt, &sensor, mqtt_value).await,
                    "Failed to publish state to MQTT broker",
                );
                let _ = params.tx.send(Arc::new(ParamUpdate {
                    device: message.sender.into(),
                    param: param.into(),
                    value: Param::Value(value.clone()),
                }));
            }
            // Ignore other message types
            _ => {}
        }
    }
}
