use std::collections::HashMap;
use std::fmt;

use num_enum::FromPrimitive;
use socketcan::{CanDataFrame, CanFrame, CanSocket, Socket};
use strum::EnumMessage;

use crate::can::device::Device;
use crate::can::message::{Message, MessageType};
use crate::can::param::{BoolParam, EnumParam, FloatParam, IntParam, Param, Unit};
use crate::config::Config;
use crate::data::PARAMETERS;
use crate::old_model::{Parameter, ParameterType};
use crate::utils::read_toml_str;

const AUSSENTEMP: FloatParam = FloatParam {
    id: 0x000c,
    name: "AUSSENTEMP",
    unit: Unit::DegCelsius,
    scale: 10,
};

const AUSSENTEMP_WAERMEPUMPE: FloatParam = FloatParam {
    id: 0x0a0c,
    name: "AUSSENTEMP_WAERMEPUMPE",
    unit: Unit::DegCelsius,
    scale: 10,
};

const WW_POWER: FloatParam = FloatParam {
    id: 0x668,
    name: "WW POWER",
    unit: Unit::KiloWattHours,
    scale: 100,
};

const ENERGIE_EXT_QUELLE_WARMWASSER: IntParam = IntParam {
    id: 0x91C,
    name: "Energie ext. Quelle Warmwasser",
    unit: Unit::KiloWattHours,
};

const ENERGIE_EXT_QUELLE_HEIZUNG: IntParam = IntParam {
    id: 0x920,
    name: "Energie ext. Quelle Heizung",
    unit: Unit::KiloWattHours,
};

const ENERGIE_WP_KUEHLUNG: IntParam = IntParam {
    id: 0x6A6,
    name: "Energie WP Kühlung",
    unit: Unit::KiloWattHours,
};

const ENERGIE_WP_HEIZUNG: IntParam = IntParam {
    id: 0x6A7,
    name: "Energie WP Heizung",
    unit: Unit::KiloWattHours,
};

const ENERGIE_WARMWASSER: IntParam = IntParam {
    id: 0x92C,
    name: "Energie Warmwasser",
    unit: Unit::KiloWattHours,
};

const ENERGIE_WP_GESAMT: IntParam = IntParam {
    id: 0x930,
    name: "Energie WP gesamt",
    unit: Unit::KiloWattHours,
};

const LAUFZEIT_PUMPE: IntParam = IntParam {
    id: 0x6A4,
    name: "Laufzeit Pumpe",
    unit: Unit::Hours,
};

const LAUFZEIT_KOMPRESSOR: IntParam = IntParam {
    id: 0x6A5,
    name: "Laufzeit Kompressor",
    unit: Unit::Hours,
};

#[derive(Debug, FromPrimitive, strum::Display)]
#[repr(u8)]
enum Betriebsart {
    Standby = 0,
    Heizen = 1,
    Kühlen = 2,
    Abtauen = 3,
    Warmwasserbereitung = 4,
    #[num_enum(catch_all)]
    Unknown(u8),
}

const DEFROST_AKTIV: BoolParam = BoolParam {
    id: 0xC0F6,
    name: "DEFROST_AKTIV",
};

const VORLAUF: FloatParam = FloatParam {
    id: 0x000f,
    name: "Vorlauf",
    unit: Unit::DegCelsius,
    scale: 10,
};

#[derive(Debug, FromPrimitive, strum::Display)]
#[repr(u8)]
enum Programm {
    Bereitschaft = 1,
    Heizen = 3,
    Absenken = 4,
    Sommer = 5,
    Automatik1 = 11,
    Automatik2 = 12,
    Kuehlen = 17,
    #[num_enum(catch_all)]
    Unknown(u8),
}

// 1=Bereitschaft,3=Heizen,4=Absenken,5=Sommer,17=Kühlen,11=Automatik 1,12=Automatik 2
const PROGRAMMSCHALTER: EnumParam<Programm> = EnumParam {
    id: 0x0112,
    name: "Modus",
    default: None,
};

#[derive(Debug, FromPrimitive, strum::Display)]
#[repr(u8)]
enum ModusSg {
    Normal = 0,
    Sg1 = 1,
    Sg2 = 2,
    Sg3 = 3,
    #[num_enum(catch_all)]
    Unknown(u8),
}

