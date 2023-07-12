use altherma_gateway::hass::{DeviceClass, Sensor};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::error::Error;
use std::time::Duration;
use tokio::{task, time};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mqttoptions = MqttOptions::new("altherma-gateway", "homeassistant.local", 1883);
    mqttoptions
        .set_credentials("altherma", "ahHu1oi3Riiviex1")
        .set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    //client.subscribe("hello/rumqtt", QoS::AtMostOnce).await?;

    let hass_discovery_prefix = "homeassistant";
    let device_prefix = "altherma-gateway";

    let sensor = Sensor {
        name: "Altherma HCM2.cAUSSENTEMP".into(),
        state_topic: "altherma-gateway/HCM2/cAUSSENTEMP".into(),
        value_template: Some("{{ (value | float) }}".into()),
        unit_of_measurement: Some(altherma_gateway::hass::UnitOfMeasurement::TempCelsius),
        device_class: Some(DeviceClass::Temperature),
        ..Default::default()
    };

    sensor
        .register(&client, hass_discovery_prefix, device_prefix)
        .await?;
    sensor.value(&client, "30.8").await?;

    while let Ok(notification) = eventloop.poll().await {
        println!("Received = {:?}", notification);
    }

    Ok(())
}
