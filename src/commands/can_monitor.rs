use console::style;
use itertools::Itertools;
use socketcan::{CanFrame, CanSocket, EmbeddedFrame, Frame, Socket};

use crate::can::device::Device;
use crate::can::message::{Message, MessageType};
use crate::can::params::PARAMS;
use crate::config::Config;

pub async fn cmd(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let socket = CanSocket::open(&config.can.interface).unwrap();

    let req = style("REQ").cyan();
    let rsp = style("RSP").green();
    let set = style("SET").red();
    let ping = style("PING").yellow();
    let pong = style("PONG").yellow();

    while let Ok(frame) = socket.read_frame() {
        let CanFrame::Data(data_frame) = frame else {
            continue;
        };
        let data = data_frame.data();
        let msg = Message::try_from(&data_frame);
        if msg
            .as_ref()
            .is_ok_and(|m| m.sender == Device::G1 || m.receiver == Device::G1)
        {
            // filter messages from Rocon G1
            continue;
        }
        print!("{:03x} ", frame.raw_id());
        let data = data.iter().map(|b| format!("{:02x}", b)).join(" ");
        print!("{} ", style(data).dim());
        match msg {
            Ok(msg) => {
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
                    if let Some(p) = PARAMS.get(&msg.param) {
                        print!("{} ", style(&p.name()).magenta());
                        if msg.r#type == MessageType::Response || msg.r#type == MessageType::Set {
                            let decoded_data = p.decode_any(msg.data);
                            if let Some(decoded_data) = decoded_data {
                                print!("= {} ", style(decoded_data).bold());
                            } else {
                                print!("= {} ", style("None").italic());
                            }
                        }
                    } else if msg.r#type == MessageType::Response || msg.r#type == MessageType::Set
                    {
                        print!(
                            "= {}",
                            msg.data.iter().map(|b| format!("{:02x}", b)).join("")
                        );
                    }
                }
                println!();
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(())
}
