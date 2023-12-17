use core::fmt;
use std::thread;
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};

use socketcan::{CanDataFrame, CanFrame, CanSocket, EmbeddedFrame, Frame, Socket, StandardId};
use strum::{AsRefStr, Display, FromRepr};
use thiserror::Error;

use crate::can::BusDriver;
use crate::can2::device::Device;
use crate::can2::message::{Message, Payload};
use crate::config::Config;
use crate::data::{DEVICES, PARAMETERS};
use crate::model::{Parameter, ParameterType, State};
use crate::utils::read_toml_str;

pub struct Query {
    name: &'static str,
    unit: &'static str,
    get: [u8; 7],
    set: [u8; 3],
    data_id: u16,
}

const WW_POWER: Query = Query {
    name: "WW POWER",
    unit: "kW/100",
    get: [0x31, 0x00, 0xFA, 0x06, 0x68, 0x00, 0x00],
    set: [0xfa, 0x06, 0x68],
    data_id: 0x668,
};

const ENERGIE_EXT_QUELLE_WARMWASSER: Query = Query {
    name: "Energie ext. Quelle Warmwasser",
    unit: "kWh",
    get: [0x31, 0x00, 0xFA, 0x09, 0x20, 0x00, 0x00],
    set: [0xFA, 0x09, 0x1C],
    data_id: 0x91C,
};

const ENERGIE_EXT_QUELLE_HEIZUNG: Query = Query {
    name: "Energie ext. Quelle Heizung",
    unit: "kWh",
    get: [0x31, 0x00, 0xFA, 0xC0, 0xF9, 0x00, 0x00],
    set: [0xFA, 0x09, 0x20],
    data_id: 0x920,
};

const ENERGIE_WP_KUEHLUNG: Query = Query {
    name: "Energie WP Kühlung",
    unit: "kWh",
    get: [0x31, 0x00, 0xFA, 0x06, 0xA6, 0x00, 0x00],
    set: [0xFA, 0x06, 0xA6],
    data_id: 0x6A6,
};

const ENERGIE_WP_HEIZUNG: Query = Query {
    name: "Energie WP Heizung",
    unit: "kWh",
    get: [0x31, 0x00, 0xFA, 0x06, 0xA7, 0x00, 0x00],
    set: [0xFA, 0x06, 0xA7],
    data_id: 0x6A7,
};

const ENERGIE_WARMWASSER: Query = Query {
    name: "Energie Warmwasser",
    unit: "kWh",
    get: [0x31, 0x00, 0xFA, 0x09, 0x2C, 0x00, 0x00],
    set: [0xFA, 0x09, 0x2C],
    data_id: 0x92C,
};

const ENERGIE_WP_GESAMT: Query = Query {
    name: "Energie WP gesamt",
    unit: "kWh",
    get: [0x31, 0x00, 0xFA, 0x09, 0x30, 0x00, 0x00],
    set: [0xFA, 0x09, 0x30],
    data_id: 0x930,
};

const LAUFZEIT_PUMPE: Query = Query {
    name: "Laufzeit Pumpe",
    unit: "h",
    get: [0x31, 0x00, 0xFA, 0x06, 0xA4, 0x00, 0x00],
    set: [0xFA, 0x06, 0xA4],
    data_id: 0x6A4,
};

const LAUFZEIT_KOMPRESSOR: Query = Query {
    name: "Laufzeit Kompressor",
    unit: "h",
    get: [0x31, 0x00, 0xFA, 0x06, 0xA5, 0x00, 0x00],
    set: [0xFA, 0x06, 0xA5],
    data_id: 0x6A5,
};

const BETRIEBSART: Query = Query {
    name: "Betriebsart",
    // 0=Standby,1=Heizen,2=Kühlen,3=Abtauen,4=Warmwasserbereitung
    unit: "[enum]",
    get: [0x31, 0x00, 0xFA, 0xC0, 0xF6, 0x00, 0x00],
    set: [0xFA, 0xC0, 0xF6],
    data_id: 0xC0F6,
};

pub fn get(socket: &CanSocket, id: u16, q: &Query) {
    let mut req = CanDataFrame::default();
    req.set_id(StandardId::new(Device::NRG as u16).unwrap());
    req.set_data(&q.get).unwrap();
    socket.write_frame(&req).unwrap();
    thread::sleep(Duration::from_millis(100));
    // wait for response
    while let Ok(frame) = socket.read_frame() {
        let CanFrame::Data(data_frame) = frame else {
            continue;
        };
        let Ok(msg) = Message::parse(&data_frame) else {
            continue;
        };
        let Payload::Response(rsp) = msg.payload else {
            continue;
        };
        if rsp.id == id {
            println!("{:20} {:02x?} {}", q.name, rsp.data, q.unit);
        }
        /*
        let data = data_frame.data();
        let [_d0, _d1, p0, p1, p2, v0, v1] = *data else {
            continue;
        };
        if [p0, p1, p2] == q.set {
            let value = ((v0 as u16) << 8) + v1 as u16;
            println!("{:20} {:5} {}", q.name, value, q.unit);
            return;
        }
        */
    }
    thread::sleep(Duration::from_millis(100));
}

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

    //get(&socket, 0x666, &WW_POWER);
    /*
    //get(&socket, 0x180, &ENERGIE_EXT_QUELLE_WARMWASSER);
    //get(&socket, 0x180, &ENERGIE_EXT_QUELLE_HEIZUNG);
    get(&socket, 0x300, &ENERGIE_WP_KUEHLUNG);
    get(&socket, 0x300, &ENERGIE_WP_HEIZUNG);
    get(&socket, 0x300, &ENERGIE_WARMWASSER);
    get(&socket, 0x300, &ENERGIE_WP_GESAMT);
    get(&socket, 0x300, &LAUFZEIT_KOMPRESSOR);
    get(&socket, 0x300, &LAUFZEIT_PUMPE);
    //get(&socket, 0x680, &BETRIEBSART);
    */

    while let Ok(frame) = socket.read_frame() {
        let CanFrame::Data(data_frame) = frame else {
            continue;
        };
        let data = data_frame.data();
        let msg = Message::parse(&data_frame);

        /*
        let [_d0, _d1, p0, p1, p2, v0, v1] = *data else {
            continue;
        };
        let device = Device::from_repr(data_frame.raw_id());
        let device_name = device
            .map(|d| d.as_ref().to_owned())
            .unwrap_or(format!("{:x}", data_frame.raw_id()));
        */
        //println!("{:02x?} {:?}", data, msg);

        match msg {
            Ok(msg) => {
                if let Some(p) = param_by_id.get(&msg.param_id()) {
                    println!("{:02x?} {} {} {:?}", data, msg, p.name, p.r#type);
                } else {
                    println!("{:02x?} {}", data, msg);
                }
            }
            Err(e) => println!("{:02x?} Error: {}", data, e),
        }
    }

    Ok(())
}
