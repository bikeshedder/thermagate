use console::style;
use itertools::Itertools;
use socketcan::EmbeddedFrame;

use crate::can::device::Device;
use crate::can::driver::CanDriver;
use crate::can::message::MessageType;
use crate::can::param::CanParam;
use crate::catalog::Catalog;
use crate::config::Config;

pub async fn cmd(config: Config, catalog: Catalog) -> Result<(), Box<dyn std::error::Error>> {
    let driver = CanDriver::new(&config.can.interface);
    let mut rx = driver.rx();

    let req = style("REQ").cyan();
    let rsp = style("RSP").green();
    let set = style("SET").red();
    let ping = style("PING").yellow();
    let pong = style("PONG").yellow();

    loop {
        let msg = rx.recv().await?;
        if msg.sender == Device::G1 || msg.receiver == Device::G1 {
            // filter messages from Rocon G1
            continue;
        }
        print!("{:03x} ", u16::from(msg.sender));
        let data = msg
            .frame
            .data()
            .iter()
            .map(|b| format!("{:02x}", b))
            .join(" ");
        print!("{} ", style(data).dim());
        let sender = style(format!(" {:7} ", msg.sender.to_string()))
            .on_color256(233)
            .white();
        let receiver = style(format!(" {:7} ", msg.receiver.to_string()))
            .on_color256(233)
            .white();
        let param = msg.param;
        match msg.r#type {
            MessageType::Set => {
                print!("{sender} -> {receiver} {set} {param:04x} ")
            }
            MessageType::Request => {
                print!("{sender} -> {receiver} {req} {param:04x} ")
            }
            MessageType::Response => {
                print!("{receiver} <- {sender} {rsp} {param:04x} ")
            }
            MessageType::Ping => {
                print!("{sender} -> {receiver} {ping} {param:04x} ")
            }
            MessageType::Pong => {
                print!("{receiver} <- {sender} {pong} {param:04x} ")
            }
            MessageType::Unknown(x) => {
                print!("{sender} -> {receiver} UNKNOWN({x})")
            }
        }
        if msg.r#type == MessageType::Request
            || msg.r#type == MessageType::Response
            || msg.r#type == MessageType::Set
        {
            if let Some(p) = catalog.param_by_id(msg.param) {
                print!("{} ", style(&p.name).magenta());
                if msg.r#type == MessageType::Response || msg.r#type == MessageType::Set {
                    let decoded_data = p.decode(msg.data);
                    if let Some(decoded_data) = decoded_data {
                        print!("= {} ", style(decoded_data).bold());
                    } else {
                        print!("= {} ", style("None").italic());
                    }
                }
            } else if msg.r#type == MessageType::Response || msg.r#type == MessageType::Set {
                print!(
                    "= {}",
                    msg.data.iter().map(|b| format!("{:02x}", b)).join("")
                );
            }
        }
        println!();
    }
}
