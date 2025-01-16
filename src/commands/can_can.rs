use std::fmt;
use std::time::Duration;

use socketcan::{CanDataFrame, CanFrame, CanSocket, Socket};
use strum::EnumMessage;

use crate::can::device::Device;
use crate::can::message::{Message, MessageType};
use crate::can::param::{DecodeParam, Param};
use crate::can::params;
use crate::config::Config;

pub fn get<P: DecodeParam>(socket: &CanSocket, dev: Device, param: &P) -> Option<P::Value> {
    let req = Message {
        sender: Device::TG,
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
        if msg.receiver != Device::TG {
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
    loop {
        fetch_params(&socket).await;
        tokio::time::sleep(Duration::from_secs(15)).await;
    }
}

async fn fetch_params(socket: &CanSocket) {
    get(&socket, Device::HG1, &params::OUTSIDE_TEMP);
    get(&socket, Device::Outdoor, &params::STATUS_COMPRESSOR);
    get(&socket, Device::HG1, &params::OPERATING_MODE);
    get(&socket, Device::HC2, &params::OPERATING_MODE);
    let flow = get(&socket, Device::HG1, &params::V);
    let t_r = get(&socket, Device::HG1, &params::T_R);
    get(&socket, Device::HG1, &params::T_DHW);
    get(&socket, Device::HG1, &params::T_AU);
    get(&socket, Device::HG1, &params::MODE);
    get(&socket, Device::HG1, &params::EXTERNAL_REQUEST);
    get(&socket, Device::HG1, &params::ROOM_THERMOSTAT_INTERLINK);
    get(&socket, Device::HG1, &params::QUIET_MODE);
    get(&socket, Device::HG1, &params::STATUS_HEATING_SUPPORT);
    get(
        &socket,
        Device::Outdoor,
        &params::STATUS_HEAT_CIRCULATION_PUMP,
    );
    get(&socket, Device::Outdoor, &params::PWM_PUMP);
    get(&socket, Device::HG1, &params::BUH_CURRENT_OUTPUT);
    get(&socket, Device::HG1, &params::MIX_3UVDHW);
    get(&socket, Device::HG1, &params::MIX_3UVB1);
    get(&socket, Device::HG1, &params::FEED_TEMPERATURE_CURRENT);
    get(&socket, Device::HG1, &params::FEED_TEMPERATURE_TARGET);
    get(&socket, Device::HC1, &params::AVERAGE_OUTSIDE_TEMP);
    get(&socket, Device::HC2, &params::AVERAGE_OUTSIDE_TEMP);
    get(&socket, Device::HG1, &params::HOT_WATER_TEMP_CURRENT);
    get(&socket, Device::HG1, &params::HOT_WATER_TEMP_TARGET);
    get(&socket, Device::HG1, &params::RETURN_FLOW_TEMP);
    get(&socket, Device::HCM2, &params::FEED_TEMP_HC_CURRENT);
    get(&socket, Device::HC1, &params::FEED_TEMP_HC_TARGET);
    let t_v = get(&socket, Device::HG1, &params::FEED_TEMP_PHX);
    get(&socket, Device::HG1, &params::FEED_TEMP_BUH);
    get(&socket, Device::HG1, &params::OUTDOOR_TEMP_OPT);
    get(&socket, Device::HG1, &params::REFRIGERANT_TEMP);
    get(&socket, Device::HG1, &params::VOLUME_FLOW);
    get(&socket, Device::HG1, &params::WATER_PRESSURE);
    get(&socket, Device::HCM2, &params::MIXER_PUMP_STATUS);
    get(&socket, Device::HC2, &params::MIXER_PUMP_PWM);
    get(&socket, Device::HCM2, &params::MIXER_INFO_1);
    get(&socket, Device::HCM2, &params::MIXER_INFO_2);
    get(&socket, Device::HCM2, &params::MIXER_INFO_3);
    get(&socket, Device::HCM2, &params::MIXER_INFO_4);
    get(&socket, Device::HG1, &params::ENERGY_HP_COOLING);
    get(&socket, Device::HG1, &params::ENERGY_HP_HEATING);
    get(&socket, Device::HG1, &params::ENERGY_HOT_WATER);
    get(&socket, Device::HG1, &params::ENERGY_HP_TOTAL);
    get(&socket, Device::HG1, &params::ENERGY_ELECTRICAL);
    get(&socket, Device::HG1, &params::ENERGY_EXT_HOT_WATER);
    get(&socket, Device::HG1, &params::ENERGY_EXT_HEATING);
    get(&socket, Device::HG1, &params::RUNTIME_COMPRESSOR);
    get(&socket, Device::HG1, &params::RUNTIME_PUMP);

    let delta_t: f32 = (t_v.unwrap() - t_r.unwrap()).try_into().unwrap();

    //println!("-------------");
    //println!("flow = {} l/h", flow.unwrap());
    //println!("delta_t = {:.1} °C", delta_t);
    let flow_m3h = flow.unwrap() as f32 / 1000.0;
    let kwh_per_k = 1.16f32;
    let kwh = flow_m3h * delta_t * kwh_per_k;
    println!(
        "{} m³/h * {:.2} °C * {:.2} kWh/°C = {:.3} kW",
        flow_m3h, delta_t, kwh_per_k, kwh
    );
}
