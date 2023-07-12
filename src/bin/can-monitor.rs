use std::collections::HashMap;

use altherma_gateway::utils::read_toml;
use futures_util::stream::StreamExt;
use socketcan::{tokio::CanSocket, EmbeddedFrame, Frame};

use altherma_gateway::model::{
    Address, Device, FloatParameter, Parameter, ParameterType, State,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let devices: HashMap<String, Device> = read_toml("data/devices.toml")?;
    let parameters: HashMap<String, Parameter> = read_toml("data/parameters.toml")?;

    let state = State::new(devices, parameters);

    let mut socket_rx = CanSocket::open("can0")?;
    //let socket_tx = CanSocket::open("vcan0")?;
    while let Some(Ok(frame)) = socket_rx.next().await {
        if let socketcan::CanFrame::Data(data_frame) = frame {
            match data_frame.data() {
                [a0, a1, a2, p0, p1, v0, v1] => {
                    let a = Address(frame.id_word(), *a0, *a1, *a2);
                    let Some((d_name, op)) = state.device_by_address.get(&a) else {
                        println!("Unsupported address: {:?}", a);
                        continue;
                    };
                    let p = ((*p0 as u16) << 8) + (*p1 as u16);
                    let Some(p_name) = state.parameter_by_address.get(&p) else {
                        println!("Unknown parameter: {}", p);
                        continue;
                    };
                    let v = ((*v0 as u16) << 8) + (*v1 as u16);
                    let v = v as i16;
                    let param = &state.parameters[p_name];
                    match param.r#type {
                        ParameterType::Float(FloatParameter { factor, .. }) => {
                            let value = if v == i16::MIN {
                                None
                            } else {
                                Some(v as f32 / factor)
                            };
                            println!(
                                "{:?} {}.{} = {}",
                                op,
                                d_name,
                                p_name,
                                value.map(|v| v.to_string()).unwrap_or("null".into())
                            );
                        }
                        _ => {}
                    }
                }
                x => {
                    println!("{} {:?}", frame.id_word(), x);
                }
            }
        }
        //socket_tx.write_frame(frame)?.await?;
    }
    Ok(())
}
