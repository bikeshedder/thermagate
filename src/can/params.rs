use super::{
    enums,
    param::{
        BoolParam, DecParam, Enum16Param, Enum8Param, I16Param, I8Param, MultilingualStr, Param,
        TimeRangeParam, Unit,
    },
};
use rust_decimal_macros::dec;
pub const KESSELSOLLTEMP: DecParam = DecParam {
    id: 0x0002,
    name: "KESSELSOLLTEMP",
    label: MultilingualStr {
        de: "Kessel Soll-Temperatur",
        en: "Boiler set temperature",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: Some(dec!(55.0)),
    min: Some(dec!(40.0)),
    max: Some(dec!(80.0)),
};
pub const VERSTELLTE_SPEICHERSOLLTEMP: DecParam = DecParam {
    id: 0x0003,
    name: "VERSTELLTE_SPEICHERSOLLTEMP",
    label: MultilingualStr {
        de: "Verstellte Warmwasser Speichersolltemperatur",
        en: "Modified DHW setpoint",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: Some(dec!(48.0)),
    min: Some(dec!(10.0)),
    max: Some(dec!(70.0)),
};
pub const VORLAUFSOLLTEMP: DecParam = DecParam {
    id: 0x0004,
    name: "VORLAUFSOLLTEMP",
    label: MultilingualStr {
        de: "eVORLAUFSOLLTEMP",
        en: "eVORLAUFSOLLTEMP",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const RAUMSOLLTEMP_I: DecParam = DecParam {
    id: 0x0005,
    name: "RAUMSOLLTEMP_I",
    label: MultilingualStr {
        de: "Raum-Soll-Temperatur 1",
        en: "Room setpoint 1",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: Some(dec!(20.0)),
    min: Some(dec!(5.0)),
    max: Some(dec!(40.0)),
};
pub const RAUMSOLLTEMP_II: DecParam = DecParam {
    id: 0x0006,
    name: "RAUMSOLLTEMP_II",
    label: MultilingualStr {
        de: "Raum-Soll-Temperatur 2",
        en: "Room setpoint 2",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: Some(dec!(20.0)),
    min: Some(dec!(5.0)),
    max: Some(dec!(40.0)),
};
pub const RAUMSOLLTEMP_III: DecParam = DecParam {
    id: 0x0007,
    name: "RAUMSOLLTEMP_III",
    label: MultilingualStr {
        de: "Raum-Soll-Temperatur 3",
        en: "Room setpoint 3",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: Some(dec!(20.0)),
    min: Some(dec!(5.0)),
    max: Some(dec!(40.0)),
};
pub const NACHTRAUMSOLLTEMP: DecParam = DecParam {
    id: 0x0008,
    name: "NACHTRAUMSOLLTEMP",
    label: MultilingualStr {
        de: "Absenktemperatur",
        en: "Eco setpoint",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: Some(dec!(15.0)),
    min: Some(dec!(5.0)),
    max: Some(dec!(40.0)),
};
pub const AUSSENTEMP: DecParam = DecParam {
    id: 0x000c,
    name: "AUSSENTEMP",
    label: MultilingualStr {
        de: "Außentemperatur",
        en: "ambient temperatur",
    },
    mutable: false,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const KESSELISTTEMP: DecParam = DecParam {
    id: 0x000d,
    name: "KESSELISTTEMP",
    label: MultilingualStr {
        de: "Kessel Ist-Temperatur",
        en: "Boiler flow temperature",
    },
    mutable: false,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const SPEICHERISTTEMP: DecParam = DecParam {
    id: 0x000e,
    name: "SPEICHERISTTEMP",
    label: MultilingualStr {
        de: "Warmwasser Ist-Temperatur",
        en: "DHW temperature",
    },
    mutable: false,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const VORLAUFISTTEMP: DecParam = DecParam {
    id: 0x000f,
    name: "VORLAUFISTTEMP",
    label: MultilingualStr {
        de: "eVORLAUFISTTEMP",
        en: "eVORLAUFISTTEMP",
    },
    mutable: false,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const RAUMISTTEMP: DecParam = DecParam {
    id: 0x0011,
    name: "RAUMISTTEMP",
    label: MultilingualStr {
        de: "Raum-Ist-Temperatur",
        en: "Room temperature",
    },
    mutable: false,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const VERSTELLTE_RAUMSOLLTEMP: DecParam = DecParam {
    id: 0x0012,
    name: "VERSTELLTE_RAUMSOLLTEMP",
    label: MultilingualStr {
        de: "Raum-Soll-Temperatur",
        en: "Room setpoint",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: Some(dec!(5.0)),
    max: Some(dec!(40.0)),
};
pub const EINSTELL_SPEICHERSOLLTEMP: DecParam = DecParam {
    id: 0x0013,
    name: "EINSTELL_SPEICHERSOLLTEMP",
    label: MultilingualStr {
        de: "Warmwasser Speichersolltemperatur 1",
        en: "DHW setpoint",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: Some(dec!(48.0)),
    min: Some(dec!(10.0)),
    max: Some(dec!(70.0)),
};
pub const RUECKLAUFTEMP: DecParam = DecParam {
    id: 0x0016,
    name: "RUECKLAUFTEMP",
    label: MultilingualStr {
        de: "Rücklauftemperatur",
        en: "Return temperature",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const P: DecParam = DecParam {
    id: 0x001c,
    name: "P",
    label: MultilingualStr {
        de: "Wasserdruck",
        en: "Water pressure",
    },
    mutable: false,
    unit: Some(Unit::Bar),
    scale: 3,
    default: None,
    min: None,
    max: None,
};
pub const MAX_VORLAUFTEMP: DecParam = DecParam {
    id: 0x0028,
    name: "MAX_VORLAUFTEMP",
    label: MultilingualStr {
        de: "Maximaltemperatur Heizung",
        en: "Max T-Flow",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WW_AKTIV: I16Param = I16Param {
    id: 0x005e,
    name: "WW_AKTIV",
    label: MultilingualStr {
        de: "Warmwasserbereitung aktiv",
        en: "DHW heating up active",
    },
    mutable: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const STATUS_KOMPRESSOR: BoolParam = BoolParam {
    id: 0x0061,
    name: "STATUS_KOMPRESSOR",
    label: MultilingualStr {
        de: "Status Kompressor",
        en: "Status Compressor",
    },
    mutable: false,
    default: None,
};
pub const AUFHEIZOPTIMIERUNG: I8Param = I8Param {
    id: 0x0103,
    name: "AUFHEIZOPTIMIERUNG",
    label: MultilingualStr {
        de: "eAUFHEIZOPTIMIERUNG",
        en: "eAUFHEIZOPTIMIERUNG",
    },
    mutable: true,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const HZKKURVE: DecParam = DecParam {
    id: 0x010e,
    name: "HZKKURVE",
    label: MultilingualStr {
        de: "Heizkurve",
        en: "Heat-Slope",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const RAUMEINFLUSS: DecParam = DecParam {
    id: 0x010f,
    name: "RAUMEINFLUSS",
    label: MultilingualStr {
        de: "Raumeinflussfaktor",
        en: "Room Influence",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const MAX_AUFHEIZVORVERLEGUNG: I16Param = I16Param {
    id: 0x0110,
    name: "MAX_AUFHEIZVORVERLEGUNG",
    label: MultilingualStr {
        de: "eMAX_AUFHEIZVORVERLEGUNG",
        en: "eMAX_AUFHEIZVORVERLEGUNG",
    },
    mutable: true,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const PROGRAMMSCHALTER: Enum8Param<enums::Programm> = Enum8Param {
    id: 0x0112,
    name: "PROGRAMMSCHALTER",
    label: MultilingualStr {
        de: "Programmschalter",
        en: "mode",
    },
    mutable: true,
    default: None,
    values: phf::phf_ordered_map! { 1i8 => "Bereitschaft" , 3i8 => "Heizen" , 4i8 => "Absenken" , 5i8 => "Sommer" , 11i8 => "Automatik1" , 12i8 => "Automatik2" , 17i8 => "Kuehlen" },
};
pub const ADAPTION: I8Param = I8Param {
    id: 0x0115,
    name: "ADAPTION",
    label: MultilingualStr {
        de: "eADAPTION",
        en: "eADAPTION",
    },
    mutable: true,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const HEIZGRENZE_TAG: DecParam = DecParam {
    id: 0x0116,
    name: "HEIZGRENZE_TAG",
    label: MultilingualStr {
        de: "Heizgrenze Tag",
        en: "T-Outside lim day",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const HEIZGRENZE_NACHT: DecParam = DecParam {
    id: 0x0117,
    name: "HEIZGRENZE_NACHT",
    label: MultilingualStr {
        de: "Heizgrenze Nacht",
        en: "T-Outside lim night",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const MODUS_URLAUB_ANFANG_TAG: I8Param = I8Param {
    id: 0x011b,
    name: "MODUS_URLAUB_ANFANG_TAG",
    label: MultilingualStr {
        de: "Urlaubsbeginn Tag",
        en: "Holiday start day",
    },
    mutable: true,
    unit: None,
    default: Some(2),
    min: Some(0),
    max: Some(31),
};
pub const MODUS_URLAUB_ANFANG_MONAT: I8Param = I8Param {
    id: 0x011c,
    name: "MODUS_URLAUB_ANFANG_MONAT",
    label: MultilingualStr {
        de: "Urlaubsbeginn Monat",
        en: "Holiday start month",
    },
    mutable: true,
    unit: None,
    default: Some(1),
    min: Some(0),
    max: Some(12),
};
pub const MODUS_URLAUB_ANFANG_JAHR: I8Param = I8Param {
    id: 0x011d,
    name: "MODUS_URLAUB_ANFANG_JAHR",
    label: MultilingualStr {
        de: "Urlaubsbeginn Jahr",
        en: "Holiday start year",
    },
    mutable: true,
    unit: None,
    default: Some(1),
    min: Some(0),
    max: Some(49),
};
pub const MODUS_URLAUB_ENDE_TAG: I8Param = I8Param {
    id: 0x011e,
    name: "MODUS_URLAUB_ENDE_TAG",
    label: MultilingualStr {
        de: "Urlaubsende Tag",
        en: "Holiday end day",
    },
    mutable: true,
    unit: None,
    default: Some(3),
    min: Some(0),
    max: Some(31),
};
pub const MODUS_URLAUB_ENDE_MONAT: I8Param = I8Param {
    id: 0x011f,
    name: "MODUS_URLAUB_ENDE_MONAT",
    label: MultilingualStr {
        de: "Urlaubsende Monat",
        en: "Holiday end month",
    },
    mutable: true,
    unit: None,
    default: Some(1),
    min: Some(0),
    max: Some(12),
};
pub const MODUS_URLAUB_ENDE_JAHR: I8Param = I8Param {
    id: 0x0120,
    name: "MODUS_URLAUB_ENDE_JAHR",
    label: MultilingualStr {
        de: "Urlaubsende Jahr",
        en: "Holiday end year",
    },
    mutable: true,
    unit: None,
    default: Some(1),
    min: Some(0),
    max: Some(49),
};
pub const TAG: I8Param = I8Param {
    id: 0x0122,
    name: "TAG",
    label: MultilingualStr {
        de: "Tag",
        en: "Day",
    },
    mutable: true,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const MONAT: I8Param = I8Param {
    id: 0x0123,
    name: "MONAT",
    label: MultilingualStr {
        de: "Monat",
        en: "Month",
    },
    mutable: true,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const JAHR: I8Param = I8Param {
    id: 0x0124,
    name: "JAHR",
    label: MultilingualStr {
        de: "Jahr",
        en: "Year",
    },
    mutable: true,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const STUNDE: I8Param = I8Param {
    id: 0x0125,
    name: "STUNDE",
    label: MultilingualStr {
        de: "Stunde",
        en: "Hour",
    },
    mutable: true,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const MINUTE: I8Param = I8Param {
    id: 0x0126,
    name: "MINUTE",
    label: MultilingualStr {
        de: "Minute",
        en: "Minute",
    },
    mutable: true,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const VORLAUFSOLLTEMP_TAG: DecParam = DecParam {
    id: 0x0129,
    name: "VORLAUFSOLLTEMP_TAG",
    label: MultilingualStr {
        de: "Vorlaufsolltemperatur Tag bei deaktivierter Witterungsführung",
        en: "T-Flow Day",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: Some(dec!(10.0)),
    max: Some(dec!(70.0)),
};
pub const VORLAUFSOLLTEMP_NACHT: DecParam = DecParam {
    id: 0x012a,
    name: "VORLAUFSOLLTEMP_NACHT",
    label: MultilingualStr {
        de: "Vorlaufsolltemperatur Nacht bei deaktivierter Witterungsführung",
        en: "T-Flow Night",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const MIN_VORLAUFTEMP: DecParam = DecParam {
    id: 0x012b,
    name: "MIN_VORLAUFTEMP",
    label: MultilingualStr {
        de: "eMIN_VORLAUFTEMP",
        en: "eMIN_VORLAUFTEMP",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const ABSENKOPTIMIERUNG: I16Param = I16Param {
    id: 0x012e,
    name: "ABSENKOPTIMIERUNG",
    label: MultilingualStr {
        de: "eABSENKOPTIMIERUNG",
        en: "eABSENKOPTIMIERUNG",
    },
    mutable: true,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const ABWESEND_RAUMSOLLTEMP: DecParam = DecParam {
    id: 0x013d,
    name: "ABWESEND_RAUMSOLLTEMP",
    label: MultilingualStr {
        de: "Raumsolltemperatur bei Abwesenheit",
        en: "T-Flow Night",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: Some(dec!(15.0)),
    min: Some(dec!(5.0)),
    max: Some(dec!(40.0)),
};
pub const EINSTELL_SPEICHERSOLLTEMP3: DecParam = DecParam {
    id: 0x013e,
    name: "EINSTELL_SPEICHERSOLLTEMP3",
    label: MultilingualStr {
        de: "Warmwasser Speichersolltemperatur 3",
        en: "DHW setpoint 3",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: Some(dec!(48.0)),
    min: Some(dec!(10.0)),
    max: Some(dec!(70.0)),
};
pub const HZK_FUNKTION: I16Param = I16Param {
    id: 0x0141,
    name: "HZK_FUNKTION",
    label: MultilingualStr {
        de: "Witterungsgeführt",
        en: "Weather-controlled",
    },
    mutable: true,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const EINMAL_WW_AKTIV: BoolParam = BoolParam {
    id: 0x0144,
    name: "EINMAL_WW_AKTIV",
    label: MultilingualStr {
        de: "Einmalige WW-Ladung",
        en: "1x Hot Water",
    },
    mutable: true,
    default: Some(false),
};
pub const GERAETE_KENNUNG: I8Param = I8Param {
    id: 0x0148,
    name: "GERAETE_KENNUNG",
    label: MultilingualStr {
        de: "Wärmeerzeugertyp",
        en: "Type of heat generator",
    },
    mutable: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const ZIRKPUMPE_BEI_WWFREIGABE: I16Param = I16Param {
    id: 0x0182,
    name: "ZIRKPUMPE_BEI_WWFREIGABE",
    label: MultilingualStr {
        de: "Aktivierung der Zirkulationspumpe",
        en: "Circl-Pump DHW",
    },
    mutable: true,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const SOFTWARE_NUMMER: I16Param = I16Param {
    id: 0x0199,
    name: "SOFTWARE_NUMMER",
    label: MultilingualStr {
        de: "Softwarenummer",
        en: "software #",
    },
    mutable: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const SOFTWARE_VERSION: I16Param = I16Param {
    id: 0x019a,
    name: "SOFTWARE_VERSION",
    label: MultilingualStr {
        de: "Softwareindex",
        en: "software index",
    },
    mutable: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const VOLUMENSTROM: DecParam = DecParam {
    id: 0x01da,
    name: "VOLUMENSTROM",
    label: MultilingualStr {
        de: "Volumenstrom",
        en: "Volume flow rate",
    },
    mutable: true,
    unit: None,
    scale: 0,
    default: None,
    min: None,
    max: None,
};
pub const SOFTWARE_UNTERINDEX: I16Param = I16Param {
    id: 0x024b,
    name: "SOFTWARE_UNTERINDEX",
    label: MultilingualStr {
        de: "cSOFTWARE_UNTERINDEX",
        en: "cSOFTWARE_UNTERINDEX",
    },
    mutable: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const ANTILEG_TEMP: DecParam = DecParam {
    id: 0x0587,
    name: "ANTILEG_TEMP",
    label: MultilingualStr {
        de: "Settemperatur Antilegionellenprogramm",
        en: "Anti-Legionella temp",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const VMIN_A1: DecParam = DecParam {
    id: 0x0661,
    name: "VMIN_A1",
    label: MultilingualStr {
        de: "Durchfluss",
        en: "Volume flow",
    },
    mutable: false,
    unit: None,
    scale: 0,
    default: None,
    min: None,
    max: None,
};
pub const LEISTUNG_WW: DecParam = DecParam {
    id: 0x0668,
    name: "LEISTUNG_WW",
    label: MultilingualStr {
        de: "Leistung Warmwasser",
        en: "Power warm water",
    },
    mutable: false,
    unit: Some(Unit::KiloWatt),
    scale: 2,
    default: None,
    min: None,
    max: None,
};
pub const WP_LEISTUNG_HEIZSTAB_S1: DecParam = DecParam {
    id: 0x0669,
    name: "WP_LEISTUNG_HEIZSTAB_S1",
    label: MultilingualStr {
        de: "Leistung Heizstab Stufe 1",
        en: "BUH s1 power",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_LEISTUNG_HEIZSTAB_S2: DecParam = DecParam {
    id: 0x066a,
    name: "WP_LEISTUNG_HEIZSTAB_S2",
    label: MultilingualStr {
        de: "Leistung Heizstab Stuife 2",
        en: "BUH s2 power",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_LEISTUNG_HZU_BIV: DecParam = DecParam {
    id: 0x066b,
    name: "WP_LEISTUNG_HZU_BIV",
    label: MultilingualStr {
        de: "Leistung HZU BIV",
        en: "Power BIV",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_MAX_TEMP_HEIZUNG: DecParam = DecParam {
    id: 0x066e,
    name: "WP_MAX_TEMP_HEIZUNG",
    label: MultilingualStr {
        de: "Maximaltemperature Heizung",
        en: "Tvbh1 max",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_HT_NT_FUNKTION: DecParam = DecParam {
    id: 0x066f,
    name: "WP_HT_NT_FUNKTION",
    label: MultilingualStr {
        de: "Funktion HT / NT Kontakt",
        en: "HT/NT Contact",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_HT_NT_FKT_ANSCHLUSS: DecParam = DecParam {
    id: 0x0670,
    name: "WP_HT_NT_FKT_ANSCHLUSS",
    label: MultilingualStr {
        de: "Anschlussoption HT / NT Kontakt",
        en: "HT/NT Function",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_RAUMTHERMOSTAT: DecParam = DecParam {
    id: 0x0678,
    name: "WP_RAUMTHERMOSTAT",
    label: MultilingualStr {
        de: "Funktion Raumthermostat",
        en: "Room thermostat",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_INTERLINKFUNKTION: DecParam = DecParam {
    id: 0x0679,
    name: "WP_INTERLINKFUNKTION",
    label: MultilingualStr {
        de: "Funktion Interlink",
        en: "Interlink function",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_PWM_LEISTUNG_MAX: DecParam = DecParam {
    id: 0x067e,
    name: "WP_PWM_LEISTUNG_MAX",
    label: MultilingualStr {
        de: "Maximalbegrenzung PWM Signal Heizkreispumpe",
        en: "Max Perform Pump",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_PWM_LEISTUNG_MIN: DecParam = DecParam {
    id: 0x067f,
    name: "WP_PWM_LEISTUNG_MIN",
    label: MultilingualStr {
        de: "Minimalbegrenzung PWM Signal Heizkreispumpe",
        en: "Min Perform Pump",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_MOD_HYST_DURCHFLUSS: DecParam = DecParam {
    id: 0x0682,
    name: "WP_MOD_HYST_DURCHFLUSS",
    label: MultilingualStr {
        de: "eWP_MOD_HYST_DURCHFLUSS",
        en: "eWP_MOD_HYST_DURCHFLUSS",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_SPREIZUNG_HZ_BETRIEB: DecParam = DecParam {
    id: 0x0683,
    name: "WP_SPREIZUNG_HZ_BETRIEB",
    label: MultilingualStr {
        de: "Sollspreizung Heizung",
        en: "Delt-T CH",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_SPREIZUNG_WW_BETRIEB: DecParam = DecParam {
    id: 0x0684,
    name: "WP_SPREIZUNG_WW_BETRIEB",
    label: MultilingualStr {
        de: "Sollspreizung Warmwasser",
        en: "Delta- DHW",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_VERZ_ZEIT_PUMPE: DecParam = DecParam {
    id: 0x0685,
    name: "WP_VERZ_ZEIT_PUMPE",
    label: MultilingualStr {
        de: "eWP_VERZ_ZEIT_PUMPE",
        en: "eWP_VERZ_ZEIT_PUMPE",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const VMIN_WP: DecParam = DecParam {
    id: 0x0688,
    name: "VMIN_WP",
    label: MultilingualStr {
        de: "Durchfluss",
        en: "Volume flow",
    },
    mutable: false,
    unit: None,
    scale: 0,
    default: None,
    min: None,
    max: None,
};
pub const WP_START_MAX_TEMP: DecParam = DecParam {
    id: 0x068c,
    name: "WP_START_MAX_TEMP",
    label: MultilingualStr {
        de: "eWP_START_MAX_TEMP",
        en: "eWP_START_MAX_TEMP",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_HYSTERESE_DHW: DecParam = DecParam {
    id: 0x0691,
    name: "WP_HYSTERESE_DHW",
    label: MultilingualStr {
        de: "Hysterese WW-Temperatur",
        en: "Hysteresis dhw temperature",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_WARTEZEIT_BOH: DecParam = DecParam {
    id: 0x0692,
    name: "WP_WARTEZEIT_BOH",
    label: MultilingualStr {
        de: "Verzögerungszeit Boosterheater",
        en: "Timer BOH",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_SMART_GRID: DecParam = DecParam {
    id: 0x0693,
    name: "WP_SMART_GRID",
    label: MultilingualStr {
        de: "Funktion Smart Grid",
        en: "SMART GRID",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_MODUS_SMART_GRID: DecParam = DecParam {
    id: 0x0694,
    name: "WP_MODUS_SMART_GRID",
    label: MultilingualStr {
        de: "Modus Smart Grid",
        en: "Mode SG",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_FLUESTERBETRIEB: DecParam = DecParam {
    id: 0x0696,
    name: "WP_FLUESTERBETRIEB",
    label: MultilingualStr {
        de: "Flüsterbetrieb",
        en: "Quite mode",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_INNENGERAET: DecParam = DecParam {
    id: 0x0699,
    name: "WP_INNENGERAET",
    label: MultilingualStr {
        de: "eWP_INNENGERAET",
        en: "eWP_INNENGERAET",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_AUSSENGERAET: DecParam = DecParam {
    id: 0x069a,
    name: "WP_AUSSENGERAET",
    label: MultilingualStr {
        de: "eWP_AUSSENGERAET",
        en: "eWP_AUSSENGERAET",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const MISCHERSTELLUNG_1: I8Param = I8Param {
    id: 0x069b,
    name: "MISCHERSTELLUNG_1",
    label: MultilingualStr {
        de: "Mischerstellung 1 [3UV1]",
        en: "Mixer Position 1 [3UV1]",
    },
    mutable: false,
    unit: Some(Unit::Percent),
    default: None,
    min: None,
    max: None,
};
pub const ENERGIE_WP_KUEHLUNG: I16Param = I16Param {
    id: 0x06a6,
    name: "ENERGIE_WP_KUEHLUNG",
    label: MultilingualStr {
        de: "Energie WP Kühlung",
        en: "Energy HP cooling",
    },
    mutable: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGIE_WP_HEIZUNG: I16Param = I16Param {
    id: 0x06a7,
    name: "ENERGIE_WP_HEIZUNG",
    label: MultilingualStr {
        de: "Energie WP Heizung",
        en: "Energy HP heating",
    },
    mutable: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const WP_SOLLWERT_ANPASSUNG_HEIZEN: DecParam = DecParam {
    id: 0x06a0,
    name: "WP_SOLLWERT_ANPASSUNG_HEIZEN",
    label: MultilingualStr {
        de: "eWP_SOLLWERT_ANPASSUNG_HEIZEN",
        en: "eWP_SOLLWERT_ANPASSUNG_HEIZEN",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WP_SOLLWERT_ANPASSUNG_KUEHLEN: DecParam = DecParam {
    id: 0x06a1,
    name: "WP_SOLLWERT_ANPASSUNG_KUEHLEN",
    label: MultilingualStr {
        de: "eWP_SOLLWERT_ANPASSUNG_KUEHLEN",
        en: "eWP_SOLLWERT_ANPASSUNG_KUEHLEN",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const PUMPENLAUFZEIT: I16Param = I16Param {
    id: 0x06a4,
    name: "PUMPENLAUFZEIT",
    label: MultilingualStr {
        de: "Pumpenlaufzeit",
        en: "Pump uptime",
    },
    mutable: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const KOMPRESSORLAUFZEIT: I16Param = I16Param {
    id: 0x06a5,
    name: "KOMPRESSORLAUFZEIT",
    label: MultilingualStr {
        de: "Kompressorlaufzeit",
        en: "Compressor uptime",
    },
    mutable: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const WASSER_SOLLDRUCK: DecParam = DecParam {
    id: 0x0725,
    name: "WASSER_SOLLDRUCK",
    label: MultilingualStr {
        de: "eWASSER_SOLLDRUCK",
        en: "eWASSER_SOLLDRUCK",
    },
    mutable: true,
    unit: None,
    scale: 3,
    default: None,
    min: None,
    max: None,
};
pub const WASSER_MAX_DRUCKVERLUST: DecParam = DecParam {
    id: 0x0726,
    name: "WASSER_MAX_DRUCKVERLUST",
    label: MultilingualStr {
        de: "eWASSER_MAX_DRUCKVERLUST",
        en: "eWASSER_MAX_DRUCKVERLUST",
    },
    mutable: true,
    unit: None,
    scale: 3,
    default: None,
    min: None,
    max: None,
};
pub const WASSER_MAXIMALDRUCK: DecParam = DecParam {
    id: 0x0727,
    name: "WASSER_MAXIMALDRUCK",
    label: MultilingualStr {
        de: "eWASSER_MAXIMALDRUCK",
        en: "eWASSER_MAXIMALDRUCK",
    },
    mutable: true,
    unit: None,
    scale: 3,
    default: None,
    min: None,
    max: None,
};
pub const WASSER_MINIMALDRUCK: DecParam = DecParam {
    id: 0x0728,
    name: "WASSER_MINIMALDRUCK",
    label: MultilingualStr {
        de: "eWASSER_MINIMALDRUCK",
        en: "eWASSER_MINIMALDRUCK",
    },
    mutable: true,
    unit: None,
    scale: 3,
    default: None,
    min: None,
    max: None,
};
pub const ENERGIE_EXT_QUELLE_WARMWASSER: I16Param = I16Param {
    id: 0x091c,
    name: "ENERGIE_EXT_QUELLE_WARMWASSER",
    label: MultilingualStr {
        de: "Energie ext. Quelle Warmwasser",
        en: "Energy ext. source warm water",
    },
    mutable: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGIE_WARMWASSER: I16Param = I16Param {
    id: 0x092c,
    name: "ENERGIE_WARMWASSER",
    label: MultilingualStr {
        de: "Energie Warmwasser",
        en: "Energy warm water",
    },
    mutable: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGIE_WP_GESAMT: I16Param = I16Param {
    id: 0x0930,
    name: "ENERGIE_WP_GESAMT",
    label: MultilingualStr {
        de: "Energie WP gesamt",
        en: "Energy HP total",
    },
    mutable: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const FROSTSCHUTZTEMP: DecParam = DecParam {
    id: 0x0a00,
    name: "FROSTSCHUTZTEMP",
    label: MultilingualStr {
        de: "Frostschutztemperatur",
        en: "T-Frost Protect",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const EINSTELL_SPEICHERSOLLTEMP2: DecParam = DecParam {
    id: 0x0a06,
    name: "EINSTELL_SPEICHERSOLLTEMP2",
    label: MultilingualStr {
        de: "Warmwasser Speichersolltemperatur 2",
        en: "DHW setpoint 2",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: Some(dec!(48.0)),
    min: Some(dec!(10.0)),
    max: Some(dec!(70.0)),
};
pub const AUSSENTEMP_WAERMEPUMPE: DecParam = DecParam {
    id: 0x0a0c,
    name: "AUSSENTEMP_WAERMEPUMPE",
    label: MultilingualStr {
        de: "Außentemperatur Wärmepumpe",
        en: "Outdoortemperature Heatpump",
    },
    mutable: false,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const ZEITMASTER: I8Param = I8Param {
    id: 0x0a1f,
    name: "ZEITMASTER",
    label: MultilingualStr {
        de: "eZEITMASTER",
        en: "eZEITMASTER",
    },
    mutable: true,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const FEIERTAGENDE_JAHR: I16Param = I16Param {
    id: 0x1355,
    name: "FEIERTAGENDE_JAHR",
    label: MultilingualStr {
        de: "eFEIERTAGENDE_JAHR",
        en: "eFEIERTAGENDE_JAHR",
    },
    mutable: true,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const MODUS_PARTY_DAUER: I16Param = I16Param {
    id: 0x1358,
    name: "MODUS_PARTY_DAUER",
    label: MultilingualStr {
        de: "Party Dauer",
        en: "Party duration",
    },
    mutable: true,
    unit: None,
    default: Some(0),
    min: Some(0),
    max: Some(360),
};
pub const KUEHLSOLLWERT_KORR_HZK_0: DecParam = DecParam {
    id: 0x1359,
    name: "KUEHLSOLLWERT_KORR_HZK_0",
    label: MultilingualStr {
        de: "eKUEHLSOLLWERT_KORR_HZK_0",
        en: "eKUEHLSOLLWERT_KORR_HZK_0",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const MAX_KUEHLEN_AUSSENTEMP_HZK0: DecParam = DecParam {
    id: 0x135c,
    name: "MAX_KUEHLEN_AUSSENTEMP_HZK0",
    label: MultilingualStr {
        de: "Obergrenze Außentemperatur für Kühlbetrieb",
        en: "Max T-Out Cooling",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const VL_SOLL_START_KUEHLEN_HZK_0: DecParam = DecParam {
    id: 0x135d,
    name: "VL_SOLL_START_KUEHLEN_HZK_0",
    label: MultilingualStr {
        de: "Maximaltemperatur im Kühlbetrieb",
        en: "Max T-Out Cooling",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const VL_SOLL_MAX_KUEHLEN_HZK0: DecParam = DecParam {
    id: 0x135e,
    name: "VL_SOLL_MAX_KUEHLEN_HZK0",
    label: MultilingualStr {
        de: "Minimal mögliche Vorlauftemperatur für Kühlberieb",
        en: "Min T-Flow Cooling",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const FEHLER_AKTUELL: I16Param = I16Param {
    id: 0x1388,
    name: "FEHLER_AKTUELL",
    label: MultilingualStr {
        de: "Aktueller Fehler",
        en: "Current error",
    },
    mutable: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const START_KUEHLEN_AUSSENTEMP_HZK0: DecParam = DecParam {
    id: 0x13b5,
    name: "START_KUEHLEN_AUSSENTEMP_HZK0",
    label: MultilingualStr {
        de: "Untergrenze Außentemperatur für Kühlbetrieb",
        en: "Start T-Out Cooling",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const HEIZPROG_1: TimeRangeParam = TimeRangeParam {
    id: 0x1400,
    name: "HEIZPROG_1",
    label: MultilingualStr {
        de: "Heizprogramm 1",
        en: "Heating program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_MO: TimeRangeParam = TimeRangeParam {
    id: 0x1410,
    name: "HEIZPROG_1_MO",
    label: MultilingualStr {
        de: "Heizprogramm 1 Montag Schaltzeit 1",
        en: "Heating program 1 monday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_MO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1411,
    name: "HEIZPROG_1_MO_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 1 Montag Schaltzeit 2",
        en: "Heating program 1 monday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_MO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1412,
    name: "HEIZPROG_1_MO_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 1 Montag Schaltzeit 3",
        en: "Heating program 1 monday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_DI: TimeRangeParam = TimeRangeParam {
    id: 0x1420,
    name: "HEIZPROG_1_DI",
    label: MultilingualStr {
        de: "Heizprogramm 1 Dienstag Schaltzeit 1",
        en: "Heating program 1 tuesday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_DI_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1421,
    name: "HEIZPROG_1_DI_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 1 Dienstag Schaltzeit 2",
        en: "Heating program 1 tuesday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_DI_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1422,
    name: "HEIZPROG_1_DI_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 1 Dienstag Schaltzeit 3",
        en: "Heating program 1 tuesday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_MI: TimeRangeParam = TimeRangeParam {
    id: 0x1430,
    name: "HEIZPROG_1_MI",
    label: MultilingualStr {
        de: "Heizprogramm 1 Mittwoch Schaltzeit 1",
        en: "Heating program 1 wednesday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_MI_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1431,
    name: "HEIZPROG_1_MI_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 1 Mittwoch Schaltzeit 2",
        en: "Heating program 1 wednesday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_MI_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1432,
    name: "HEIZPROG_1_MI_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 1 Mittwoch Schaltzeit 3",
        en: "Heating program 1 wednesday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_DO: TimeRangeParam = TimeRangeParam {
    id: 0x1440,
    name: "HEIZPROG_1_DO",
    label: MultilingualStr {
        de: "Heizprogramm 1 Donnerstag Schaltzeit 1",
        en: "Heating program 1 thursday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_DO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1441,
    name: "HEIZPROG_1_DO_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 1 Donnerstag Schaltzeit 2",
        en: "Heating program 1 thursday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_DO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1442,
    name: "HEIZPROG_1_DO_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 1 Donnerstag Schaltzeit 3",
        en: "Heating program 1 thursday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_FR: TimeRangeParam = TimeRangeParam {
    id: 0x1450,
    name: "HEIZPROG_1_FR",
    label: MultilingualStr {
        de: "Heizprogramm 1 Freitag Schaltzeit 1",
        en: "Heating program 1 friday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_FR_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1451,
    name: "HEIZPROG_1_FR_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 1 Freitag Schaltzeit 2",
        en: "Heating program 1 friday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_FR_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1452,
    name: "HEIZPROG_1_FR_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 1 Freitag Schaltzeit 3",
        en: "Heating program 1 friday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_SA: TimeRangeParam = TimeRangeParam {
    id: 0x1460,
    name: "HEIZPROG_1_SA",
    label: MultilingualStr {
        de: "Heizprogramm 1 Samstag Schaltzeit 1",
        en: "Heating program 1 saturday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_SA_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1461,
    name: "HEIZPROG_1_SA_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 1 Samstag Schaltzeit 2",
        en: "Heating program 1 saturday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_SA_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1462,
    name: "HEIZPROG_1_SA_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 1 Samstag Schaltzeit 3",
        en: "Heating program 1 saturday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_SO: TimeRangeParam = TimeRangeParam {
    id: 0x1470,
    name: "HEIZPROG_1_SO",
    label: MultilingualStr {
        de: "Heizprogramm 1 Sonntag Schaltzeit 1",
        en: "Heating program 1 sunday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_SO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1471,
    name: "HEIZPROG_1_SO_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 1 Sonntag Schaltzeit 2",
        en: "Heating program 1 sunday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_SO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1472,
    name: "HEIZPROG_1_SO_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 1 Sonntag Schaltzeit 3",
        en: "Heating program 1 sunday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_MO_FR: TimeRangeParam = TimeRangeParam {
    id: 0x1480,
    name: "HEIZPROG_1_MO_FR",
    label: MultilingualStr {
        de: "Heizprogramm 1 Montag bis Freitag Schaltzeit 1",
        en: "Heating program 1 monday to friday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_MO_FR_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1481,
    name: "HEIZPROG_1_MO_FR_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 1 Montag bis Freitag Schaltzeit 2",
        en: "Heating program 1 monday to friday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_MO_FR_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1482,
    name: "HEIZPROG_1_MO_FR_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 1 Montag bis Freitag Schaltzeit 3",
        en: "Heating program 1 monday to friday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_SA_SO: TimeRangeParam = TimeRangeParam {
    id: 0x1490,
    name: "HEIZPROG_1_SA_SO",
    label: MultilingualStr {
        de: "Heizprogramm 1 Samstag und Sonntag Schaltzeit 1",
        en: "Heating program 1 saturday to sunday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_SA_SO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1491,
    name: "HEIZPROG_1_SA_SO_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 1 Samstag und Sonntag Schaltzeit 2",
        en: "Heating program 1 saturday to sunday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_SA_SO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1492,
    name: "HEIZPROG_1_SA_SO_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 1 Samstag und Sonntag Schaltzeit 3",
        en: "Heating program 1 saturday to sunday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_MO_SO: TimeRangeParam = TimeRangeParam {
    id: 0x14a0,
    name: "HEIZPROG_1_MO_SO",
    label: MultilingualStr {
        de: "Heizprogramm 1 Montag bis Sonntag Schaltzeit 1",
        en: "Heating program 1 monday to sunday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_MO_SO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x14a1,
    name: "HEIZPROG_1_MO_SO_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 1 Montag bis Sonntag Schaltzeit 2",
        en: "Heating program 1 monday to sunday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_MO_SO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x14a2,
    name: "HEIZPROG_1_MO_SO_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 1 Montag bis Sonntag Schaltzeit 3",
        en: "Heating program 1 monday to sunday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_MO_DO: TimeRangeParam = TimeRangeParam {
    id: 0x14b0,
    name: "HEIZPROG_1_MO_DO",
    label: MultilingualStr {
        de: "Heizprogramm 1 Montag bis Donnerstag Schaltzeit 1",
        en: "Heating program 1 monday to thursday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_MO_DO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x14b1,
    name: "HEIZPROG_1_MO_DO_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 1 Montag bis Donnerstag Schaltzeit 2",
        en: "Heating program 1 monday to thursday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_1_MO_DO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x14b2,
    name: "HEIZPROG_1_MO_DO_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 1 Montag bis Donnerstag Schaltzeit 3",
        en: "Heating program 1 monday to thursday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2: TimeRangeParam = TimeRangeParam {
    id: 0x1500,
    name: "HEIZPROG_2",
    label: MultilingualStr {
        de: "Heizprogramm 2",
        en: "Heating program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_MO: TimeRangeParam = TimeRangeParam {
    id: 0x1510,
    name: "HEIZPROG_2_MO",
    label: MultilingualStr {
        de: "Heizprogramm 2 Montag Schaltzeit 1",
        en: "Heating program 2 monday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_MO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1511,
    name: "HEIZPROG_2_MO_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 2 Montag Schaltzeit 2",
        en: "Heating program 2 monday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_MO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1512,
    name: "HEIZPROG_2_MO_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 2 Montag Schaltzeit 3",
        en: "Heating program 2 monday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_DI: TimeRangeParam = TimeRangeParam {
    id: 0x1520,
    name: "HEIZPROG_2_DI",
    label: MultilingualStr {
        de: "Heizprogramm 2 Dienstag Schaltzeit 1",
        en: "Heating program 2 tuesday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_DI_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1521,
    name: "HEIZPROG_2_DI_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 2 Dienstag Schaltzeit 2",
        en: "Heating program 2 tuesday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_DI_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1522,
    name: "HEIZPROG_2_DI_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 2 Dienstag Schaltzeit 3",
        en: "Heating program 2 tuesday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_MI: TimeRangeParam = TimeRangeParam {
    id: 0x1530,
    name: "HEIZPROG_2_MI",
    label: MultilingualStr {
        de: "Heizprogramm 2 Mittwoch Schaltzeit 1",
        en: "Heating program 2 wednesday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_MI_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1531,
    name: "HEIZPROG_2_MI_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 2 Mittwoch Schaltzeit 2",
        en: "Heating program 2 wednesday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_MI_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1532,
    name: "HEIZPROG_2_MI_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 2 Mittwoch Schaltzeit 3",
        en: "Heating program 2 wednesday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_DO: TimeRangeParam = TimeRangeParam {
    id: 0x1540,
    name: "HEIZPROG_2_DO",
    label: MultilingualStr {
        de: "Heizprogramm 2 Donnerstag Schaltzeit 1",
        en: "Heating program 2 thursday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_DO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1541,
    name: "HEIZPROG_2_DO_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 2 Donnerstag Schaltzeit 2",
        en: "Heating program 2 thursday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_DO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1542,
    name: "HEIZPROG_2_DO_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 2 Donnerstag Schaltzeit 3",
        en: "Heating program 2 thursday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_FR: TimeRangeParam = TimeRangeParam {
    id: 0x1550,
    name: "HEIZPROG_2_FR",
    label: MultilingualStr {
        de: "Heizprogramm 2 Freitag Schaltzeit 1",
        en: "Heating program 2 friday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_FR_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1551,
    name: "HEIZPROG_2_FR_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 2 Freitag Schaltzeit 2",
        en: "Heating program 2 friday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_FR_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1552,
    name: "HEIZPROG_2_FR_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 2 Freitag Schaltzeit 3",
        en: "Heating program 2 friday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_SA: TimeRangeParam = TimeRangeParam {
    id: 0x1560,
    name: "HEIZPROG_2_SA",
    label: MultilingualStr {
        de: "Heizprogramm 2 Samstag Schaltzeit 1",
        en: "Heating program 2 saturday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_SA_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1561,
    name: "HEIZPROG_2_SA_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 2 Samstag Schaltzeit 2",
        en: "Heating program 2 saturday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_SA_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1562,
    name: "HEIZPROG_2_SA_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 2 Samstag Schaltzeit 3",
        en: "Heating program 2 saturday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_SO: TimeRangeParam = TimeRangeParam {
    id: 0x1570,
    name: "HEIZPROG_2_SO",
    label: MultilingualStr {
        de: "Heizprogramm 2 Sonntag Schaltzeit 1",
        en: "Heating program 2 sunday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_SO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1571,
    name: "HEIZPROG_2_SO_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 2 Sonntag Schaltzeit 2",
        en: "Heating program 2 sunday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_SO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1572,
    name: "HEIZPROG_2_SO_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 2 Sonntag Schaltzeit 3",
        en: "Heating program 2 sunday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_MO_FR: TimeRangeParam = TimeRangeParam {
    id: 0x1580,
    name: "HEIZPROG_2_MO_FR",
    label: MultilingualStr {
        de: "Heizprogramm 2 Montag bis Freitag Schaltzeit 1",
        en: "Heating program 2 monday to friday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_MO_FR_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1581,
    name: "HEIZPROG_2_MO_FR_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 2 Montag bis Freitag Schaltzeit 2",
        en: "Heating program 2 monday to friday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_MO_FR_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1582,
    name: "HEIZPROG_2_MO_FR_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 2 Montag bis Freitag Schaltzeit 3",
        en: "Heating program 2 monday to friday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_SA_SO: TimeRangeParam = TimeRangeParam {
    id: 0x1590,
    name: "HEIZPROG_2_SA_SO",
    label: MultilingualStr {
        de: "Heizprogramm 2 Samstag und Sonntag Schaltzeit 1",
        en: "Heating program 2 saturday to sunday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_SA_SO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1591,
    name: "HEIZPROG_2_SA_SO_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 2 Samstag und Sonntag Schaltzeit 2",
        en: "Heating program 2 saturday to sunday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_SA_SO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1592,
    name: "HEIZPROG_2_SA_SO_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 2 Samstag und Sonntag Schaltzeit 3",
        en: "Heating program 2 saturday to sunday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_MO_SO: TimeRangeParam = TimeRangeParam {
    id: 0x15a0,
    name: "HEIZPROG_2_MO_SO",
    label: MultilingualStr {
        de: "Heizprogramm 2 Montag bis Sonntag Schaltzeit 1",
        en: "Heating program 2 monday to sunday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_MO_SO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x15a1,
    name: "HEIZPROG_2_MO_SO_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 2 Montag bis Sonntag Schaltzeit 2",
        en: "Heating program 2 monday to sunday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_MO_SO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x15a2,
    name: "HEIZPROG_2_MO_SO_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 2 Montag bis Sonntag Schaltzeit 3",
        en: "Heating program 2 monday to sunday time program 3",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_MO_DO: TimeRangeParam = TimeRangeParam {
    id: 0x15b0,
    name: "HEIZPROG_2_MO_DO",
    label: MultilingualStr {
        de: "Heizprogramm 2 Montag bis Donnerstag Schaltzeit 1",
        en: "Heating program 2 monday to thursday time program 1",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_MO_DO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x15b1,
    name: "HEIZPROG_2_MO_DO_SCHALT_2",
    label: MultilingualStr {
        de: "Heizprogramm 2 Montag bis Donnerstag Schaltzeit 2",
        en: "Heating program 2 monday to thursday time program 2",
    },
    mutable: true,
    default: None,
};
pub const HEIZPROG_2_MO_DO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x15b2,
    name: "HEIZPROG_2_MO_DO_SCHALT_3",
    label: MultilingualStr {
        de: "Heizprogramm 2 Montag bis Donnerstag Schaltzeit 3",
        en: "Heating program 2 monday to thursday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1: TimeRangeParam = TimeRangeParam {
    id: 0x1700,
    name: "W_WASSERPROG_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1",
        en: "DHW program 1",
    },
    mutable: false,
    default: None,
};
pub const W_WASSERPROG_1_MO: TimeRangeParam = TimeRangeParam {
    id: 0x1710,
    name: "W_WASSERPROG_1_MO",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Montag Schaltzeit 1",
        en: "DHW program 1 monday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_MO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1711,
    name: "W_WASSERPROG_1_MO_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Montag Schaltzeit 2",
        en: "DHW program 1 monday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_MO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1712,
    name: "W_WASSERPROG_1_MO_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Montag Schaltzeit 3",
        en: "DHW program 1 monday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_DI: TimeRangeParam = TimeRangeParam {
    id: 0x1720,
    name: "W_WASSERPROG_1_DI",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Dienstag Schaltzeit 1",
        en: "DHW program 1 tuesday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_DI_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1721,
    name: "W_WASSERPROG_1_DI_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Dienstag Schaltzeit 2",
        en: "DHW program 1 tuesday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_DI_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1722,
    name: "W_WASSERPROG_1_DI_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Dienstag Schaltzeit 3",
        en: "DHW program 1 tuesday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_MI: TimeRangeParam = TimeRangeParam {
    id: 0x1730,
    name: "W_WASSERPROG_1_MI",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Mittwoch Schaltzeit 1",
        en: "DHW program 1 wednesday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_MI_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1731,
    name: "W_WASSERPROG_1_MI_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Mittwoch Schaltzeit 2",
        en: "DHW program 1 wednesday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_MI_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1732,
    name: "W_WASSERPROG_1_MI_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Mittwoch Schaltzeit 3",
        en: "DHW program 1 wednesday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_DO: TimeRangeParam = TimeRangeParam {
    id: 0x1740,
    name: "W_WASSERPROG_1_DO",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Donnerstag Schaltzeit 1",
        en: "DHW program 1 thursday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_DO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1741,
    name: "W_WASSERPROG_1_DO_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Donnerstag Schaltzeit 2",
        en: "DHW program 1 thursday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_DO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1742,
    name: "W_WASSERPROG_1_DO_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Donnerstag Schaltzeit 3",
        en: "DHW program 1 thursday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_FR: TimeRangeParam = TimeRangeParam {
    id: 0x1750,
    name: "W_WASSERPROG_1_FR",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Freitag Schaltzeit 1",
        en: "DHW program 1 friday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_FR_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1751,
    name: "W_WASSERPROG_1_FR_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Freitag Schaltzeit 2",
        en: "DHW program 1 friday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_FR_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1752,
    name: "W_WASSERPROG_1_FR_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Freitag Schaltzeit 3",
        en: "DHW program 1 friday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_SA: TimeRangeParam = TimeRangeParam {
    id: 0x1760,
    name: "W_WASSERPROG_1_SA",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Samstag Schaltzeit 1",
        en: "DHW program 1 saturday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_SA_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1761,
    name: "W_WASSERPROG_1_SA_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Samstag Schaltzeit 2",
        en: "DHW program 1 saturday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_SA_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1762,
    name: "W_WASSERPROG_1_SA_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Samstag Schaltzeit 3",
        en: "DHW program 1 saturday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_SO: TimeRangeParam = TimeRangeParam {
    id: 0x1770,
    name: "W_WASSERPROG_1_SO",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Sonntag Schaltzeit 1",
        en: "DHW program 1 sunday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_SO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1771,
    name: "W_WASSERPROG_1_SO_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Sonntag Schaltzeit 2",
        en: "DHW program 1 sunday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_SO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1772,
    name: "W_WASSERPROG_1_SO_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Sonntag Schaltzeit 3",
        en: "DHW program 1 sunday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_MO_FR: TimeRangeParam = TimeRangeParam {
    id: 0x1780,
    name: "W_WASSERPROG_1_MO_FR",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Montag bis Freitag Schaltzeit 1",
        en: "DHW program 1 monday to friday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_MO_FR_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1781,
    name: "W_WASSERPROG_1_MO_FR_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Montag bis Freitag Schaltzeit 2",
        en: "DHW program 1 monday to friday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_MO_FR_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1782,
    name: "W_WASSERPROG_1_MO_FR_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Montag bis Freitag Schaltzeit 3",
        en: "DHW program 1 monday to friday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_SA_SO: TimeRangeParam = TimeRangeParam {
    id: 0x1790,
    name: "W_WASSERPROG_1_SA_SO",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Samstag und Sonntag Schaltzeit 1",
        en: "DHW program 1 saturday to sunday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_SA_SO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1791,
    name: "W_WASSERPROG_1_SA_SO_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Samstag und Sonntag Schaltzeit 2",
        en: "DHW program 1 saturday to sunday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_SA_SO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1792,
    name: "W_WASSERPROG_1_SA_SO_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Samstag und Sonntag Schaltzeit 3",
        en: "DHW program 1 saturday to sunday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_MO_SO: TimeRangeParam = TimeRangeParam {
    id: 0x17a0,
    name: "W_WASSERPROG_1_MO_SO",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Montag bis Sonntag Schaltzeit 1",
        en: "DHW program 1 monday to sunday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_MO_SO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x17a1,
    name: "W_WASSERPROG_1_MO_SO_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Montag bis Sonntag Schaltzeit 2",
        en: "DHW program 1 monday to sunday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_MO_SO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x17a2,
    name: "W_WASSERPROG_1_MO_SO_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Montag bis Sonntag Schaltzeit 3",
        en: "DHW program 1 monday to sunday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_MO_DO: TimeRangeParam = TimeRangeParam {
    id: 0x17b0,
    name: "W_WASSERPROG_1_MO_DO",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Montag bis Donnerstag Schaltzeit 1",
        en: "DHW program 1 monday to thursday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_MO_DO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x17b1,
    name: "W_WASSERPROG_1_MO_DO_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Montag bis Donnerstag Schaltzeit 2",
        en: "DHW program 1 monday to thursday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_1_MO_DO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x17b2,
    name: "W_WASSERPROG_1_MO_DO_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 Montag bis Donnerstag Schaltzeit 3",
        en: "DHW program 1 monday to thursday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2: TimeRangeParam = TimeRangeParam {
    id: 0x1800,
    name: "W_WASSERPROG_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2",
        en: "DHW program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_MO: TimeRangeParam = TimeRangeParam {
    id: 0x1810,
    name: "W_WASSERPROG_2_MO",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Montag Schaltzeit 1",
        en: "DHW program 2 monday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_MO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1811,
    name: "W_WASSERPROG_2_MO_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Montag Schaltzeit 2",
        en: "DHW program 2 monday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_MO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1812,
    name: "W_WASSERPROG_2_MO_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Montag Schaltzeit 3",
        en: "DHW program 2 monday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_DI: TimeRangeParam = TimeRangeParam {
    id: 0x1820,
    name: "W_WASSERPROG_2_DI",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Dienstag Schaltzeit 1",
        en: "DHW program 2 tuesday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_DI_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1821,
    name: "W_WASSERPROG_2_DI_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Dienstag Schaltzeit 2",
        en: "DHW program 2 tuesday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_DI_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1822,
    name: "W_WASSERPROG_2_DI_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Dienstag Schaltzeit 3",
        en: "DHW program 2 tuesday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_MI: TimeRangeParam = TimeRangeParam {
    id: 0x1830,
    name: "W_WASSERPROG_2_MI",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Mittwoch Schaltzeit 1",
        en: "DHW program 2 wednesday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_MI_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1831,
    name: "W_WASSERPROG_2_MI_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Mittwoch Schaltzeit 2",
        en: "DHW program 2 wednesday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_MI_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1832,
    name: "W_WASSERPROG_2_MI_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Mittwoch Schaltzeit 3",
        en: "DHW program 2 wednesday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_DO: TimeRangeParam = TimeRangeParam {
    id: 0x1840,
    name: "W_WASSERPROG_2_DO",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Donnerstag Schaltzeit 1",
        en: "DHW program 2 thursday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_DO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1841,
    name: "W_WASSERPROG_2_DO_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Donnerstag Schaltzeit 2",
        en: "DHW program 2 thursday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_DO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1842,
    name: "W_WASSERPROG_2_DO_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Donnerstag Schaltzeit 3",
        en: "DHW program 2 thursday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_FR: TimeRangeParam = TimeRangeParam {
    id: 0x1850,
    name: "W_WASSERPROG_2_FR",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Freitag Schaltzeit 1",
        en: "DHW program 2 friday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_FR_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1851,
    name: "W_WASSERPROG_2_FR_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Freitag Schaltzeit 2",
        en: "DHW program 2 friday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_FR_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1852,
    name: "W_WASSERPROG_2_FR_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Freitag Schaltzeit 3",
        en: "DHW program 2 friday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_SA: TimeRangeParam = TimeRangeParam {
    id: 0x1860,
    name: "W_WASSERPROG_2_SA",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Samstag Schaltzeit 1",
        en: "DHW program 2 saturday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_SA_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1861,
    name: "W_WASSERPROG_2_SA_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Samstag Schaltzeit 2",
        en: "DHW program 2 saturday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_SA_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1862,
    name: "W_WASSERPROG_2_SA_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Samstag Schaltzeit 3",
        en: "DHW program 2 saturday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_SO: TimeRangeParam = TimeRangeParam {
    id: 0x1870,
    name: "W_WASSERPROG_2_SO",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Sonntag Schaltzeit 1",
        en: "DHW program 2 sunday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_SO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1871,
    name: "W_WASSERPROG_2_SO_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Sonntag Schaltzeit 2",
        en: "DHW program 2 sunday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_SO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1872,
    name: "W_WASSERPROG_2_SO_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Sonntag Schaltzeit 3",
        en: "DHW program 2 sunday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_MO_FR: TimeRangeParam = TimeRangeParam {
    id: 0x1880,
    name: "W_WASSERPROG_2_MO_FR",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Montag bis Freitag Schaltzeit 1",
        en: "DHW program 2 monday to friday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_MO_FR_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1881,
    name: "W_WASSERPROG_2_MO_FR_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Montag bis Freitag Schaltzeit 2",
        en: "DHW program 2 monday to friday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_MO_FR_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1882,
    name: "W_WASSERPROG_2_MO_FR_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Montag bis Freitag Schaltzeit 3",
        en: "DHW program 2 monday to friday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_SA_SO: TimeRangeParam = TimeRangeParam {
    id: 0x1890,
    name: "W_WASSERPROG_2_SA_SO",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Samstag und Sonntag Schaltzeit 1",
        en: "DHW program 2 saturday to sunday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_SA_SO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1891,
    name: "W_WASSERPROG_2_SA_SO_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Samstag und Sonntag Schaltzeit 2",
        en: "DHW program 2 saturday to sunday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_SA_SO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1892,
    name: "W_WASSERPROG_2_SA_SO_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Samstag und Sonntag Schaltzeit 3",
        en: "DHW program 2 saturday to sunday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_MO_SO: TimeRangeParam = TimeRangeParam {
    id: 0x18a0,
    name: "W_WASSERPROG_2_MO_SO",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Montag bis Sonntag Schaltzeit 1",
        en: "DHW program 2 monday to sunday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_MO_SO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x18a1,
    name: "W_WASSERPROG_2_MO_SO_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Montag bis Sonntag Schaltzeit 2",
        en: "DHW program 2 monday to sunday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_MO_SO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x18a2,
    name: "W_WASSERPROG_2_MO_SO_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Montag bis Sonntag Schaltzeit 3",
        en: "DHW program 2 monday to sunday time program 3",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_MO_DO: TimeRangeParam = TimeRangeParam {
    id: 0x18b0,
    name: "W_WASSERPROG_2_MO_DO",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Montag bis Donnerstag Schaltzeit 1",
        en: "DHW program 2 monday to thursday time program 1",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_MO_DO_SCHALT_2: TimeRangeParam = TimeRangeParam {
    id: 0x18b1,
    name: "W_WASSERPROG_2_MO_DO_SCHALT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Montag bis Donnerstag Schaltzeit 2",
        en: "DHW program 2 monday to thursday time program 2",
    },
    mutable: true,
    default: None,
};
pub const W_WASSERPROG_2_MO_DO_SCHALT_3: TimeRangeParam = TimeRangeParam {
    id: 0x18b2,
    name: "W_WASSERPROG_2_MO_DO_SCHALT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 Montag bis Donnerstag Schaltzeit 3",
        en: "DHW program 2 monday to thursday time program 3",
    },
    mutable: true,
    default: None,
};
pub const VMIN_GCU: DecParam = DecParam {
    id: 0xc09d,
    name: "VMIN_GCU",
    label: MultilingualStr {
        de: "Durchfluss",
        en: "Volume flow",
    },
    mutable: false,
    unit: None,
    scale: 0,
    default: None,
    min: None,
    max: None,
};
pub const SONDERFKT_SCHALTKONTAKT: DecParam = DecParam {
    id: 0xc0b1,
    name: "SONDERFKT_SCHALTKONTAKT",
    label: MultilingualStr {
        de: "eSONDERFKT_SCHALTKONTAKT",
        en: "eSONDERFKT_SCHALTKONTAKT",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WARTEZEIT_SONDERFKT: DecParam = DecParam {
    id: 0xc0b2,
    name: "WARTEZEIT_SONDERFKT",
    label: MultilingualStr {
        de: "eWARTEZEIT_SONDERFKT",
        en: "eWARTEZEIT_SONDERFKT",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const SCHALTSCHWELLE_TDHW: DecParam = DecParam {
    id: 0xc0b3,
    name: "SCHALTSCHWELLE_TDHW",
    label: MultilingualStr {
        de: "eSCHALTSCHWELLE_TDHW",
        en: "eSCHALTSCHWELLE_TDHW",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const BETRIEBSART: Enum16Param<enums::Betriebsart> = Enum16Param {
    id: 0xc0f6,
    name: "BETRIEBSART",
    label: MultilingualStr {
        de: "Betriebsart",
        en: "Betriebsart",
    },
    mutable: false,
    default: None,
    values: phf::phf_ordered_map! { 0i16 => "Standby" , 1i16 => "Heizen" , 2i16 => "Kühlen" , 3i16 => "Abtauen" , 4i16 => "Warmwasserbereitung" },
};
pub const PWM_SIGNAL: I8Param = I8Param {
    id: 0xc0f7,
    name: "PWM_SIGNAL",
    label: MultilingualStr {
        de: "PWM_SIGNAL",
        en: "PWM signal",
    },
    mutable: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const SG: Enum16Param<enums::SG> = Enum16Param {
    id: 0xc0f8,
    name: "SG",
    label: MultilingualStr {
        de: "Smart Grid Signal",
        en: "Smart Grid Signal",
    },
    mutable: false,
    default: None,
    values: phf::phf_ordered_map! { 3i16 => "SGN" , 4i16 => "SG1" , 5i16 => "SG2" , 6i16 => "SG3" },
};
pub const MISCHERSTELLUNG_2_3UVB: I8Param = I8Param {
    id: 0xc0fb,
    name: "MISCHERSTELLUNG_2_3UVB",
    label: MultilingualStr {
        de: "Mischerstellung 2 3UVB",
        en: "Mixer position 2 3UVB",
    },
    mutable: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const T_V: DecParam = DecParam {
    id: 0xc0fc,
    name: "T_V",
    label: MultilingualStr {
        de: "t-V",
        en: "t-V",
    },
    mutable: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const T_TVBHMIX: DecParam = DecParam {
    id: 0xc0fe,
    name: "T_TVBHMIX",
    label: MultilingualStr {
        de: "TVBH",
        en: "TVBH",
    },
    mutable: false,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const V: I16Param = I16Param {
    id: 0xc101,
    name: "V",
    label: MultilingualStr {
        de: "Volumenstrom",
        en: "Volume flow",
    },
    mutable: false,
    unit: Some(Unit::LitersPerHour),
    default: None,
    min: None,
    max: None,
};
pub const T_TVBH1: DecParam = DecParam {
    id: 0xc102,
    name: "T_TVBH1",
    label: MultilingualStr {
        de: "TVBH1",
        en: "TVBH1",
    },
    mutable: false,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const T_LIQ: DecParam = DecParam {
    id: 0xc103,
    name: "T_LIQ",
    label: MultilingualStr {
        de: "t-liq",
        en: "t-liq",
    },
    mutable: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const T_R: DecParam = DecParam {
    id: 0xc104,
    name: "T_R",
    label: MultilingualStr {
        de: "t-R",
        en: "t-R",
    },
    mutable: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const T_DHW: DecParam = DecParam {
    id: 0xc106,
    name: "T_DHW",
    label: MultilingualStr {
        de: "T_DHW",
        en: "t-R",
    },
    mutable: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const T_AU: DecParam = DecParam {
    id: 0xc176,
    name: "T_AU",
    label: MultilingualStr {
        de: "Außentemperatur",
        en: "Ambient temperature",
    },
    mutable: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const T_TVBH: DecParam = DecParam {
    id: 0xc1bf,
    name: "T_TVBH",
    label: MultilingualStr {
        de: "TVBH2",
        en: "TVBH2",
    },
    mutable: false,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const ANTILEG_START_ZEIT: DecParam = DecParam {
    id: 0xfd4f,
    name: "ANTILEG_START_ZEIT",
    label: MultilingualStr {
        de: "Startzeit Antileginoellenprogramm",
        en: "Anti-Legionella time",
    },
    mutable: true,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const STATUS_UMWAELZPUMPE: BoolParam = BoolParam {
    id: 0xfdac,
    name: "STATUS_UMWAELZPUMPE",
    label: MultilingualStr {
        de: "Status Umwälzpumpe",
        en: "Status circulating pump",
    },
    mutable: false,
    default: None,
};
pub static PARAMS: phf::Map<u16, &dyn Param> = phf::phf_map! { 0x0002u16 => & KESSELSOLLTEMP , 0x0003u16 => & VERSTELLTE_SPEICHERSOLLTEMP , 0x0004u16 => & VORLAUFSOLLTEMP , 0x0005u16 => & RAUMSOLLTEMP_I , 0x0006u16 => & RAUMSOLLTEMP_II , 0x0007u16 => & RAUMSOLLTEMP_III , 0x0008u16 => & NACHTRAUMSOLLTEMP , 0x000cu16 => & AUSSENTEMP , 0x000du16 => & KESSELISTTEMP , 0x000eu16 => & SPEICHERISTTEMP , 0x000fu16 => & VORLAUFISTTEMP , 0x0011u16 => & RAUMISTTEMP , 0x0012u16 => & VERSTELLTE_RAUMSOLLTEMP , 0x0013u16 => & EINSTELL_SPEICHERSOLLTEMP , 0x0016u16 => & RUECKLAUFTEMP , 0x001cu16 => & P , 0x0028u16 => & MAX_VORLAUFTEMP , 0x005eu16 => & WW_AKTIV , 0x0061u16 => & STATUS_KOMPRESSOR , 0x0103u16 => & AUFHEIZOPTIMIERUNG , 0x010eu16 => & HZKKURVE , 0x010fu16 => & RAUMEINFLUSS , 0x0110u16 => & MAX_AUFHEIZVORVERLEGUNG , 0x0112u16 => & PROGRAMMSCHALTER , 0x0115u16 => & ADAPTION , 0x0116u16 => & HEIZGRENZE_TAG , 0x0117u16 => & HEIZGRENZE_NACHT , 0x011bu16 => & MODUS_URLAUB_ANFANG_TAG , 0x011cu16 => & MODUS_URLAUB_ANFANG_MONAT , 0x011du16 => & MODUS_URLAUB_ANFANG_JAHR , 0x011eu16 => & MODUS_URLAUB_ENDE_TAG , 0x011fu16 => & MODUS_URLAUB_ENDE_MONAT , 0x0120u16 => & MODUS_URLAUB_ENDE_JAHR , 0x0122u16 => & TAG , 0x0123u16 => & MONAT , 0x0124u16 => & JAHR , 0x0125u16 => & STUNDE , 0x0126u16 => & MINUTE , 0x0129u16 => & VORLAUFSOLLTEMP_TAG , 0x012au16 => & VORLAUFSOLLTEMP_NACHT , 0x012bu16 => & MIN_VORLAUFTEMP , 0x012eu16 => & ABSENKOPTIMIERUNG , 0x013du16 => & ABWESEND_RAUMSOLLTEMP , 0x013eu16 => & EINSTELL_SPEICHERSOLLTEMP3 , 0x0141u16 => & HZK_FUNKTION , 0x0144u16 => & EINMAL_WW_AKTIV , 0x0148u16 => & GERAETE_KENNUNG , 0x0182u16 => & ZIRKPUMPE_BEI_WWFREIGABE , 0x0199u16 => & SOFTWARE_NUMMER , 0x019au16 => & SOFTWARE_VERSION , 0x01dau16 => & VOLUMENSTROM , 0x024bu16 => & SOFTWARE_UNTERINDEX , 0x0587u16 => & ANTILEG_TEMP , 0x0661u16 => & VMIN_A1 , 0x0668u16 => & LEISTUNG_WW , 0x0669u16 => & WP_LEISTUNG_HEIZSTAB_S1 , 0x066au16 => & WP_LEISTUNG_HEIZSTAB_S2 , 0x066bu16 => & WP_LEISTUNG_HZU_BIV , 0x066eu16 => & WP_MAX_TEMP_HEIZUNG , 0x066fu16 => & WP_HT_NT_FUNKTION , 0x0670u16 => & WP_HT_NT_FKT_ANSCHLUSS , 0x0678u16 => & WP_RAUMTHERMOSTAT , 0x0679u16 => & WP_INTERLINKFUNKTION , 0x067eu16 => & WP_PWM_LEISTUNG_MAX , 0x067fu16 => & WP_PWM_LEISTUNG_MIN , 0x0682u16 => & WP_MOD_HYST_DURCHFLUSS , 0x0683u16 => & WP_SPREIZUNG_HZ_BETRIEB , 0x0684u16 => & WP_SPREIZUNG_WW_BETRIEB , 0x0685u16 => & WP_VERZ_ZEIT_PUMPE , 0x0688u16 => & VMIN_WP , 0x068cu16 => & WP_START_MAX_TEMP , 0x0691u16 => & WP_HYSTERESE_DHW , 0x0692u16 => & WP_WARTEZEIT_BOH , 0x0693u16 => & WP_SMART_GRID , 0x0694u16 => & WP_MODUS_SMART_GRID , 0x0696u16 => & WP_FLUESTERBETRIEB , 0x0699u16 => & WP_INNENGERAET , 0x069au16 => & WP_AUSSENGERAET , 0x069bu16 => & MISCHERSTELLUNG_1 , 0x06A6u16 => & ENERGIE_WP_KUEHLUNG , 0x06A7u16 => & ENERGIE_WP_HEIZUNG , 0x06a0u16 => & WP_SOLLWERT_ANPASSUNG_HEIZEN , 0x06a1u16 => & WP_SOLLWERT_ANPASSUNG_KUEHLEN , 0x06a4u16 => & PUMPENLAUFZEIT , 0x06a5u16 => & KOMPRESSORLAUFZEIT , 0x0725u16 => & WASSER_SOLLDRUCK , 0x0726u16 => & WASSER_MAX_DRUCKVERLUST , 0x0727u16 => & WASSER_MAXIMALDRUCK , 0x0728u16 => & WASSER_MINIMALDRUCK , 0x091cu16 => & ENERGIE_EXT_QUELLE_WARMWASSER , 0x092cu16 => & ENERGIE_WARMWASSER , 0x0930u16 => & ENERGIE_WP_GESAMT , 0x0a00u16 => & FROSTSCHUTZTEMP , 0x0a06u16 => & EINSTELL_SPEICHERSOLLTEMP2 , 0x0a0cu16 => & AUSSENTEMP_WAERMEPUMPE , 0x0a1fu16 => & ZEITMASTER , 0x1355u16 => & FEIERTAGENDE_JAHR , 0x1358u16 => & MODUS_PARTY_DAUER , 0x1359u16 => & KUEHLSOLLWERT_KORR_HZK_0 , 0x135cu16 => & MAX_KUEHLEN_AUSSENTEMP_HZK0 , 0x135du16 => & VL_SOLL_START_KUEHLEN_HZK_0 , 0x135eu16 => & VL_SOLL_MAX_KUEHLEN_HZK0 , 0x1388u16 => & FEHLER_AKTUELL , 0x13b5u16 => & START_KUEHLEN_AUSSENTEMP_HZK0 , 0x1400u16 => & HEIZPROG_1 , 0x1410u16 => & HEIZPROG_1_MO , 0x1411u16 => & HEIZPROG_1_MO_SCHALT_2 , 0x1412u16 => & HEIZPROG_1_MO_SCHALT_3 , 0x1420u16 => & HEIZPROG_1_DI , 0x1421u16 => & HEIZPROG_1_DI_SCHALT_2 , 0x1422u16 => & HEIZPROG_1_DI_SCHALT_3 , 0x1430u16 => & HEIZPROG_1_MI , 0x1431u16 => & HEIZPROG_1_MI_SCHALT_2 , 0x1432u16 => & HEIZPROG_1_MI_SCHALT_3 , 0x1440u16 => & HEIZPROG_1_DO , 0x1441u16 => & HEIZPROG_1_DO_SCHALT_2 , 0x1442u16 => & HEIZPROG_1_DO_SCHALT_3 , 0x1450u16 => & HEIZPROG_1_FR , 0x1451u16 => & HEIZPROG_1_FR_SCHALT_2 , 0x1452u16 => & HEIZPROG_1_FR_SCHALT_3 , 0x1460u16 => & HEIZPROG_1_SA , 0x1461u16 => & HEIZPROG_1_SA_SCHALT_2 , 0x1462u16 => & HEIZPROG_1_SA_SCHALT_3 , 0x1470u16 => & HEIZPROG_1_SO , 0x1471u16 => & HEIZPROG_1_SO_SCHALT_2 , 0x1472u16 => & HEIZPROG_1_SO_SCHALT_3 , 0x1480u16 => & HEIZPROG_1_MO_FR , 0x1481u16 => & HEIZPROG_1_MO_FR_SCHALT_2 , 0x1482u16 => & HEIZPROG_1_MO_FR_SCHALT_3 , 0x1490u16 => & HEIZPROG_1_SA_SO , 0x1491u16 => & HEIZPROG_1_SA_SO_SCHALT_2 , 0x1492u16 => & HEIZPROG_1_SA_SO_SCHALT_3 , 0x14a0u16 => & HEIZPROG_1_MO_SO , 0x14a1u16 => & HEIZPROG_1_MO_SO_SCHALT_2 , 0x14a2u16 => & HEIZPROG_1_MO_SO_SCHALT_3 , 0x14b0u16 => & HEIZPROG_1_MO_DO , 0x14b1u16 => & HEIZPROG_1_MO_DO_SCHALT_2 , 0x14b2u16 => & HEIZPROG_1_MO_DO_SCHALT_3 , 0x1500u16 => & HEIZPROG_2 , 0x1510u16 => & HEIZPROG_2_MO , 0x1511u16 => & HEIZPROG_2_MO_SCHALT_2 , 0x1512u16 => & HEIZPROG_2_MO_SCHALT_3 , 0x1520u16 => & HEIZPROG_2_DI , 0x1521u16 => & HEIZPROG_2_DI_SCHALT_2 , 0x1522u16 => & HEIZPROG_2_DI_SCHALT_3 , 0x1530u16 => & HEIZPROG_2_MI , 0x1531u16 => & HEIZPROG_2_MI_SCHALT_2 , 0x1532u16 => & HEIZPROG_2_MI_SCHALT_3 , 0x1540u16 => & HEIZPROG_2_DO , 0x1541u16 => & HEIZPROG_2_DO_SCHALT_2 , 0x1542u16 => & HEIZPROG_2_DO_SCHALT_3 , 0x1550u16 => & HEIZPROG_2_FR , 0x1551u16 => & HEIZPROG_2_FR_SCHALT_2 , 0x1552u16 => & HEIZPROG_2_FR_SCHALT_3 , 0x1560u16 => & HEIZPROG_2_SA , 0x1561u16 => & HEIZPROG_2_SA_SCHALT_2 , 0x1562u16 => & HEIZPROG_2_SA_SCHALT_3 , 0x1570u16 => & HEIZPROG_2_SO , 0x1571u16 => & HEIZPROG_2_SO_SCHALT_2 , 0x1572u16 => & HEIZPROG_2_SO_SCHALT_3 , 0x1580u16 => & HEIZPROG_2_MO_FR , 0x1581u16 => & HEIZPROG_2_MO_FR_SCHALT_2 , 0x1582u16 => & HEIZPROG_2_MO_FR_SCHALT_3 , 0x1590u16 => & HEIZPROG_2_SA_SO , 0x1591u16 => & HEIZPROG_2_SA_SO_SCHALT_2 , 0x1592u16 => & HEIZPROG_2_SA_SO_SCHALT_3 , 0x15a0u16 => & HEIZPROG_2_MO_SO , 0x15a1u16 => & HEIZPROG_2_MO_SO_SCHALT_2 , 0x15a2u16 => & HEIZPROG_2_MO_SO_SCHALT_3 , 0x15b0u16 => & HEIZPROG_2_MO_DO , 0x15b1u16 => & HEIZPROG_2_MO_DO_SCHALT_2 , 0x15b2u16 => & HEIZPROG_2_MO_DO_SCHALT_3 , 0x1700u16 => & W_WASSERPROG_1 , 0x1710u16 => & W_WASSERPROG_1_MO , 0x1711u16 => & W_WASSERPROG_1_MO_SCHALT_2 , 0x1712u16 => & W_WASSERPROG_1_MO_SCHALT_3 , 0x1720u16 => & W_WASSERPROG_1_DI , 0x1721u16 => & W_WASSERPROG_1_DI_SCHALT_2 , 0x1722u16 => & W_WASSERPROG_1_DI_SCHALT_3 , 0x1730u16 => & W_WASSERPROG_1_MI , 0x1731u16 => & W_WASSERPROG_1_MI_SCHALT_2 , 0x1732u16 => & W_WASSERPROG_1_MI_SCHALT_3 , 0x1740u16 => & W_WASSERPROG_1_DO , 0x1741u16 => & W_WASSERPROG_1_DO_SCHALT_2 , 0x1742u16 => & W_WASSERPROG_1_DO_SCHALT_3 , 0x1750u16 => & W_WASSERPROG_1_FR , 0x1751u16 => & W_WASSERPROG_1_FR_SCHALT_2 , 0x1752u16 => & W_WASSERPROG_1_FR_SCHALT_3 , 0x1760u16 => & W_WASSERPROG_1_SA , 0x1761u16 => & W_WASSERPROG_1_SA_SCHALT_2 , 0x1762u16 => & W_WASSERPROG_1_SA_SCHALT_3 , 0x1770u16 => & W_WASSERPROG_1_SO , 0x1771u16 => & W_WASSERPROG_1_SO_SCHALT_2 , 0x1772u16 => & W_WASSERPROG_1_SO_SCHALT_3 , 0x1780u16 => & W_WASSERPROG_1_MO_FR , 0x1781u16 => & W_WASSERPROG_1_MO_FR_SCHALT_2 , 0x1782u16 => & W_WASSERPROG_1_MO_FR_SCHALT_3 , 0x1790u16 => & W_WASSERPROG_1_SA_SO , 0x1791u16 => & W_WASSERPROG_1_SA_SO_SCHALT_2 , 0x1792u16 => & W_WASSERPROG_1_SA_SO_SCHALT_3 , 0x17a0u16 => & W_WASSERPROG_1_MO_SO , 0x17a1u16 => & W_WASSERPROG_1_MO_SO_SCHALT_2 , 0x17a2u16 => & W_WASSERPROG_1_MO_SO_SCHALT_3 , 0x17b0u16 => & W_WASSERPROG_1_MO_DO , 0x17b1u16 => & W_WASSERPROG_1_MO_DO_SCHALT_2 , 0x17b2u16 => & W_WASSERPROG_1_MO_DO_SCHALT_3 , 0x1800u16 => & W_WASSERPROG_2 , 0x1810u16 => & W_WASSERPROG_2_MO , 0x1811u16 => & W_WASSERPROG_2_MO_SCHALT_2 , 0x1812u16 => & W_WASSERPROG_2_MO_SCHALT_3 , 0x1820u16 => & W_WASSERPROG_2_DI , 0x1821u16 => & W_WASSERPROG_2_DI_SCHALT_2 , 0x1822u16 => & W_WASSERPROG_2_DI_SCHALT_3 , 0x1830u16 => & W_WASSERPROG_2_MI , 0x1831u16 => & W_WASSERPROG_2_MI_SCHALT_2 , 0x1832u16 => & W_WASSERPROG_2_MI_SCHALT_3 , 0x1840u16 => & W_WASSERPROG_2_DO , 0x1841u16 => & W_WASSERPROG_2_DO_SCHALT_2 , 0x1842u16 => & W_WASSERPROG_2_DO_SCHALT_3 , 0x1850u16 => & W_WASSERPROG_2_FR , 0x1851u16 => & W_WASSERPROG_2_FR_SCHALT_2 , 0x1852u16 => & W_WASSERPROG_2_FR_SCHALT_3 , 0x1860u16 => & W_WASSERPROG_2_SA , 0x1861u16 => & W_WASSERPROG_2_SA_SCHALT_2 , 0x1862u16 => & W_WASSERPROG_2_SA_SCHALT_3 , 0x1870u16 => & W_WASSERPROG_2_SO , 0x1871u16 => & W_WASSERPROG_2_SO_SCHALT_2 , 0x1872u16 => & W_WASSERPROG_2_SO_SCHALT_3 , 0x1880u16 => & W_WASSERPROG_2_MO_FR , 0x1881u16 => & W_WASSERPROG_2_MO_FR_SCHALT_2 , 0x1882u16 => & W_WASSERPROG_2_MO_FR_SCHALT_3 , 0x1890u16 => & W_WASSERPROG_2_SA_SO , 0x1891u16 => & W_WASSERPROG_2_SA_SO_SCHALT_2 , 0x1892u16 => & W_WASSERPROG_2_SA_SO_SCHALT_3 , 0x18a0u16 => & W_WASSERPROG_2_MO_SO , 0x18a1u16 => & W_WASSERPROG_2_MO_SO_SCHALT_2 , 0x18a2u16 => & W_WASSERPROG_2_MO_SO_SCHALT_3 , 0x18b0u16 => & W_WASSERPROG_2_MO_DO , 0x18b1u16 => & W_WASSERPROG_2_MO_DO_SCHALT_2 , 0x18b2u16 => & W_WASSERPROG_2_MO_DO_SCHALT_3 , 0xc09du16 => & VMIN_GCU , 0xc0b1u16 => & SONDERFKT_SCHALTKONTAKT , 0xc0b2u16 => & WARTEZEIT_SONDERFKT , 0xc0b3u16 => & SCHALTSCHWELLE_TDHW , 0xc0f6u16 => & BETRIEBSART , 0xc0f7u16 => & PWM_SIGNAL , 0xc0f8u16 => & SG , 0xc0fbu16 => & MISCHERSTELLUNG_2_3UVB , 0xc0fcu16 => & T_V , 0xc0feu16 => & T_TVBHMIX , 0xc101u16 => & V , 0xc102u16 => & T_TVBH1 , 0xc103u16 => & T_LIQ , 0xc104u16 => & T_R , 0xc106u16 => & T_DHW , 0xc176u16 => & T_AU , 0xc1bfu16 => & T_TVBH , 0xfd4fu16 => & ANTILEG_START_ZEIT , 0xfdacu16 => & STATUS_UMWAELZPUMPE };
