use std::collections::HashMap;
use std::fmt;

use socketcan::{CanDataFrame, CanFrame, CanSocket, Socket};
use strum::EnumMessage;

use crate::can::device::Device;
use crate::can::message::{Message, MessageType};
use crate::can::param::Param;
use crate::can::params;
use crate::config::Config;
use crate::data::PARAMETERS;
use crate::old_model::{Parameter, ParameterType};
use crate::utils::read_toml_str;

pub fn get<P: Param>(socket: &CanSocket, dev: Device, param: &P) -> P::Value {
    let req = Message {
        sender: Device::GW,
        receiver: dev,
        param: param.id(),
        r#type: MessageType::Request,
        data: [0, 0],
    };
    socket.write_frame(&CanDataFrame::from(req)).unwrap();
    // wait for response
    while let Ok(frame) = socket.read_frame() {
        let CanFrame::Data(data_frame) = frame else {
            continue;
        };
        let Ok(msg) = Message::try_from(&data_frame) else {
            continue;
        };
        if msg.receiver != Device::GW {
            /*
            println!(
                "{:02x?} {:20} {:02x?} {}",
                msg, param.name, msg.data, param.unit
            );
            */
            continue;
        }
        if msg.sender != req.receiver {
            continue;
        }
        if msg.param == req.param {
            return param.decode(msg.data);
        }
    }
    unreachable!()
}

struct BetterParam {
    name: String,
    r#type: ParameterType,
}

pub fn get_and_print<P: Param>(socket: &CanSocket, dev: Device, param: &P) -> P::Value
where
    P: Param,
    P::Value: fmt::Display,
{
    let value = get(socket, dev, param);
    println!(
        "{} = {} {}",
        param.name(),
        value,
        param.unit().and_then(|e| e.get_message()).unwrap_or("")
    );
    value
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

    get_and_print(&socket, Device::HG1, &params::T_AU);
    get_and_print(&socket, Device::HG1, &params::P);
    let flow = get_and_print(&socket, Device::HG1, &params::V);

    let t_r = get_and_print(&socket, Device::HG1, &params::T_R); // ???
    let t_v_bh = get_and_print(&socket, Device::HG1, &params::T_V_BH); // ???

    get_and_print(&socket, Device::HG1, &params::T_LIQ);
    let t_v = get_and_print(&socket, Device::HG1, &params::T_V); // ???

    get_and_print(&socket, Device::HG1, &params::T_DHW);

    get_and_print(&socket, Device::HG1, &params::B1);
    get_and_print(&socket, Device::HG1, &params::MISCHERSTELLUNG_1);

    //get_and_print(&socket, Device::HG1, &DHW_EHS);
    let delta_t: f32 = (t_v - t_r).try_into().unwrap();

    println!("-------------");
    println!("flow = {} l/h", flow);
    println!("delta_t = {:.1} °C", delta_t);
    let kwh_per_c = 1.16f32;
    let kwh = flow as f32 * delta_t * kwh_per_c;
    println!(
        "{} l/h * {:.2} °C * {:.2} kWh/°C = {:.3} kW",
        flow, delta_t, kwh_per_c, kwh
    );

    get_and_print(&socket, Device::HG1, &params::PROGRAMMSCHALTER);
    get_and_print(&socket, Device::HG1, &params::SG);
    get_and_print(&socket, Device::HG1, &params::DEFROST_AKTIV);

    /*
    get_and_print(&socket, Device::HCM1, &AUSSENTEMP);
    get_and_print(&socket, Device::HC1, &AUSSENTEMP_WAERMEPUMPE);
    get_and_print(&socket, Device::HCM1, &VORLAUF);

    get_and_print(&socket, Device::HG1, &ENERGIE_WP_KUEHLUNG);
    get_and_print(&socket, Device::HG1, &ENERGIE_WP_HEIZUNG);
    get_and_print(&socket, Device::HG1, &WW_POWER);
    get_and_print(&socket, Device::HG1, &ENERGIE_EXT_QUELLE_WARMWASSER);
    get_and_print(&socket, Device::HG1, &ENERGIE_EXT_QUELLE_HEIZUNG);
    get_and_print(&socket, Device::HG1, &ENERGIE_WARMWASSER);
    get_and_print(&socket, Device::HG1, &ENERGIE_WP_GESAMT);
    get_and_print(&socket, Device::HG1, &LAUFZEIT_KOMPRESSOR);
    get_and_print(&socket, Device::HG1, &LAUFZEIT_PUMPE);
    */

    get_and_print(&socket, Device::Outdoor, &params::STATUS_KOMPRESSOR);
    get_and_print(&socket, Device::Outdoor, &params::STATUS_UMWAELZPUMPE);

    Ok(())
}
