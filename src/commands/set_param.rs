use std::time::Duration;

use clap::Parser;
use tracing::{info, warn};

use crate::can::device::Device;
use crate::can::driver::{BusState, CanDriver};
use crate::can::message::{Message, MessageType};
use crate::can::params::ParamName;
use crate::config::Config;

#[derive(Debug, Parser)]
pub struct Args {
    pub device: Device,
    pub param: ParamName,
    pub value: String,
    #[clap(long)]
    pub sender: Option<Device>,
}

pub async fn cmd(config: Config, args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let can = CanDriver::new(&config.can.interface);
    info!("Waiting for CAN bus to connect...");
    can.wait_for_state(BusState::Connected).await;
    info!("Connected.");

    info!("Get {} / {}...", args.device, args.param.param().name());
    let value = can.get_any(args.device, args.param.param()).await?;
    warn!("Current value: {:?}", value);

    let encoded_value = args.param.param().encode_str(&args.value).unwrap();
    info!(
        "New value (hex encoded): {:02x} {:02x}",
        encoded_value[0], encoded_value[1]
    );

    let sender = args.sender.unwrap_or(Device::TG);

    info!("Setting value...");
    can.send(Message {
        sender,
        receiver: args.device,
        r#type: MessageType::Set,
        param: args.param.id(),
        data: encoded_value,
    })?;

    info!("Sleeping 1s ...");
    tokio::time::sleep(Duration::from_secs(1)).await;

    info!("Get {} / {}...", args.device, args.param.param().name());
    let value = can.get_any(args.device, args.param.param()).await?;
    warn!("Updated value: {:?}", value);

    info!("Waiting for bus driver to shut down...");
    can.shutdown().await;
    info!("Done");

    Ok(())
}
