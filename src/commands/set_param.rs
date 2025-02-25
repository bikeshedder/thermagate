use std::time::Duration;

use clap::Parser;
use tracing::{info, warn};

use crate::can::device::Device;
use crate::can::driver::{BusState, CanDriver};
use crate::can::message::{Message, MessageType};
use crate::can::param::CanParam;
use crate::catalog::Catalog;
use crate::config::Config;

#[derive(Debug, Parser)]
pub struct Args {
    pub device: Device,
    pub param: String,
    pub value: String,
    #[clap(long)]
    pub sender: Option<Device>,
}

pub async fn cmd(
    config: Config,
    catalog: Catalog,
    args: Args,
) -> Result<(), Box<dyn std::error::Error>> {
    let param = catalog
        .param_by_name(&args.param)
        // FIXME add proper error handling
        .expect("Unknown parameter");
    let can = CanDriver::new(&config.can.interface);
    info!("Waiting for CAN bus to connect...");
    can.wait_for_state(BusState::Connected).await;
    info!("Connected.");

    info!("Get {} / {}...", args.device, param.name);
    let value = can.get(args.device, param).await?;
    warn!("Current value: {:?}", value);

    let encoded_value = param.encode_str(&args.value).unwrap();
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
        param: param.id,
        data: encoded_value,
    })?;

    info!("Sleeping 1s ...");
    tokio::time::sleep(Duration::from_secs(1)).await;

    info!("Get {} / {}...", args.device, param.name);
    let value = can.get(args.device, param).await?;
    warn!("Updated value: {:?}", value);

    info!("Waiting for bus driver to shut down...");
    can.shutdown().await;
    info!("Done");

    Ok(())
}
