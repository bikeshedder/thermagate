use std::time::Duration;

use rumqttc::{AsyncClient, MqttOptions};

use crate::{
    config::Config,
    hass::{DeviceClass, Sensor},
};

pub async fn cmd(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut mqttoptions =
        MqttOptions::new(config.mqtt.device_id, config.mqtt.host, config.mqtt.port);
    mqttoptions
        .set_credentials(config.mqtt.username, config.mqtt.password)
        .set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    //client.subscribe("hello/rumqtt", QoS::AtMostOnce).await?;

    let sensor = Sensor {
        name: "Altherma HCM2.cAUSSENTEMP".into(),
        state_topic: "altherma-gateway/HCM2/cAUSSENTEMP".into(),
        value_template: Some("{{ (value | float) }}".into()),
        unit_of_measurement: Some(crate::hass::UnitOfMeasurement::TempCelsius),
        device_class: Some(DeviceClass::Temperature),
        ..Default::default()
    };

    sensor
        .register(&client, &config.hass.mqtt_discovery_prefix, &config.hass.device_prefix)
        .await?;
    sensor.value(&client, "30.8").await?;

    while let Ok(notification) = eventloop.poll().await {
        println!("Received = {:?}", notification);
    }

    Ok(())
}
