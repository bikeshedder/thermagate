use std::collections::HashMap;

use socketcan::{CanFrame, CanSocket, EmbeddedFrame, Frame, Socket};

use crate::can2::message::{Message, MessageType};
use crate::config::Config;
use crate::data::PARAMETERS;
use crate::model::{Parameter, ParameterType};
use crate::utils::read_toml_str;

struct BetterParam {
    name: String,
    r#type: ParameterType,
}

pub async fn cmd(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let socket = CanSocket::open(&config.can.interface).unwrap();

    let parameters: HashMap<String, Parameter> = read_toml_str(PARAMETERS)?;
    let param_by_id: HashMap<u16, BetterParam> = parameters
        .into_iter()
        .map(|(name, p)| {
            (
                p.info_number,
                BetterParam {
                    name,
                    r#type: p.r#type,
                },
            )
        })
        .collect();

    while let Ok(frame) = socket.read_frame() {
        let CanFrame::Data(data_frame) = frame else {
            continue;
        };
        let data = data_frame.data();
        let msg = Message::try_from(&data_frame);
        match msg {
            Ok(msg) => {
                if let Some(p) = param_by_id.get(&msg.param) {
                    if msg.r#type == MessageType::Response || msg.r#type == MessageType::Set {
                        println!(
                            "{:03x} {:02x?} {} {} {}",
                            frame.raw_id(),
                            data,
                            msg,
                            p.name,
                            p.r#type.as_ref()
                        );
                    } else {
                        println!("{:03x} {:02x?} {} {}", frame.raw_id(), data, msg, p.name);
                    }
                } else {
                    println!("{:03x} {:02x?} {}", frame.raw_id(), data, msg);
                }
            }
            Err(e) => println!("{:02x?} Error: {}", data, e),
        }
    }

    Ok(())
}
