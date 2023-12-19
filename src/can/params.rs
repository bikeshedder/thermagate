use super::{
    enums,
    param::{BoolParam, DecParam, EnumParam, IntParam, Unit},
};
pub const AUSSENTEMP: DecParam = DecParam {
    id: 0x000c,
    name: "AUSSENTEMP",
    label: "Außentemperatur",
    unit: Some(Unit::DegCelsius),
    scale: 1,
};
pub const VORLAUF: DecParam = DecParam {
    id: 0x000f,
    name: "VORLAUF",
    label: "Vorlauf",
    unit: Some(Unit::DegCelsius),
    scale: 1,
};
pub const P: DecParam = DecParam {
    id: 0x001c,
    name: "P",
    label: "P",
    unit: Some(Unit::Bar),
    scale: 3,
};
pub const STATUS_KOMPRESSOR: BoolParam = BoolParam {
    id: 0x0061,
    name: "STATUS_KOMPRESSOR",
    label: "Status Kompressor",
};
pub const PROGRAMMSCHALTER: EnumParam<enums::Programm> = EnumParam {
    id: 0x0112,
    name: "PROGRAMMSCHALTER",
    label: "Modus",
    default: None,
};
pub const SOFTWARE_VERSION: IntParam = IntParam {
    id: 0x019a,
    name: "SOFTWARE_VERSION",
    label: "Software Version",
    unit: None,
};
pub const SOFTWARE_UNTERINDEX: IntParam = IntParam {
    id: 0x024b,
    name: "SOFTWARE_UNTERINDEX",
    label: "Software Unterindex",
    unit: None,
};
pub const LEISTUNG_WW: IntParam = IntParam {
    id: 0x0668,
    name: "LEISTUNG_WW",
    label: "Leistung Warmwasser",
    unit: Some(Unit::KiloWattHours),
};
pub const SG: EnumParam<enums::SmartGrid> = EnumParam {
    id: 0x0694,
    name: "SG",
    label: "Smart Grid",
    default: None,
};
pub const MISCHERSTELLUNG_1: IntParam = IntParam {
    id: 0x069b,
    name: "MISCHERSTELLUNG_1",
    label: "MISCHERSTELLUNG_1 [3UV1]",
    unit: Some(Unit::Percent),
};
pub const ENERGIE_WP_KUEHLUNG: IntParam = IntParam {
    id: 0x06a6,
    name: "ENERGIE_WP_KUEHLUNG",
    label: "Energie WP Kühlung",
    unit: Some(Unit::KiloWattHours),
};
pub const ENERGIE_WP_HEIZUNG: IntParam = IntParam {
    id: 0x06a7,
    name: "ENERGIE_WP_HEIZUNG",
    label: "Energie WP Heizung",
    unit: Some(Unit::KiloWattHours),
};
pub const LAUFZEIT_PUMPE: IntParam = IntParam {
    id: 0x06a4,
    name: "LAUFZEIT_PUMPE",
    label: "Laufzeit Pumpe",
    unit: Some(Unit::Hours),
};
pub const LAUFZEIT_KOMPRESSOR: IntParam = IntParam {
    id: 0x06a5,
    name: "LAUFZEIT_KOMPRESSOR",
    label: "Laufzeit Kompressor",
    unit: Some(Unit::Hours),
};
pub const ENERGIE_EXT_QUELLE_WARMWASSER: IntParam = IntParam {
    id: 0x091c,
    name: "ENERGIE_EXT_QUELLE_WARMWASSER",
    label: "Energie ext. Quelle Warmwasser",
    unit: Some(Unit::KiloWattHours),
};
pub const ENERGIE_EXT_QUELLE_HEIZUNG: IntParam = IntParam {
    id: 0x0920,
    name: "ENERGIE_EXT_QUELLE_HEIZUNG",
    label: "Energie ext. Quelle Heizung",
    unit: Some(Unit::KiloWattHours),
};
pub const ENERGIE_WARMWASSER: IntParam = IntParam {
    id: 0x092c,
    name: "ENERGIE_WARMWASSER",
    label: "Energie Warmwasser",
    unit: Some(Unit::KiloWattHours),
};
pub const ENERGIE_WP_GESAMT: IntParam = IntParam {
    id: 0x0930,
    name: "ENERGIE_WP_GESAMT",
    label: "Energie WP gesamt",
    unit: Some(Unit::KiloWattHours),
};
pub const AUSSENTEMP_WAERMEPUMPE: DecParam = DecParam {
    id: 0x0a0c,
    name: "AUSSENTEMP_WAERMEPUMPE",
    label: "Außentemperatur Wärmepumpe",
    unit: Some(Unit::DegCelsius),
    scale: 1,
};
pub const DEFROST_AKTIV: BoolParam = BoolParam {
    id: 0xc0f6,
    name: "DEFROST_AKTIV",
    label: "Defrost Aktiv",
};
pub const DHW: DecParam = DecParam {
    id: 0xc0f9,
    name: "DHW",
    label: "DHW/EHS",
    unit: Some(Unit::Percent),
    scale: 1,
};
pub const B1: IntParam = IntParam {
    id: 0xc0fb,
    name: "B1",
    label: "MISCHERSTELLUNG_2 B1 [3UVB1]",
    unit: Some(Unit::Percent),
};
pub const T_V: DecParam = DecParam {
    id: 0xc0fc,
    name: "T_V",
    label: "t-V",
    unit: Some(Unit::DegCelsius),
    scale: 1,
};
pub const T_V_BH: DecParam = DecParam {
    id: 0xc0fe,
    name: "T_V_BH",
    label: "t-V BH",
    unit: Some(Unit::DegCelsius),
    scale: 1,
};
pub const V: IntParam = IntParam {
    id: 0xc101,
    name: "V",
    label: "Volumenstrom",
    unit: Some(Unit::LitersPerHour),
};
pub const T_LIQ: DecParam = DecParam {
    id: 0xc103,
    name: "T_LIQ",
    label: "t-liq",
    unit: Some(Unit::DegCelsius),
    scale: 1,
};
pub const T_R: DecParam = DecParam {
    id: 0xc104,
    name: "T_R",
    label: "t-R",
    unit: Some(Unit::DegCelsius),
    scale: 1,
};
pub const T_DHW: DecParam = DecParam {
    id: 0xc106,
    name: "T_DHW",
    label: "T_DHW",
    unit: Some(Unit::DegCelsius),
    scale: 1,
};
pub const T_AU: DecParam = DecParam {
    id: 0xc176,
    name: "T_AU",
    label: "Außentemperatur",
    unit: Some(Unit::DegCelsius),
    scale: 1,
};
pub const STATUS_UMWAELZPUMPE: BoolParam = BoolParam {
    id: 0xfd8c,
    name: "STATUS_UMWAELZPUMPE",
    label: "Status Umwälzpumpe",
};
