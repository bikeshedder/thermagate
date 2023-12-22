use std::fmt;

use socketcan::{CanDataFrame, CanFrame, CanSocket, Socket};
use strum::EnumMessage;

use crate::can::device::Device;
use crate::can::message::{Message, MessageType};
use crate::can::param::{DecodeParam, Param};
use crate::can::params;
use crate::config::Config;

pub fn get<P: DecodeParam>(socket: &CanSocket, dev: Device, param: &P) -> Option<P::Value> {
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

pub fn get_and_print<P: DecodeParam>(socket: &CanSocket, dev: Device, param: &P) -> Option<P::Value>
where
    P: Param,
    P::Value: fmt::Display,
{
    let value = get(socket, dev, param);
    println!(
        "{} = {} {}",
        param.label().de,
        value.as_ref().map(|v| format!("{}", v)).unwrap_or_default(),
        param.unit().and_then(|e| e.get_message()).unwrap_or("")
    );
    value
}

pub async fn cmd(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let socket = CanSocket::open(&config.can.interface).unwrap();

    get_and_print(&socket, Device::HG1, &params::T_AU);
    get_and_print(&socket, Device::HG1, &params::P);
    let flow = get_and_print(&socket, Device::HG1, &params::V);

    let t_r = get_and_print(&socket, Device::HG1, &params::T_R); // ???
    get_and_print(&socket, Device::HG1, &params::T_TVBH); // ???

    get_and_print(&socket, Device::HG1, &params::T_LIQ);
    let t_v = get_and_print(&socket, Device::HG1, &params::T_V); // ???

    get_and_print(&socket, Device::HG1, &params::T_DHW);

    get_and_print(&socket, Device::HG1, &params::MISCHERSTELLUNG_2_3UVB);
    get_and_print(&socket, Device::HG1, &params::MISCHERSTELLUNG_1);

    //get_and_print(&socket, Device::HG1, &DHW_EHS);
    let delta_t: f32 = (t_v.unwrap() - t_r.unwrap()).try_into().unwrap();

    println!("-------------");
    println!("flow = {} l/h", flow.unwrap());
    println!("delta_t = {:.1} °C", delta_t);
    let kwh_per_k = 1.16f32;
    let kwh = flow.unwrap() as f32 * delta_t * kwh_per_k;
    println!(
        "{} l/h * {:.2} °C * {:.2} kWh/°C = {:.3} kW",
        flow.unwrap(),
        delta_t,
        kwh_per_k,
        kwh
    );

    get_and_print(&socket, Device::HG1, &params::PROGRAMMSCHALTER);
    get_and_print(&socket, Device::HG1, &params::SG);
    get_and_print(&socket, Device::HG1, &params::BETRIEBSART);

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

    get_and_print(&socket, Device::HG1, &params::LEISTUNG_WW);
    get_and_print(&socket, Device::HG1, &params::WW_AKTIV);

    get_and_print(&socket, Device::Outdoor, &params::STATUS_KOMPRESSOR);
    get_and_print(&socket, Device::Outdoor, &params::STATUS_UMWAELZPUMPE);

    Ok(())
}
