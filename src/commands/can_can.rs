use std::collections::HashMap;

use socketcan::{CanDataFrame, CanFrame, CanSocket, Socket};

use crate::can2::device::Device;
use crate::can2::message::{Message, MessageType};
use crate::config::Config;
use crate::data::PARAMETERS;
use crate::model::{Parameter, ParameterType};
use crate::utils::read_toml_str;

pub struct Query {
    name: &'static str,
    unit: &'static str,
    data_id: u16,
}

const WW_POWER: Query = Query {
    name: "WW POWER",
    unit: "kW/100",
    data_id: 0x668,
};

const ENERGIE_EXT_QUELLE_WARMWASSER: Query = Query {
    name: "Energie ext. Quelle Warmwasser",
    unit: "kWh",
    data_id: 0x91C,
};

const ENERGIE_EXT_QUELLE_HEIZUNG: Query = Query {
    name: "Energie ext. Quelle Heizung",
    unit: "kWh",
    data_id: 0x920,
};

const ENERGIE_WP_KUEHLUNG: Query = Query {
    name: "Energie WP Kühlung",
    unit: "kWh",
    data_id: 0x6A6,
};

const ENERGIE_WP_HEIZUNG: Query = Query {
    name: "Energie WP Heizung",
    unit: "kWh",
    data_id: 0x6A7,
};

const ENERGIE_WARMWASSER: Query = Query {
    name: "Energie Warmwasser",
    unit: "kWh",
    data_id: 0x92C,
};

const ENERGIE_WP_GESAMT: Query = Query {
    name: "Energie WP gesamt",
    unit: "kWh",
    data_id: 0x930,
};

const LAUFZEIT_PUMPE: Query = Query {
    name: "Laufzeit Pumpe",
    unit: "h",
    data_id: 0x6A4,
};

const LAUFZEIT_KOMPRESSOR: Query = Query {
    name: "Laufzeit Kompressor",
    unit: "h",
    data_id: 0x6A5,
};

const BETRIEBSART: Query = Query {
    name: "Betriebsart",
    // 0=Standby,1=Heizen,2=Kühlen,3=Abtauen,4=Warmwasserbereitung
    unit: "[enum]",
    data_id: 0xC0F6,
};

pub fn get(socket: &CanSocket, dev: Device, q: &Query) {
    let req = Message {
        sender: Device::GW,
        receiver: dev,
        param: q.data_id,
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
        if msg.receiver == Device::GW {
            println!("{:02x?} {:20} {:02x?} {}", msg, q.name, msg.data, q.unit);
            return;
        }
    }
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

    get(&socket, Device::HG1, &ENERGIE_WP_KUEHLUNG);
    get(&socket, Device::HG1, &ENERGIE_WP_HEIZUNG);
    get(&socket, Device::HG1, &WW_POWER);
    get(&socket, Device::HG1, &ENERGIE_EXT_QUELLE_WARMWASSER);
    get(&socket, Device::HG1, &ENERGIE_EXT_QUELLE_HEIZUNG);
    get(&socket, Device::HG1, &ENERGIE_WARMWASSER);
    get(&socket, Device::HG1, &ENERGIE_WP_GESAMT);
    get(&socket, Device::HG1, &LAUFZEIT_KOMPRESSOR);
    get(&socket, Device::HG1, &LAUFZEIT_PUMPE);
    get(&socket, Device::HG1, &BETRIEBSART);

    Ok(())
}