const SG: EnumParam<ModusSg> = EnumParam {
    id: 0x0694,
    name: "SG",
    default: None,
};

/// Ok: Außentemperatur
const T_AU: FloatParam = FloatParam {
    id: 0xc176,
    name: "T-AU",
    unit: Unit::DegCelsius,
    scale: 10,
};

/// Ok
const P: FloatParam = FloatParam {
    id: 0x001c,
    name: "P",
    unit: Unit::Bar,
    scale: 1000,
};

/// Ok
const V: IntParam = IntParam {
    id: 0xc101,
    name: "V",
    unit: Unit::LitersPerHour,
};

// Probably correct, iobroker calls it EHS
const DHW_EHS: FloatParam = FloatParam {
    id: 0xc0f9,
    name: "DHW (EHS?)",
    unit: Unit::Percent,
    scale: 10, // FIXME what's the correct scale?
};

// Ok
const T_V_BH: FloatParam = FloatParam {
    id: 0xc0fe, // ???
    name: "t-V,BH",
    unit: Unit::DegCelsius,
    scale: 10,
};

// Ok
const T_V: FloatParam = FloatParam {
    id: 0xc0fc,
    name: "T_V",
    unit: Unit::DegCelsius,
    scale: 10,
};

// Ok
const T_R: FloatParam = FloatParam {
    id: 0xc104,
    name: "t-R",
    unit: Unit::DegCelsius,
    scale: 10,
};

/// Ok ???
const T_DHW: FloatParam = FloatParam {
    id: 0xc106,
    name: "T_DHW",
    unit: Unit::DegCelsius,
    scale: 10,
};

/// Ok
const T_LIQ: FloatParam = FloatParam {
    id: 0xc103,
    name: "t-liq",
    unit: Unit::DegCelsius,
    scale: 10,
};

// Ok
const B1: IntParam = IntParam {
    id: 0xc0fb,
    name: "MISCHERSTELLUNG_2 B1 [3UVB1]",
    unit: Unit::Percent,
};

// 3UVDHW ???
const MISCHERSTELLUNG_1: IntParam = IntParam {
    id: 0x069b,
    name: "MISCHERSTELLUNG_1 [3UV1]",
    unit: Unit::Percent,
};

const STATUS_KOMPRESSOR: BoolParam = BoolParam {
    id: 0x0061,
    name: "STATUS KOMPRESSOR",
};

const STATUS_UMWAELZ_PUMPE: BoolParam = BoolParam {
    id: 0xfd8c,
    name: "STATUS UMWÄLZ PUMPE",
};

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
        param.unit().get_message().unwrap_or("")
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

    get_and_print(&socket, Device::HG1, &T_AU);
    get_and_print(&socket, Device::HG1, &P);
    let flow = get_and_print(&socket, Device::HG1, &V);

    let t_r = get_and_print(&socket, Device::HG1, &T_R); // ???
    let t_v_bh = get_and_print(&socket, Device::HG1, &T_V_BH); // ???

    get_and_print(&socket, Device::HG1, &T_LIQ);
    let t_v = get_and_print(&socket, Device::HG1, &T_V); // ???

    get_and_print(&socket, Device::HG1, &T_DHW);

    get_and_print(&socket, Device::HG1, &B1);
    get_and_print(&socket, Device::HG1, &MISCHERSTELLUNG_1);

    //get_and_print(&socket, Device::HG1, &DHW_EHS);
    let delta_t = t_v - t_r;

    println!("-------------");
    println!("flow = {} l/h", flow);
    println!("delta_t = {:.1} °C", delta_t);
    let kwh_per_c = 1.16f32;
    let kwh = flow as f32 * delta_t * kwh_per_c;
    println!(
        "{} l/h * {:.2} °C * {:.2} kWh/°C = {:.3} kW",
        flow, delta_t, kwh_per_c, kwh
    );

    get_and_print(&socket, Device::HG1, &PROGRAMMSCHALTER);
    get_and_print(&socket, Device::HG1, &SG);
    get_and_print(&socket, Device::HG1, &DEFROST_AKTIV);

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

    get_and_print(&socket, Device::Outdoor, &STATUS_KOMPRESSOR);
    get_and_print(&socket, Device::Outdoor, &STATUS_UMWAELZ_PUMPE);

    Ok(())
}
