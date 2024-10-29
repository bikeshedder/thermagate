use super::{
    enums,
    param::{
        BoolParam, DecParam, Enum16Param, Enum8Param, I16Param, I8Param, MultilingualStr, Param,
        Time, TimeParam, TimeRange, TimeRangeParam, Unit,
    },
};
use rust_decimal_macros::dec;
pub const FEED_TEMPERATURE_TARGET: DecParam = DecParam {
    id: 0x0002,
    name: "FEED_TEMPERATURE_TARGET",
    label: MultilingualStr {
        de: "Vorlauftemperatur Soll",
        en: "Feed temperature, target",
    },
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(55.0)),
    min: Some(dec!(40.0)),
    max: Some(dec!(80.0)),
};
pub const HOT_WATER_TEMP_TARGET: DecParam = DecParam {
    id: 0x0003,
    name: "HOT_WATER_TEMP_TARGET",
    label: MultilingualStr {
        de: "Warmwassertemperatur Soll",
        en: "Hot water temperature, target",
    },
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(48.0)),
    min: Some(dec!(10.0)),
    max: Some(dec!(70.0)),
};
pub const FEED_TEMP_HC_TARGET: DecParam = DecParam {
    id: 0x0004,
    name: "FEED_TEMP_HC_TARGET",
    label: MultilingualStr {
        de: "Vorlauftemperatur HK Soll",
        en: "Feed temperature HC target",
    },
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const ROOM_TEMP_TARGET_1: DecParam = DecParam {
    id: 0x0005,
    name: "ROOM_TEMP_TARGET_1",
    label: MultilingualStr {
        de: "Raumtemperatur Soll 1",
        en: "Room temperature target 1",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(20.0)),
    min: Some(dec!(5.0)),
    max: Some(dec!(40.0)),
};
pub const ROOM_TEMP_TARGET_2: DecParam = DecParam {
    id: 0x0006,
    name: "ROOM_TEMP_TARGET_2",
    label: MultilingualStr {
        de: "Raumtemperatur Soll 2",
        en: "Room temperature target 2",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(20.0)),
    min: Some(dec!(5.0)),
    max: Some(dec!(40.0)),
};
pub const ROOM_TEMP_TARGET_3: DecParam = DecParam {
    id: 0x0007,
    name: "ROOM_TEMP_TARGET_3",
    label: MultilingualStr {
        de: "Raumtemperatur Soll 3",
        en: "Room temperature target 3",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(20.0)),
    min: Some(dec!(5.0)),
    max: Some(dec!(40.0)),
};
pub const REDUCE_ROOM_TEMP: DecParam = DecParam {
    id: 0x0008,
    name: "REDUCE_ROOM_TEMP",
    label: MultilingualStr {
        de: "Raumtemperatur Absenken",
        en: "Reduce room temperature",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(15.0)),
    min: Some(dec!(5.0)),
    max: Some(dec!(40.0)),
};
pub const OUTSIDE_TEMP: DecParam = DecParam {
    id: 0x000c,
    name: "OUTSIDE_TEMP",
    label: MultilingualStr {
        de: "Außentemperatur",
        en: "Outside temperature",
    },
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const FEED_TEMPERATURE_CURRENT: DecParam = DecParam {
    id: 0x000d,
    name: "FEED_TEMPERATURE_CURRENT",
    label: MultilingualStr {
        de: "Vorlauftemperatur Aktuell",
        en: "Feed temperature, current",
    },
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const HOT_WATER_TEMP_CURRENT: DecParam = DecParam {
    id: 0x000e,
    name: "HOT_WATER_TEMP_CURRENT",
    label: MultilingualStr {
        de: "Warmwassertemperatur Aktuell",
        en: "Hot water temperature, current",
    },
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const FEED_TEMP_HC_CURRENT: DecParam = DecParam {
    id: 0x000f,
    name: "FEED_TEMP_HC_CURRENT",
    label: MultilingualStr {
        de: "Vorlauftemperatur HK Aktuell",
        en: "Feed temperature HC current",
    },
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const MYSTERY_0010: I16Param = I16Param {
    id: 0x0010,
    name: "MYSTERY_0010",
    label: MultilingualStr {
        de: "Mystery 0x0010",
        en: "Mystery 0x0010",
    },
    read: true,
    write: false,
    unit: None,
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
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
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
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: Some(dec!(5.0)),
    max: Some(dec!(40.0)),
};
pub const HOT_WATER_TEMP_TARGET_1: DecParam = DecParam {
    id: 0x0013,
    name: "HOT_WATER_TEMP_TARGET_1",
    label: MultilingualStr {
        de: "Warmwassertemperatur Soll 1",
        en: "Hot water temperature target 1",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(48.0)),
    min: Some(dec!(35.0)),
    max: Some(dec!(70.0)),
};
pub const RETURN_FLOW_TEMP: DecParam = DecParam {
    id: 0x0016,
    name: "RETURN_FLOW_TEMP",
    label: MultilingualStr {
        de: "Rücklauftemperatur",
        en: "Return flow temperature",
    },
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const WATER_PRESSURE: DecParam = DecParam {
    id: 0x001c,
    name: "WATER_PRESSURE",
    label: MultilingualStr {
        de: "Wasserdruck (P)",
        en: "Water pressure (P)",
    },
    read: true,
    write: false,
    unit: Some(Unit::Bar),
    scale: 3,
    default: None,
    min: None,
    max: None,
};
pub const MAX_FEED_TEMP: DecParam = DecParam {
    id: 0x0028,
    name: "MAX_FEED_TEMP",
    label: MultilingualStr {
        de: "Max. Vorlauftemperatur",
        en: "Max. feed temperature",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(80.0)),
    min: Some(dec!(20.0)),
    max: Some(dec!(90.0)),
};
pub const MIXER_PUMP_STATUS: BoolParam = BoolParam {
    id: 0x0052,
    name: "MIXER_PUMP_STATUS",
    label: MultilingualStr {
        de: "Status Mischerpumpe",
        en: "Mixer pump status",
    },
    read: true,
    write: false,
    default: None,
};
pub const MIXER_INFO_1: I16Param = I16Param {
    id: 0x0056,
    name: "MIXER_INFO_1",
    label: MultilingualStr {
        de: "Mischer Info 1",
        en: "Mixer Info 1",
    },
    read: true,
    write: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const MIXER_INFO_2: I16Param = I16Param {
    id: 0x0057,
    name: "MIXER_INFO_2",
    label: MultilingualStr {
        de: "Mischer Info 2",
        en: "Mixer Info 2",
    },
    read: true,
    write: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const MIXER_INFO_3: I16Param = I16Param {
    id: 0x0058,
    name: "MIXER_INFO_3",
    label: MultilingualStr {
        de: "Mischer Info 3",
        en: "Mixer Info 3",
    },
    read: true,
    write: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const MIXER_INFO_4: I16Param = I16Param {
    id: 0x0059,
    name: "MIXER_INFO_4",
    label: MultilingualStr {
        de: "Mischer Info 4",
        en: "Mixer Info 4",
    },
    read: true,
    write: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const HOT_WATER_ACTIVE: BoolParam = BoolParam {
    id: 0x005e,
    name: "HOT_WATER_ACTIVE",
    label: MultilingualStr {
        de: "Warmwasserbereitung aktiv",
        en: "DHW heating up active",
    },
    read: true,
    write: false,
    default: None,
};
pub const STATUS_COMPRESSOR: BoolParam = BoolParam {
    id: 0x0061,
    name: "STATUS_COMPRESSOR",
    label: MultilingualStr {
        de: "Status Kompressor",
        en: "Status compressor",
    },
    read: true,
    write: false,
    default: None,
};
pub const ANTI_LEGIONELLA_DAY: Enum16Param<enums::AntiLegionellaDay> = Enum16Param {
    id: 0x0101,
    name: "ANTI_LEGIONELLA_DAY",
    label: MultilingualStr {
        de: "Antilegionellen Tag",
        en: "Anti-legionella day",
    },
    read: true,
    write: true,
    default: None,
};
pub const AUFHEIZOPTIMIERUNG: I8Param = I8Param {
    id: 0x0103,
    name: "AUFHEIZOPTIMIERUNG",
    label: MultilingualStr {
        de: "eAUFHEIZOPTIMIERUNG",
        en: "eAUFHEIZOPTIMIERUNG",
    },
    read: true,
    write: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const BUILDING_INSULATION: Enum8Param<enums::BuildingInsulation> = Enum8Param {
    id: 0x010c,
    name: "BUILDING_INSULATION",
    label: MultilingualStr {
        de: "Gebäudedämmung",
        en: "Building insulation",
    },
    read: true,
    write: true,
    default: None,
};
pub const HEATING_CURVE: DecParam = DecParam {
    id: 0x010e,
    name: "HEATING_CURVE",
    label: MultilingualStr {
        de: "Heizkurve",
        en: "Heating curve",
    },
    read: true,
    write: true,
    unit: None,
    scale: 2,
    default: Some(dec!(0.5)),
    min: Some(dec!(0.0)),
    max: Some(dec!(3.0)),
};
pub const ROOM_SENSOR_ADAPTATION: I8Param = I8Param {
    id: 0x010f,
    name: "ROOM_SENSOR_ADAPTATION",
    label: MultilingualStr {
        de: "Raumeinfluss",
        en: "Room sensor adaptation",
    },
    read: true,
    write: true,
    unit: Some(Unit::Kelvin),
    default: None,
    min: Some(0),
    max: Some(20),
};
pub const MAX_AUFHEIZVORVERLEGUNG: I16Param = I16Param {
    id: 0x0110,
    name: "MAX_AUFHEIZVORVERLEGUNG",
    label: MultilingualStr {
        de: "eMAX_AUFHEIZVORVERLEGUNG",
        en: "eMAX_AUFHEIZVORVERLEGUNG",
    },
    read: true,
    write: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const SLOPE_OFFSET: DecParam = DecParam {
    id: 0x0111,
    name: "SLOPE_OFFSET",
    label: MultilingualStr {
        de: "Kurvenabstand",
        en: "Slope offset",
    },
    read: true,
    write: true,
    unit: Some(Unit::Kelvin),
    scale: 1,
    default: Some(dec!(5.0)),
    min: Some(dec!(0.0)),
    max: Some(dec!(50.0)),
};
pub const OPERATING_MODE: Enum8Param<enums::OperatingMode> = Enum8Param {
    id: 0x0112,
    name: "OPERATING_MODE",
    label: MultilingualStr {
        de: "Betriebsart",
        en: "Operating mode",
    },
    read: true,
    write: true,
    default: None,
};
pub const HEATING_CURVE_ADAPTION: BoolParam = BoolParam {
    id: 0x0115,
    name: "HEATING_CURVE_ADAPTION",
    label: MultilingualStr {
        de: "Heizkurvenadaption",
        en: "Heating curve adaption",
    },
    read: true,
    write: true,
    default: Some(false),
};
pub const HEAT_LIMIT_HEATING_MODE: DecParam = DecParam {
    id: 0x0116,
    name: "HEAT_LIMIT_HEATING_MODE",
    label: MultilingualStr {
        de: "Heizgrenze Heizbetrieb",
        en: "Heat limit, heating mode",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(19.0)),
    min: Some(dec!(10.0)),
    max: Some(dec!(40.0)),
};
pub const HEAT_LIMIT_REDUCING_MODE: DecParam = DecParam {
    id: 0x0117,
    name: "HEAT_LIMIT_REDUCING_MODE",
    label: MultilingualStr {
        de: "Heizgrenze Absenkbetrieb",
        en: "Heat limit, reducing mode",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(10.0)),
    min: Some(dec!(10.0)),
    max: Some(dec!(40.0)),
};
pub const SCREED: BoolParam = BoolParam {
    id: 0x011a,
    name: "SCREED",
    label: MultilingualStr {
        de: "Screed",
        en: "Estrich",
    },
    read: true,
    write: true,
    default: None,
};
pub const VACATION_BEGIN_DAY: I8Param = I8Param {
    id: 0x011b,
    name: "VACATION_BEGIN_DAY",
    label: MultilingualStr {
        de: "Urlaub :: Datum 1. Tag :: Tag",
        en: "Holiday :: Date 1st day :: Day",
    },
    read: true,
    write: true,
    unit: None,
    default: Some(2),
    min: Some(0),
    max: Some(31),
};
pub const VACATION_BEGIN_MONTH: I8Param = I8Param {
    id: 0x011c,
    name: "VACATION_BEGIN_MONTH",
    label: MultilingualStr {
        de: "Urlaub :: Datum 1. Tag :: Monat",
        en: "Holiday :: Date 1st day :: Month",
    },
    read: true,
    write: true,
    unit: None,
    default: Some(1),
    min: Some(0),
    max: Some(12),
};
pub const VACATION_BEGIN_YEAR: I8Param = I8Param {
    id: 0x011d,
    name: "VACATION_BEGIN_YEAR",
    label: MultilingualStr {
        de: "Urlaub :: Datum 1. Tag :: Jahr",
        en: "Holiday :: Date 1st day :: Year",
    },
    read: true,
    write: true,
    unit: None,
    default: Some(1),
    min: Some(0),
    max: Some(49),
};
pub const VACATION_END_DAY: I8Param = I8Param {
    id: 0x011e,
    name: "VACATION_END_DAY",
    label: MultilingualStr {
        de: "Urlaub :: Datum letzter Tag :: Tag",
        en: "Holiday :: Date last day :: Day",
    },
    read: true,
    write: true,
    unit: None,
    default: Some(3),
    min: Some(0),
    max: Some(31),
};
pub const VACATION_END_MONTH: I8Param = I8Param {
    id: 0x011f,
    name: "VACATION_END_MONTH",
    label: MultilingualStr {
        de: "Urlaub :: Datum letzter Tag :: Monat",
        en: "Holiday :: Date last day :: Month",
    },
    read: true,
    write: true,
    unit: None,
    default: Some(1),
    min: Some(0),
    max: Some(12),
};
pub const VACATION_END_YEAR: I8Param = I8Param {
    id: 0x0120,
    name: "VACATION_END_YEAR",
    label: MultilingualStr {
        de: "Urlaub :: Datum letzter Tag :: Jahr",
        en: "Holiday :: Date last day :: Year",
    },
    read: true,
    write: true,
    unit: None,
    default: Some(1),
    min: Some(0),
    max: Some(49),
};
pub const DATE_DAY: I8Param = I8Param {
    id: 0x0122,
    name: "DATE_DAY",
    label: MultilingualStr {
        de: "Datum :: Tag",
        en: "Date :: Day",
    },
    read: true,
    write: true,
    unit: None,
    default: None,
    min: Some(1),
    max: Some(31),
};
pub const DATE_MONTH: I8Param = I8Param {
    id: 0x0123,
    name: "DATE_MONTH",
    label: MultilingualStr {
        de: "Datum :: Monat",
        en: "Date :: Month",
    },
    read: true,
    write: true,
    unit: None,
    default: None,
    min: Some(1),
    max: Some(12),
};
pub const DATE_YEAR: I8Param = I8Param {
    id: 0x0124,
    name: "DATE_YEAR",
    label: MultilingualStr {
        de: "Datum :: Jahr",
        en: "Date :: Year",
    },
    read: true,
    write: true,
    unit: None,
    default: None,
    min: Some(0),
    max: Some(49),
};
pub const TIME_HOUR: I8Param = I8Param {
    id: 0x0125,
    name: "TIME_HOUR",
    label: MultilingualStr {
        de: "Datum :: Stunde",
        en: "Time :: Hour",
    },
    read: true,
    write: true,
    unit: None,
    default: None,
    min: Some(0),
    max: Some(23),
};
pub const TIME_MINUTE: I8Param = I8Param {
    id: 0x0126,
    name: "TIME_MINUTE",
    label: MultilingualStr {
        de: "Datum :: Minute",
        en: "Time :: Minute",
    },
    read: true,
    write: true,
    unit: None,
    default: None,
    min: Some(0),
    max: Some(59),
};
pub const FEED_TEMP_HEATING_MODE: DecParam = DecParam {
    id: 0x0129,
    name: "FEED_TEMP_HEATING_MODE",
    label: MultilingualStr {
        de: "Vorlauftemperatur Heizbetrieb",
        en: "Feed temperature, heating mode",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(40.0)),
    min: Some(dec!(10.0)),
    max: Some(dec!(70.0)),
};
pub const FEED_TEMP_REDUCING_MODE: DecParam = DecParam {
    id: 0x012a,
    name: "FEED_TEMP_REDUCING_MODE",
    label: MultilingualStr {
        de: "Vorlauftemperatur Absenkbetrieb",
        en: "Feed temperature, reducing mode",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(10.0)),
    min: Some(dec!(10.0)),
    max: Some(dec!(90.0)),
};
pub const MIN_FEED_TEMP: DecParam = DecParam {
    id: 0x012b,
    name: "MIN_FEED_TEMP",
    label: MultilingualStr {
        de: "Min. Vorlauftemperatur",
        en: "Min. feed temperature",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(10.0)),
    min: Some(dec!(10.0)),
    max: Some(dec!(90.0)),
};
pub const ABSENKOPTIMIERUNG: I16Param = I16Param {
    id: 0x012e,
    name: "ABSENKOPTIMIERUNG",
    label: MultilingualStr {
        de: "eABSENKOPTIMIERUNG",
        en: "eABSENKOPTIMIERUNG",
    },
    read: true,
    write: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const MAX_MIXER_VALVE_PUMP: I16Param = I16Param {
    id: 0x012f,
    name: "MAX_MIXER_VALVE_PUMP",
    label: MultilingualStr {
        de: "Max. Mischer Pumpe",
        en: "Max. mixer valve pump",
    },
    read: true,
    write: true,
    unit: None,
    default: Some(100),
    min: Some(20),
    max: Some(100),
};
pub const MIN_MIXER_VALVE_PUMP: I16Param = I16Param {
    id: 0x0130,
    name: "MIN_MIXER_VALVE_PUMP",
    label: MultilingualStr {
        de: "Min. Mischer Pumpe",
        en: "Min. mixer valve pump",
    },
    read: true,
    write: true,
    unit: None,
    default: Some(30),
    min: Some(10),
    max: Some(100),
};
pub const MIXER_PUMP_PWM: I16Param = I16Param {
    id: 0x0131,
    name: "MIXER_PUMP_PWM",
    label: MultilingualStr {
        de: "PWM Mischerpumpe",
        en: "PWM mixer pump",
    },
    read: true,
    write: false,
    unit: Some(Unit::Percent),
    default: None,
    min: Some(0),
    max: Some(100),
};
pub const ROOM_TEMP_ABSENT: DecParam = DecParam {
    id: 0x013d,
    name: "ROOM_TEMP_ABSENT",
    label: MultilingualStr {
        de: "Raumtemperatur Abwesend",
        en: "Room temperature absent",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(15.0)),
    min: Some(dec!(5.0)),
    max: Some(dec!(40.0)),
};
pub const HOT_WATER_TEMP_TARGET_3: DecParam = DecParam {
    id: 0x013e,
    name: "HOT_WATER_TEMP_TARGET_3",
    label: MultilingualStr {
        de: "Warmwassertemperatur Soll 3",
        en: "Hot water temperature target 3",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(48.0)),
    min: Some(dec!(35.0)),
    max: Some(dec!(70.0)),
};
pub const WEATHER_COMPENSATED: Enum16Param<enums::WeatherCompensated> = Enum16Param {
    id: 0x0141,
    name: "WEATHER_COMPENSATED",
    label: MultilingualStr {
        de: "Witterungsgeführt",
        en: "Weather-compensated",
    },
    read: true,
    write: true,
    default: None,
};
pub const LOAD_HOT_WATER: BoolParam = BoolParam {
    id: 0x0144,
    name: "LOAD_HOT_WATER",
    label: MultilingualStr {
        de: "1x Warmwasser",
        en: "1x Hot Water",
    },
    read: true,
    write: true,
    default: Some(false),
};
pub const GERAETE_KENNUNG: Enum16Param<enums::HeatGeneratorType> = Enum16Param {
    id: 0x0148,
    name: "GERAETE_KENNUNG",
    label: MultilingualStr {
        de: "Wärmeerzeugertyp",
        en: "Type of heat generator",
    },
    read: true,
    write: false,
    default: None,
};
pub const MAX_HOT_WATER_LOADING_TIME: I16Param = I16Param {
    id: 0x0180,
    name: "MAX_HOT_WATER_LOADING_TIME",
    label: MultilingualStr {
        de: "Max, Warmwasser Ladezeit",
        en: "Max. hot water loading time",
    },
    read: true,
    write: true,
    unit: Some(Unit::Minutes),
    default: Some(60),
    min: Some(10),
    max: Some(240),
};
pub const CIRCULATION_PUMP_CONTROL: BoolParam = BoolParam {
    id: 0x0182,
    name: "CIRCULATION_PUMP_CONTROL",
    label: MultilingualStr {
        de: "Zirkulationspumpe Ansteuerung",
        en: "Circulation pump control",
    },
    read: true,
    write: true,
    default: None,
};
pub const VERSION_MAJOR: I16Param = I16Param {
    id: 0x0199,
    name: "VERSION_MAJOR",
    label: MultilingualStr {
        de: "Major Version",
        en: "Major version",
    },
    read: true,
    write: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const VERSION_MINOR: I16Param = I16Param {
    id: 0x019a,
    name: "VERSION_MINOR",
    label: MultilingualStr {
        de: "Minor Version",
        en: "Minor version",
    },
    read: true,
    write: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const VOLUME_FLOW: DecParam = DecParam {
    id: 0x01da,
    name: "VOLUME_FLOW",
    label: MultilingualStr {
        de: "Volumenstrom",
        en: "Volume flow",
    },
    read: true,
    write: false,
    unit: Some(Unit::LitersPerHour),
    scale: 0,
    default: None,
    min: None,
    max: None,
};
pub const ABSENT: I16Param = I16Param {
    id: 0x01ec,
    name: "ABSENT",
    label: MultilingualStr {
        de: "Abwesend",
        en: "Absent",
    },
    read: true,
    write: true,
    unit: Some(Unit::Minutes),
    default: Some(0),
    min: None,
    max: None,
};
pub const VERSION_PATCH: I16Param = I16Param {
    id: 0x024b,
    name: "VERSION_PATCH",
    label: MultilingualStr {
        de: "Patch Version",
        en: "Patch version",
    },
    read: true,
    write: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const FEED_TEMP_COOLING_MODE: DecParam = DecParam {
    id: 0x03dd,
    name: "FEED_TEMP_COOLING_MODE",
    label: MultilingualStr {
        de: "Vorlauftemperatur Kühlbetrieb",
        en: "Feed temperature, cooling mode",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(18.0)),
    min: Some(dec!(8.0)),
    max: Some(dec!(30.0)),
};
pub const PROGRAMMABLE_OUTPUT: Enum16Param<enums::ProgrammableOutput> = Enum16Param {
    id: 0x0489,
    name: "PROGRAMMABLE_OUTPUT",
    label: MultilingualStr {
        de: "Programmierbarer Ausgang (230V)",
        en: "Programmable output (230V)",
    },
    read: true,
    write: true,
    default: None,
};
pub const ANTI_LEGIONELLA_TEMP: DecParam = DecParam {
    id: 0x0587,
    name: "ANTI_LEGIONELLA_TEMP",
    label: MultilingualStr {
        de: "Antilegionellen Temperatur",
        en: "Anti-legionella temperature",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(65.0)),
    min: Some(dec!(60.0)),
    max: Some(dec!(70.0)),
};
pub const CIRCULATION_PUMP_INTERVAL: I16Param = I16Param {
    id: 0x065e,
    name: "CIRCULATION_PUMP_INTERVAL",
    label: MultilingualStr {
        de: "Zirkulationspumpe Intervall",
        en: "Circulation pump interval",
    },
    read: true,
    write: true,
    unit: Some(Unit::Minutes),
    default: None,
    min: Some(1),
    max: Some(15),
};
pub const VMIN_A1: DecParam = DecParam {
    id: 0x0661,
    name: "VMIN_A1",
    label: MultilingualStr {
        de: "Durchfluss (VMIN_A1)",
        en: "Volume flow",
    },
    read: true,
    write: false,
    unit: Some(Unit::LitersPerHour),
    scale: 0,
    default: None,
    min: None,
    max: None,
};
pub const EXT_POWER_HOT_WATER: DecParam = DecParam {
    id: 0x0668,
    name: "EXT_POWER_HOT_WATER",
    label: MultilingualStr {
        de: "Ext. Leistung Warmwasser",
        en: "External power hot water",
    },
    read: true,
    write: true,
    unit: Some(Unit::KiloWatt),
    scale: 2,
    default: Some(dec!(3.0)),
    min: Some(dec!(1.0)),
    max: Some(dec!(40.0)),
};
pub const EXT_POWER_STAGE_1: DecParam = DecParam {
    id: 0x0669,
    name: "EXT_POWER_STAGE_1",
    label: MultilingualStr {
        de: "Ext. Leistung Stufe 1",
        en: "External power stage 1",
    },
    read: true,
    write: true,
    unit: Some(Unit::KiloWatt),
    scale: 2,
    default: Some(dec!(3.0)),
    min: Some(dec!(1.0)),
    max: Some(dec!(40.0)),
};
pub const EXT_POWER_STAGE_2: DecParam = DecParam {
    id: 0x066a,
    name: "EXT_POWER_STAGE_2",
    label: MultilingualStr {
        de: "Ext. Leistung Stufe 2",
        en: "External power stage 2",
    },
    read: true,
    write: true,
    unit: Some(Unit::KiloWatt),
    scale: 2,
    default: Some(dec!(3.0)),
    min: Some(dec!(1.0)),
    max: Some(dec!(40.0)),
};
pub const HEATING_SUPPORT_POWER: DecParam = DecParam {
    id: 0x066b,
    name: "HEATING_SUPPORT_POWER",
    label: MultilingualStr {
        de: "Leistung BIV",
        en: "Heating support power",
    },
    read: true,
    write: true,
    unit: Some(Unit::KiloWatt),
    scale: 2,
    default: Some(dec!(15.0)),
    min: Some(dec!(3.0)),
    max: Some(dec!(40.0)),
};
pub const HEATING_SUPPORT: BoolParam = BoolParam {
    id: 0x066c,
    name: "HEATING_SUPPORT",
    label: MultilingualStr {
        de: "Heizungsunterstützung (HZU)",
        en: "Heating support (HZU)",
    },
    read: true,
    write: true,
    default: None,
};
pub const HEATING_SUPPORT_HYSTERESIS: DecParam = DecParam {
    id: 0x066d,
    name: "HEATING_SUPPORT_HYSTERESIS",
    label: MultilingualStr {
        de: "HZU Hysterese",
        en: "Heating support hysteresis",
    },
    read: true,
    write: true,
    unit: Some(Unit::Kelvin),
    scale: 1,
    default: Some(dec!(5.0)),
    min: Some(dec!(2.0)),
    max: Some(dec!(15.0)),
};
pub const HEATING_SUPPORT_MAX_TEMP: DecParam = DecParam {
    id: 0x066e,
    name: "HEATING_SUPPORT_MAX_TEMP",
    label: MultilingualStr {
        de: "HZU Max. Temperatur",
        en: "Heating support max. temp.",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(60.0)),
    min: Some(dec!(5.0)),
    max: Some(dec!(85.0)),
};
pub const HT_NT_FUNCTION: Enum16Param<enums::HtNtFunction> = Enum16Param {
    id: 0x066f,
    name: "HT_NT_FUNCTION",
    label: MultilingualStr {
        de: "HT/NT Funktion",
        en: "HT/NT function",
    },
    read: true,
    write: true,
    default: None,
};
pub const HT_NT_CONTACT: Enum16Param<enums::HtNtContact> = Enum16Param {
    id: 0x0670,
    name: "HT_NT_CONTACT",
    label: MultilingualStr {
        de: "HT/NT Anschluss",
        en: "HT/NT contact",
    },
    read: true,
    write: true,
    default: None,
};
pub const AUX_SWITCHING_FUNCTION: Enum16Param<enums::AuxSwitchingFunction> = Enum16Param {
    id: 0x0671,
    name: "AUX_SWITCHING_FUNCTION",
    label: MultilingualStr {
        de: "AUX-Schaltfunktion",
        en: "AUX switching function",
    },
    read: true,
    write: true,
    default: None,
};
pub const AUX_WAIT_TIME: I16Param = I16Param {
    id: 0x0672,
    name: "AUX_WAIT_TIME",
    label: MultilingualStr {
        de: "AUX-Wartezeit",
        en: "AUX wait time",
    },
    read: true,
    write: true,
    unit: Some(Unit::Seconds),
    default: Some(120),
    min: Some(0),
    max: Some(600),
};
pub const AUX_SWITCHING_THRESHOLD_TDHW: DecParam = DecParam {
    id: 0x0673,
    name: "AUX_SWITCHING_THRESHOLD_TDHW",
    label: MultilingualStr {
        de: "Schaltschwelle DTHW (AUX)",
        en: "Switching threshold TDHW (AUX)",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(50.0)),
    min: Some(dec!(20.0)),
    max: Some(dec!(85.0)),
};
pub const ROOM_THERMOSTAT: BoolParam = BoolParam {
    id: 0x0678,
    name: "ROOM_THERMOSTAT",
    label: MultilingualStr {
        de: "Raumthermostat",
        en: "Room thermostat",
    },
    read: true,
    write: true,
    default: Some(false),
};
pub const INTERLINK_FUNCTION: BoolParam = BoolParam {
    id: 0x0679,
    name: "INTERLINK_FUNCTION",
    label: MultilingualStr {
        de: "Interlinkfunktion",
        en: "Interlink function",
    },
    read: true,
    write: true,
    default: Some(false),
};
pub const MIN_PUMP_POWER: DecParam = DecParam {
    id: 0x067e,
    name: "MIN_PUMP_POWER",
    label: MultilingualStr {
        de: "Min. Leistung Pumpe",
        en: "Min. pump power",
    },
    read: true,
    write: true,
    unit: Some(Unit::Percent),
    scale: 1,
    default: Some(dec!(50.0)),
    min: Some(dec!(40.0)),
    max: Some(dec!(80.0)),
};
pub const MAX_PUMP_POWER: DecParam = DecParam {
    id: 0x067f,
    name: "MAX_PUMP_POWER",
    label: MultilingualStr {
        de: "Max. Leistung Pumpe",
        en: "Max. pump power",
    },
    read: true,
    write: true,
    unit: Some(Unit::Percent),
    scale: 1,
    default: Some(dec!(50.0)),
    min: Some(dec!(60.0)),
    max: Some(dec!(80.0)),
};
pub const WP_MOD_HYST_DURCHFLUSS: DecParam = DecParam {
    id: 0x0682,
    name: "WP_MOD_HYST_DURCHFLUSS",
    label: MultilingualStr {
        de: "eWP_MOD_HYST_DURCHFLUSS",
        en: "eWP_MOD_HYST_DURCHFLUSS",
    },
    read: true,
    write: false,
    unit: Some(Unit::LitersPerHour),
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
    read: true,
    write: false,
    unit: Some(Unit::Kelvin),
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
    read: true,
    write: false,
    unit: Some(Unit::Kelvin),
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
    read: true,
    write: false,
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
        de: "Durchfluss (VMIN_WP)",
        en: "Volume flow",
    },
    read: true,
    write: false,
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
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const HOT_WATER_HYSTERESIS: DecParam = DecParam {
    id: 0x0691,
    name: "HOT_WATER_HYSTERESIS",
    label: MultilingualStr {
        de: "Warmwasser Hysterese",
        en: "Hot water hysteresis",
    },
    read: true,
    write: true,
    unit: Some(Unit::Kelvin),
    scale: 1,
    default: Some(dec!(7.0)),
    min: Some(dec!(2.0)),
    max: Some(dec!(20.0)),
};
pub const WAITING_TIME_EXT_HEAT_GENERATOR: I16Param = I16Param {
    id: 0x0692,
    name: "WAITING_TIME_EXT_HEAT_GENERATOR",
    label: MultilingualStr {
        de: "Wartezeit ext. Wärmeerzeuger",
        en: "Wait time ext. heat generator",
    },
    read: true,
    write: true,
    unit: Some(Unit::Minutes),
    default: Some(1),
    min: Some(20),
    max: Some(95),
};
pub const SMART_GRID: BoolParam = BoolParam {
    id: 0x0693,
    name: "SMART_GRID",
    label: MultilingualStr {
        de: "Smart Grid",
        en: "Smart grid",
    },
    read: true,
    write: true,
    default: None,
};
pub const SMART_GRID_MODE: Enum16Param<enums::SmartGridMode> = Enum16Param {
    id: 0x0694,
    name: "SMART_GRID_MODE",
    label: MultilingualStr {
        de: "Modus Smart Grid",
        en: "Smart grid mode",
    },
    read: true,
    write: true,
    default: None,
};
pub const VENTILATION_FUNCTION: BoolParam = BoolParam {
    id: 0x0695,
    name: "VENTILATION_FUNCTION",
    label: MultilingualStr {
        de: "Entlüftungsfunktion",
        en: "Ventilation function",
    },
    read: true,
    write: true,
    default: None,
};
pub const QUIET_MODE: I16Param = I16Param {
    id: 0x0696,
    name: "QUIET_MODE",
    label: MultilingualStr {
        de: "Flüsterbetrieb",
        en: "Quiet mode",
    },
    read: true,
    write: false,
    unit: None,
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
    read: true,
    write: false,
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
    read: true,
    write: false,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const MIX_3UVDHW: I16Param = I16Param {
    id: 0x069b,
    name: "MIX_3UVDHW",
    label: MultilingualStr {
        de: "3UVDHW Position (DHW)",
        en: "3UVDHW position (DHW)",
    },
    read: true,
    write: false,
    unit: Some(Unit::Percent),
    default: None,
    min: Some(0),
    max: Some(100),
};
pub const ENERGY_HP_COOLING: I16Param = I16Param {
    id: 0x06a6,
    name: "ENERGY_HP_COOLING",
    label: MultilingualStr {
        de: "Energie WP Kühlung",
        en: "Energy HP cooling",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING: I16Param = I16Param {
    id: 0x06a7,
    name: "ENERGY_HP_HEATING",
    label: MultilingualStr {
        de: "Energie WP Heizung",
        en: "Energy HP heating",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const INTERLINK_TEMP_INCREASE: DecParam = DecParam {
    id: 0x06a0,
    name: "INTERLINK_TEMP_INCREASE",
    label: MultilingualStr {
        de: "Interlink Temperaturerhöhung",
        en: "Interlink temperature increase",
    },
    read: true,
    write: true,
    unit: None,
    scale: 1,
    default: Some(dec!(5.0)),
    min: Some(dec!(1.0)),
    max: Some(dec!(50.0)),
};
pub const INTERLINK_TEMP_REDUCTION: DecParam = DecParam {
    id: 0x06a1,
    name: "INTERLINK_TEMP_REDUCTION",
    label: MultilingualStr {
        de: "Interlink Temperaturreduktion",
        en: "Interlink temperature reduction",
    },
    read: true,
    write: true,
    unit: None,
    scale: 1,
    default: Some(dec!(5.0)),
    min: Some(dec!(1.0)),
    max: Some(dec!(50.0)),
};
pub const RUNTIME_PUMP: I16Param = I16Param {
    id: 0x06a4,
    name: "RUNTIME_PUMP",
    label: MultilingualStr {
        de: "Laufzeit Pumpe",
        en: "Pump uptime",
    },
    read: true,
    write: false,
    unit: Some(Unit::Hours),
    default: None,
    min: None,
    max: None,
};
pub const RUNTIME_COMPRESSOR: I16Param = I16Param {
    id: 0x06a5,
    name: "RUNTIME_COMPRESSOR",
    label: MultilingualStr {
        de: "Laufzeit Kompressor",
        en: "Compressor uptime",
    },
    read: true,
    write: false,
    unit: Some(Unit::Hours),
    default: None,
    min: None,
    max: None,
};
pub const GLYCOL: BoolParam = BoolParam {
    id: 0x06d0,
    name: "GLYCOL",
    label: MultilingualStr {
        de: "Glykol",
        en: "Glycol",
    },
    read: true,
    write: true,
    default: None,
};
pub const HP_POWER_LIMITATION: I16Param = I16Param {
    id: 0x06d1,
    name: "HP_POWER_LIMITATION",
    label: MultilingualStr {
        de: "Leistungsbegrenzung Wärmepumpe",
        en: "HP Power limitation",
    },
    read: true,
    write: true,
    unit: Some(Unit::Ampere),
    default: Some(50),
    min: Some(20),
    max: Some(50),
};
pub const EXT_HEAT_SOURCE: Enum16Param<enums::ExtHeatSource> = Enum16Param {
    id: 0x06d2,
    name: "EXT_HEAT_SOURCE",
    label: MultilingualStr {
        de: "Konfig. externe Wärmequelle",
        en: "Config. ext. heat source",
    },
    read: true,
    write: true,
    default: None,
};
pub const BIVALENCE_FUNCTION: Enum16Param<enums::BivalenceFunction> = Enum16Param {
    id: 0x06d3,
    name: "BIVALENCE_FUNCTION",
    label: MultilingualStr {
        de: "Bivalenzfunktion",
        en: "Bivalence function",
    },
    read: true,
    write: true,
    default: None,
};
pub const BIVALENCE_TEMP: DecParam = DecParam {
    id: 0x06d4,
    name: "BIVALENCE_TEMP",
    label: MultilingualStr {
        de: "Bivalenztemperatur",
        en: "Bivalence temperature",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(0.0)),
    min: Some(dec!(-15.0)),
    max: Some(dec!(35.0)),
};
pub const PUMP_DT_HEATING: DecParam = DecParam {
    id: 0x06db,
    name: "PUMP_DT_HEATING",
    label: MultilingualStr {
        de: "Pumpe dT Heizen",
        en: "Pump dT heating",
    },
    read: true,
    write: true,
    unit: Some(Unit::Kelvin),
    scale: 1,
    default: Some(dec!(5.0)),
    min: Some(dec!(3.0)),
    max: Some(dec!(10.0)),
};
pub const PUMP_DT_COOLING: DecParam = DecParam {
    id: 0x06dc,
    name: "PUMP_DT_COOLING",
    label: MultilingualStr {
        de: "Pumpe dT Kuehlen",
        en: "Pump dT cooling",
    },
    read: true,
    write: true,
    unit: Some(Unit::Kelvin),
    scale: 1,
    default: Some(dec!(5.0)),
    min: Some(dec!(3.0)),
    max: Some(dec!(10.0)),
};
pub const HEATING_SYSTEM: Enum16Param<enums::HeatingSystem> = Enum16Param {
    id: 0x06dd,
    name: "HEATING_SYSTEM",
    label: MultilingualStr {
        de: "Heizsystem",
        en: "Heating System",
    },
    read: true,
    write: true,
    default: None,
};
pub const PUMP_LIMIT: Enum16Param<enums::PumpLimit> = Enum16Param {
    id: 0x06e1,
    name: "PUMP_LIMIT",
    label: MultilingualStr {
        de: "Pumpenlimit",
        en: "Pump limit",
    },
    read: true,
    write: true,
    default: None,
};
pub const FEED_TEMP_OVERSHOOT: DecParam = DecParam {
    id: 0x06e2,
    name: "FEED_TEMP_OVERSHOOT",
    label: MultilingualStr {
        de: "Vorlauftemperatur Überhöhung",
        en: "Feed temperature, overshoot",
    },
    read: true,
    write: true,
    unit: Some(Unit::Kelvin),
    scale: 1,
    default: Some(dec!(3.0)),
    min: Some(dec!(0.0)),
    max: Some(dec!(4.0)),
};
pub const CONTINUOUS_HEATING: BoolParam = BoolParam {
    id: 0x06e3,
    name: "CONTINUOUS_HEATING",
    label: MultilingualStr {
        de: "Continuous heating",
        en: "Continuous heating",
    },
    read: true,
    write: true,
    default: Some(true),
};
pub const COMFORT_HEATING: BoolParam = BoolParam {
    id: 0x06e5,
    name: "COMFORT_HEATING",
    label: MultilingualStr {
        de: "Comfort Heating",
        en: "Comfort Heating",
    },
    read: true,
    write: true,
    default: None,
};
pub const FUNC_BURNER_BLOCKING_CONTACT: Enum16Param<enums::FuncBurnerBlockingContact> =
    Enum16Param {
        id: 0x06e6,
        name: "FUNC_BURNER_BLOCKING_CONTACT",
        label: MultilingualStr {
            de: "Funktion Brennersperrkontakt",
            en: "Func. burner blocking contact",
        },
        read: true,
        write: true,
        default: None,
    };
pub const EMERGENCY: BoolParam = BoolParam {
    id: 0x06e7,
    name: "EMERGENCY",
    label: MultilingualStr {
        de: "Notbetrieb",
        en: "Emergency",
    },
    read: true,
    write: true,
    default: Some(false),
};
pub const WATER_PRESSURE_TARGET: DecParam = DecParam {
    id: 0x0725,
    name: "WATER_PRESSURE_TARGET",
    label: MultilingualStr {
        de: "Wassedruck Soll",
        en: "Water pressure, target",
    },
    read: true,
    write: true,
    unit: Some(Unit::Bar),
    scale: 3,
    default: Some(dec!(0.9)),
    min: Some(dec!(0.1)),
    max: Some(dec!(5.0)),
};
pub const WATER_MAX_PRESSURE_LOSS: DecParam = DecParam {
    id: 0x0726,
    name: "WATER_MAX_PRESSURE_LOSS",
    label: MultilingualStr {
        de: "Max. Druckverlust",
        en: "Maximum pressure loss",
    },
    read: true,
    write: true,
    unit: Some(Unit::Bar),
    scale: 3,
    default: Some(dec!(0.5)),
    min: Some(dec!(0.1)),
    max: Some(dec!(5.0)),
};
pub const WATER_PRESSURE_MAX: DecParam = DecParam {
    id: 0x0727,
    name: "WATER_PRESSURE_MAX",
    label: MultilingualStr {
        de: "Wassedruck Max.",
        en: "Water pressure, max.",
    },
    read: true,
    write: true,
    unit: Some(Unit::Bar),
    scale: 3,
    default: Some(dec!(3.0)),
    min: Some(dec!(0.1)),
    max: Some(dec!(5.0)),
};
pub const WATER_PRESSURE_MIN: DecParam = DecParam {
    id: 0x0728,
    name: "WATER_PRESSURE_MIN",
    label: MultilingualStr {
        de: "Wassedruck Min.",
        en: "Water pressure, min.",
    },
    read: true,
    write: true,
    unit: Some(Unit::Bar),
    scale: 3,
    default: Some(dec!(0.5)),
    min: Some(dec!(0.1)),
    max: Some(dec!(5.0)),
};
pub const ENERGY_EXT_HOT_WATER: I16Param = I16Param {
    id: 0x091c,
    name: "ENERGY_EXT_HOT_WATER",
    label: MultilingualStr {
        de: "Energie ext. Quelle Warmwasser",
        en: "Ext. energy source, hot water",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_EXT_HEATING: I16Param = I16Param {
    id: 0x0920,
    name: "ENERGY_EXT_HEATING",
    label: MultilingualStr {
        de: "Energie ext. Quelle Heizung",
        en: "Ext. energy source, heating",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER: I16Param = I16Param {
    id: 0x092c,
    name: "ENERGY_HOT_WATER",
    label: MultilingualStr {
        de: "Energie Warmwasser",
        en: "Energy hot water",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL: I16Param = I16Param {
    id: 0x0930,
    name: "ENERGY_HP_TOTAL",
    label: MultilingualStr {
        de: "Energie WP gesamt",
        en: "Energy HP total",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const MYSTERY_093C: I16Param = I16Param {
    id: 0x093c,
    name: "MYSTERY_093C",
    label: MultilingualStr {
        de: "Mystery 0x093c",
        en: "Mystery 0x093c",
    },
    read: true,
    write: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const WATER_SENSORS: I16Param = I16Param {
    id: 0x0961,
    name: "WATER_SENSORS",
    label: MultilingualStr {
        de: "Wassersensoren",
        en: "Water sensors",
    },
    read: true,
    write: true,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const FROST_PROTECTION_TEMP: DecParam = DecParam {
    id: 0x0a00,
    name: "FROST_PROTECTION_TEMP",
    label: MultilingualStr {
        de: "Frostschutztemperatur",
        en: "Frost protection temperature",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(0.0)),
    min: Some(dec!(-15.0)),
    max: Some(dec!(5.0)),
};
pub const HOT_WATER_TEMP_TARGET_2: DecParam = DecParam {
    id: 0x0a06,
    name: "HOT_WATER_TEMP_TARGET_2",
    label: MultilingualStr {
        de: "Warmwassertemperatur Soll 2",
        en: "Hot water temperature target 2",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(48.0)),
    min: Some(dec!(35.0)),
    max: Some(dec!(70.0)),
};
pub const AVERAGE_OUTSIDE_TEMP: DecParam = DecParam {
    id: 0x0a0c,
    name: "AVERAGE_OUTSIDE_TEMP",
    label: MultilingualStr {
        de: "Außentemperatur gemittelt",
        en: "Average outside temperature",
    },
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
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
    read: true,
    write: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const OUTSIDE_TEMP_CORRECTION: DecParam = DecParam {
    id: 0x0c1f,
    name: "OUTSIDE_TEMP_CORRECTION",
    label: MultilingualStr {
        de: "Außenfühlerkorrektur",
        en: "Outside temperature correction",
    },
    read: true,
    write: true,
    unit: Some(Unit::Kelvin),
    scale: 1,
    default: Some(dec!(0.0)),
    min: Some(dec!(-5.0)),
    max: Some(dec!(5.0)),
};
pub const HOLIDAY_BEGIN_DAY: I16Param = I16Param {
    id: 0x1350,
    name: "HOLIDAY_BEGIN_DAY",
    label: MultilingualStr {
        de: "Feiertag :: Datum 1. Tag :: Tag",
        en: "Holiday :: Date 1st day :: Day",
    },
    read: true,
    write: true,
    unit: None,
    default: None,
    min: Some(0),
    max: Some(31),
};
pub const HOLIDAY_BEGIN_MONTH: I16Param = I16Param {
    id: 0x1351,
    name: "HOLIDAY_BEGIN_MONTH",
    label: MultilingualStr {
        de: "Feiertag :: Datum 1. Tag :: Monat",
        en: "Holiday :: Date 1st day :: Month",
    },
    read: true,
    write: true,
    unit: None,
    default: None,
    min: Some(0),
    max: Some(12),
};
pub const HOLIDAY_BEGIN_YEAR: I16Param = I16Param {
    id: 0x1352,
    name: "HOLIDAY_BEGIN_YEAR",
    label: MultilingualStr {
        de: "Feiertag :: Datum 1. Tag :: Jahr",
        en: "Holiday :: Date 1st day :: Year",
    },
    read: true,
    write: true,
    unit: None,
    default: None,
    min: Some(0),
    max: Some(49),
};
pub const HOLIDAY_END_DAY: I16Param = I16Param {
    id: 0x1353,
    name: "HOLIDAY_END_DAY",
    label: MultilingualStr {
        de: "Feiertag :: Datum letzter Tag :: Tag",
        en: "Holiday :: Date last day :: Day",
    },
    read: true,
    write: true,
    unit: None,
    default: None,
    min: Some(0),
    max: Some(31),
};
pub const HOLIDAY_END_MONTH: I16Param = I16Param {
    id: 0x1354,
    name: "HOLIDAY_END_MONTH",
    label: MultilingualStr {
        de: "Feiertag :: Datum letzter Tag :: Monat",
        en: "Holiday :: Date last day :: Month",
    },
    read: true,
    write: true,
    unit: None,
    default: None,
    min: Some(0),
    max: Some(12),
};
pub const HOLIDAY_END_YEAR: I16Param = I16Param {
    id: 0x1355,
    name: "HOLIDAY_END_YEAR",
    label: MultilingualStr {
        de: "Feiertag :: Datum letzter Tag :: Jahr",
        en: "Holiday :: Date last day :: Year",
    },
    read: true,
    write: true,
    unit: None,
    default: None,
    min: Some(0),
    max: Some(49),
};
pub const TEMP_MANUAL_OPERATION: DecParam = DecParam {
    id: 0x1357,
    name: "TEMP_MANUAL_OPERATION",
    label: MultilingualStr {
        de: "Temperature, manual operation",
        en: "Temperatur Handbetrieb",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: Some(dec!(20.0)),
    max: Some(dec!(80.0)),
};
pub const PARTY: I16Param = I16Param {
    id: 0x1358,
    name: "PARTY",
    label: MultilingualStr {
        de: "Party",
        en: "Party",
    },
    read: true,
    write: true,
    unit: Some(Unit::Minutes),
    default: Some(0),
    min: Some(0),
    max: Some(360),
};
pub const COOLING_SETPOINT_CORRECTION: DecParam = DecParam {
    id: 0x1359,
    name: "COOLING_SETPOINT_CORRECTION",
    label: MultilingualStr {
        de: "Kühlsollwert Korrektur",
        en: "Cooling setpoint correction",
    },
    read: true,
    write: true,
    unit: Some(Unit::Kelvin),
    scale: 1,
    default: Some(dec!(0.0)),
    min: Some(dec!(-5.0)),
    max: Some(dec!(5.0)),
};
pub const START_COOLING_OUTSIDE_TEMP: DecParam = DecParam {
    id: 0x135b,
    name: "START_COOLING_OUTSIDE_TEMP",
    label: MultilingualStr {
        de: "Start Kühlen Außentemperatur",
        en: "Start cooling outside temp.",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(24.0)),
    min: Some(dec!(15.0)),
    max: Some(dec!(45.0)),
};
pub const MAX_COOLING_OUTSIDE_TEMP: DecParam = DecParam {
    id: 0x135c,
    name: "MAX_COOLING_OUTSIDE_TEMP",
    label: MultilingualStr {
        de: "Max. Kühlen Außentemperatur",
        en: "Max. cooling outside temp.",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(35.0)),
    min: Some(dec!(20.0)),
    max: Some(dec!(45.0)),
};
pub const TARGET_FLOW_COOLING_START: DecParam = DecParam {
    id: 0x135d,
    name: "TARGET_FLOW_COOLING_START",
    label: MultilingualStr {
        de: "VL-Soll Start Kühlen",
        en: "Target flow cooling, start",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(18.0)),
    min: Some(dec!(5.0)),
    max: Some(dec!(25.0)),
};
pub const TARGET_FLOW_COOLING_MAX: DecParam = DecParam {
    id: 0x135e,
    name: "TARGET_FLOW_COOLING_MAX",
    label: MultilingualStr {
        de: "VL-Soll Max Kühlen",
        en: "Target flow cooling, max.",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(8.0)),
    min: Some(dec!(5.0)),
    max: Some(dec!(25.0)),
};
pub const FEED_TEMP_LOWER_LIMIT: DecParam = DecParam {
    id: 0x1363,
    name: "FEED_TEMP_LOWER_LIMIT",
    label: MultilingualStr {
        de: "Untergrenze Vorlauftemperatur",
        en: "Feed temperature lower limit",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(18.0)),
    min: Some(dec!(5.0)),
    max: Some(dec!(25.0)),
};
pub const FEHLER_AKTUELL: I16Param = I16Param {
    id: 0x1388,
    name: "FEHLER_AKTUELL",
    label: MultilingualStr {
        de: "Aktueller Fehler",
        en: "Current error",
    },
    read: true,
    write: false,
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
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const HC_AUTO_1: TimeRangeParam = TimeRangeParam {
    id: 0x1400,
    name: "HC_AUTO_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1",
        en: "Heating circuit auto. 1",
    },
    read: false,
    write: false,
    default: None,
};
pub const HC_AUTO_1_MON_1: TimeRangeParam = TimeRangeParam {
    id: 0x1410,
    name: "HC_AUTO_1_MON_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Montag :: Schaltzyklus 1",
        en: "Heating circuit auto. 1 :: Monday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(6, 0),
        end: Time::new_const(22, 0),
    }),
};
pub const HC_AUTO_1_MON_2: TimeRangeParam = TimeRangeParam {
    id: 0x1411,
    name: "HC_AUTO_1_MON_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Montag :: Schaltzyklus 2",
        en: "Heating circuit auto. 1 :: Monday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_1_MON_3: TimeRangeParam = TimeRangeParam {
    id: 0x1412,
    name: "HC_AUTO_1_MON_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Montag :: Schaltzyklus 3",
        en: "Heating circuit auto. 1 :: Monday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_1_TUE_1: TimeRangeParam = TimeRangeParam {
    id: 0x1420,
    name: "HC_AUTO_1_TUE_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Dienstag :: Schaltzyklus 1",
        en: "Heating circuit auto. 1 :: Tuesday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(6, 0),
        end: Time::new_const(22, 0),
    }),
};
pub const HC_AUTO_1_TUE_2: TimeRangeParam = TimeRangeParam {
    id: 0x1421,
    name: "HC_AUTO_1_TUE_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Dienstag :: Schaltzyklus 2",
        en: "Heating circuit auto. 1 :: Tuesday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_1_TUE_3: TimeRangeParam = TimeRangeParam {
    id: 0x1422,
    name: "HC_AUTO_1_TUE_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Dienstag :: Schaltzyklus 3",
        en: "Heating circuit auto. 1 :: Tuesday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_1_WED_1: TimeRangeParam = TimeRangeParam {
    id: 0x1430,
    name: "HC_AUTO_1_WED_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Mittwoch :: Schaltzyklus 1",
        en: "Heating circuit auto. 1 :: Wednesday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(6, 0),
        end: Time::new_const(22, 0),
    }),
};
pub const HC_AUTO_1_WED_2: TimeRangeParam = TimeRangeParam {
    id: 0x1431,
    name: "HC_AUTO_1_WED_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Mittwoch :: Schaltzyklus 2",
        en: "Heating circuit auto. 1 :: Wednesday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_1_WED_3: TimeRangeParam = TimeRangeParam {
    id: 0x1432,
    name: "HC_AUTO_1_WED_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Mittwoch :: Schaltzyklus 3",
        en: "Heating circuit auto. 1 :: Wednesday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_1_THU_1: TimeRangeParam = TimeRangeParam {
    id: 0x1440,
    name: "HC_AUTO_1_THU_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Donnerstag :: Schaltzyklus 1",
        en: "Heating circuit auto. 1 :: Thursday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(6, 0),
        end: Time::new_const(22, 0),
    }),
};
pub const HC_AUTO_1_THU_2: TimeRangeParam = TimeRangeParam {
    id: 0x1441,
    name: "HC_AUTO_1_THU_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Donnerstag :: Schaltzyklus 2",
        en: "Heating circuit auto. 1 :: Thursday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_1_THU_3: TimeRangeParam = TimeRangeParam {
    id: 0x1442,
    name: "HC_AUTO_1_THU_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Donnerstag :: Schaltzyklus 3",
        en: "Heating circuit auto. 1 :: Thursday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_1_FRI_1: TimeRangeParam = TimeRangeParam {
    id: 0x1450,
    name: "HC_AUTO_1_FRI_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Freitag :: Schaltzyklus 1",
        en: "Heating circuit auto. 1 :: Friday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(6, 0),
        end: Time::new_const(22, 0),
    }),
};
pub const HC_AUTO_1_FRI_2: TimeRangeParam = TimeRangeParam {
    id: 0x1451,
    name: "HC_AUTO_1_FRI_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Freitag :: Schaltzyklus 2",
        en: "Heating circuit auto. 1 :: Friday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_1_FRI_3: TimeRangeParam = TimeRangeParam {
    id: 0x1452,
    name: "HC_AUTO_1_FRI_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Freitag :: Schaltzyklus 3",
        en: "Heating circuit auto. 1 :: Friday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_1_SAT_1: TimeRangeParam = TimeRangeParam {
    id: 0x1460,
    name: "HC_AUTO_1_SAT_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Samstag :: Schaltzyklus 1",
        en: "Heating circuit auto. 1 :: Saturday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(7, 0),
        end: Time::new_const(23, 0),
    }),
};
pub const HC_AUTO_1_SAT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1461,
    name: "HC_AUTO_1_SAT_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Samstag :: Schaltzyklus 2",
        en: "Heating circuit auto. 1 :: Saturday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_1_SAT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1462,
    name: "HC_AUTO_1_SAT_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Samstag :: Schaltzyklus 3",
        en: "Heating circuit auto. 1 :: Saturday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_1_SUN_1: TimeRangeParam = TimeRangeParam {
    id: 0x1470,
    name: "HC_AUTO_1_SUN_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Sonntag :: Schaltzyklus 1",
        en: "Heating circuit auto. 1 :: Sunday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(7, 0),
        end: Time::new_const(23, 0),
    }),
};
pub const HC_AUTO_1_SUN_2: TimeRangeParam = TimeRangeParam {
    id: 0x1471,
    name: "HC_AUTO_1_SUN_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Sonntag :: Schaltzyklus 2",
        en: "Heating circuit auto. 1 :: Sunday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_1_SUN_3: TimeRangeParam = TimeRangeParam {
    id: 0x1472,
    name: "HC_AUTO_1_SUN_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Sonntag :: Schaltzyklus 3",
        en: "Heating circuit auto. 1 :: Sunday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_1_MON_FRI_1: TimeRangeParam = TimeRangeParam {
    id: 0x1480,
    name: "HC_AUTO_1_MON_FRI_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Montag – Freitag :: Schaltzyklus 1",
        en: "Heating circuit auto. 1 :: Monday – Friday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_1_MON_FRI_2: TimeRangeParam = TimeRangeParam {
    id: 0x1481,
    name: "HC_AUTO_1_MON_FRI_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Montag – Freitag :: Schaltzyklus 2",
        en: "Heating circuit auto. 1 :: Monday – Friday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_1_MON_FRI_3: TimeRangeParam = TimeRangeParam {
    id: 0x1482,
    name: "HC_AUTO_1_MON_FRI_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Montag – Freitag :: Schaltzyklus 3",
        en: "Heating circuit auto. 1 :: Monday – Friday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_1_SAT_SUN_1: TimeRangeParam = TimeRangeParam {
    id: 0x1490,
    name: "HC_AUTO_1_SAT_SUN_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Samstag, Sonntag :: Schaltzyklus 1",
        en: "Heating circuit auto. 1 :: Saturday, Sunday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_1_SAT_SUN_2: TimeRangeParam = TimeRangeParam {
    id: 0x1491,
    name: "HC_AUTO_1_SAT_SUN_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Samstag, Sonntag :: Schaltzyklus 2",
        en: "Heating circuit auto. 1 :: Saturday, Sunday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_1_SAT_SUN_3: TimeRangeParam = TimeRangeParam {
    id: 0x1492,
    name: "HC_AUTO_1_SAT_SUN_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Samstag, Sonntag :: Schaltzyklus 3",
        en: "Heating circuit auto. 1 :: Saturday, Sunday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_1_MON_SUN_1: TimeRangeParam = TimeRangeParam {
    id: 0x14a0,
    name: "HC_AUTO_1_MON_SUN_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Montag – Sonntag :: Schaltzyklus 1",
        en: "Heating circuit auto. 1 :: Monday – Sunday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_1_MON_SUN_2: TimeRangeParam = TimeRangeParam {
    id: 0x14a1,
    name: "HC_AUTO_1_MON_SUN_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Montag – Sonntag :: Schaltzyklus 2",
        en: "Heating circuit auto. 1 :: Monday – Sunday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_1_MON_SUN_3: TimeRangeParam = TimeRangeParam {
    id: 0x14a2,
    name: "HC_AUTO_1_MON_SUN_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Montag – Sonntag :: Schaltzyklus 3",
        en: "Heating circuit auto. 1 :: Monday – Sunday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_1_MON_THU_1: TimeRangeParam = TimeRangeParam {
    id: 0x14b0,
    name: "HC_AUTO_1_MON_THU_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Montag – Donnerstag :: Schaltzyklus 1",
        en: "Heating circuit auto. 1 :: Monday – Thursday :: Switching cycle 1",
    },
    read: false,
    write: false,
    default: None,
};
pub const HC_AUTO_1_MON_THU_2: TimeRangeParam = TimeRangeParam {
    id: 0x14b1,
    name: "HC_AUTO_1_MON_THU_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Montag – Donnerstag :: Schaltzyklus 2",
        en: "Heating circuit auto. 1 :: Monday – Thursday :: Switching cycle 2",
    },
    read: false,
    write: false,
    default: None,
};
pub const HC_AUTO_1_MON_THU_3: TimeRangeParam = TimeRangeParam {
    id: 0x14b2,
    name: "HC_AUTO_1_MON_THU_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 1 :: Montag – Donnerstag :: Schaltzyklus 3",
        en: "Heating circuit auto. 1 :: Monday – Thursday :: Switching cycle 3",
    },
    read: false,
    write: false,
    default: None,
};
pub const HC_AUTO_2: TimeRangeParam = TimeRangeParam {
    id: 0x1500,
    name: "HC_AUTO_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2",
        en: "Heating circuit auto. 2",
    },
    read: false,
    write: false,
    default: None,
};
pub const HC_AUTO_2_MON_1: TimeRangeParam = TimeRangeParam {
    id: 0x1510,
    name: "HC_AUTO_2_MON_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Montag :: Schaltzyklus 1",
        en: "Heating circuit auto. 2 :: Monday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(6, 0),
        end: Time::new_const(8, 0),
    }),
};
pub const HC_AUTO_2_MON_2: TimeRangeParam = TimeRangeParam {
    id: 0x1511,
    name: "HC_AUTO_2_MON_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Montag :: Schaltzyklus 2",
        en: "Heating circuit auto. 2 :: Monday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_2_MON_3: TimeRangeParam = TimeRangeParam {
    id: 0x1512,
    name: "HC_AUTO_2_MON_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Montag :: Schaltzyklus 3",
        en: "Heating circuit auto. 2 :: Monday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_2_TUE_1: TimeRangeParam = TimeRangeParam {
    id: 0x1520,
    name: "HC_AUTO_2_TUE_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Dienstag :: Schaltzyklus 1",
        en: "Heating circuit auto. 2 :: Tuesday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(6, 0),
        end: Time::new_const(8, 0),
    }),
};
pub const HC_AUTO_2_TUE_2: TimeRangeParam = TimeRangeParam {
    id: 0x1521,
    name: "HC_AUTO_2_TUE_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Dienstag :: Schaltzyklus 2",
        en: "Heating circuit auto. 2 :: Tuesday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_2_TUE_3: TimeRangeParam = TimeRangeParam {
    id: 0x1522,
    name: "HC_AUTO_2_TUE_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Dienstag :: Schaltzyklus 3",
        en: "Heating circuit auto. 2 :: Tuesday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_2_WED_1: TimeRangeParam = TimeRangeParam {
    id: 0x1530,
    name: "HC_AUTO_2_WED_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Mittwoch :: Schaltzyklus 1",
        en: "Heating circuit auto. 2 :: Wednesday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(6, 0),
        end: Time::new_const(8, 0),
    }),
};
pub const HC_AUTO_2_WED_2: TimeRangeParam = TimeRangeParam {
    id: 0x1531,
    name: "HC_AUTO_2_WED_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Mittwoch :: Schaltzyklus 2",
        en: "Heating circuit auto. 2 :: Wednesday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_2_WED_3: TimeRangeParam = TimeRangeParam {
    id: 0x1532,
    name: "HC_AUTO_2_WED_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Mittwoch :: Schaltzyklus 3",
        en: "Heating circuit auto. 2 :: Wednesday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_2_THU_1: TimeRangeParam = TimeRangeParam {
    id: 0x1540,
    name: "HC_AUTO_2_THU_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Donnerstag :: Schaltzyklus 1",
        en: "Heating circuit auto. 2 :: Thursday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(6, 0),
        end: Time::new_const(8, 0),
    }),
};
pub const HC_AUTO_2_THU_2: TimeRangeParam = TimeRangeParam {
    id: 0x1541,
    name: "HC_AUTO_2_THU_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Donnerstag :: Schaltzyklus 2",
        en: "Heating circuit auto. 2 :: Thursday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_2_THU_3: TimeRangeParam = TimeRangeParam {
    id: 0x1542,
    name: "HC_AUTO_2_THU_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Donnerstag :: Schaltzyklus 3",
        en: "Heating circuit auto. 2 :: Thursday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_2_FRI_1: TimeRangeParam = TimeRangeParam {
    id: 0x1550,
    name: "HC_AUTO_2_FRI_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Freitag :: Schaltzyklus 1",
        en: "Heating circuit auto. 2 :: Friday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(6, 0),
        end: Time::new_const(8, 0),
    }),
};
pub const HC_AUTO_2_FRI_2: TimeRangeParam = TimeRangeParam {
    id: 0x1551,
    name: "HC_AUTO_2_FRI_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Freitag :: Schaltzyklus 2",
        en: "Heating circuit auto. 2 :: Friday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_2_FRI_3: TimeRangeParam = TimeRangeParam {
    id: 0x1552,
    name: "HC_AUTO_2_FRI_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Freitag :: Schaltzyklus 3",
        en: "Heating circuit auto. 2 :: Friday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_2_SAT_1: TimeRangeParam = TimeRangeParam {
    id: 0x1560,
    name: "HC_AUTO_2_SAT_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Samstag :: Schaltzyklus 1",
        en: "Heating circuit auto. 2 :: Saturday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(7, 0),
        end: Time::new_const(23, 0),
    }),
};
pub const HC_AUTO_2_SAT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1561,
    name: "HC_AUTO_2_SAT_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Samstag :: Schaltzyklus 2",
        en: "Heating circuit auto. 2 :: Saturday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_2_SAT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1562,
    name: "HC_AUTO_2_SAT_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Samstag :: Schaltzyklus 3",
        en: "Heating circuit auto. 2 :: Saturday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const HC_AUTO_2_SUN_1: TimeRangeParam = TimeRangeParam {
    id: 0x1570,
    name: "HC_AUTO_2_SUN_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Sonntag :: Schaltzyklus 1",
        en: "Heating circuit auto. 2 :: Sunday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(7, 0),
        end: Time::new_const(23, 0),
    }),
};
pub const HC_AUTO_2_SUN_2: TimeRangeParam = TimeRangeParam {
    id: 0x1571,
    name: "HC_AUTO_2_SUN_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Sonntag :: Schaltzyklus 2",
        en: "Heating circuit auto. 2 :: Sunday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_2_SUN_3: TimeRangeParam = TimeRangeParam {
    id: 0x1572,
    name: "HC_AUTO_2_SUN_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Sonntag :: Schaltzyklus 3",
        en: "Heating circuit auto. 2 :: Sunday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_2_MON_FRI_1: TimeRangeParam = TimeRangeParam {
    id: 0x1580,
    name: "HC_AUTO_2_MON_FRI_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Montag – Freitag :: Schaltzyklus 1",
        en: "Heating circuit auto. 2 :: Monday – Friday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_2_MON_FRI_2: TimeRangeParam = TimeRangeParam {
    id: 0x1581,
    name: "HC_AUTO_2_MON_FRI_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Montag – Freitag :: Schaltzyklus 2",
        en: "Heating circuit auto. 2 :: Monday – Friday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_2_MON_FRI_3: TimeRangeParam = TimeRangeParam {
    id: 0x1582,
    name: "HC_AUTO_2_MON_FRI_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Montag – Freitag :: Schaltzyklus 3",
        en: "Heating circuit auto. 2 :: Monday – Friday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_2_SAT_SUN_1: TimeRangeParam = TimeRangeParam {
    id: 0x1590,
    name: "HC_AUTO_2_SAT_SUN_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Samstag, Sonntag :: Schaltzyklus 1",
        en: "Heating circuit auto. 2 :: Saturday, Sunday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_2_SAT_SUN_2: TimeRangeParam = TimeRangeParam {
    id: 0x1591,
    name: "HC_AUTO_2_SAT_SUN_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Samstag, Sonntag :: Schaltzyklus 2",
        en: "Heating circuit auto. 2 :: Saturday, Sunday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_2_SAT_SUN_3: TimeRangeParam = TimeRangeParam {
    id: 0x1592,
    name: "HC_AUTO_2_SAT_SUN_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Samstag, Sonntag :: Schaltzyklus 3",
        en: "Heating circuit auto. 2 :: Saturday, Sunday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_2_MON_SUN_1: TimeRangeParam = TimeRangeParam {
    id: 0x15a0,
    name: "HC_AUTO_2_MON_SUN_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Montag – Sonntag :: Schaltzyklus 1",
        en: "Heating circuit auto. 2 :: Monday – Sunday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_2_MON_SUN_2: TimeRangeParam = TimeRangeParam {
    id: 0x15a1,
    name: "HC_AUTO_2_MON_SUN_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Montag – Sonntag :: Schaltzyklus 2",
        en: "Heating circuit auto. 2 :: Monday – Sunday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_2_MON_SUN_3: TimeRangeParam = TimeRangeParam {
    id: 0x15a2,
    name: "HC_AUTO_2_MON_SUN_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Montag – Sonntag :: Schaltzyklus 3",
        en: "Heating circuit auto. 2 :: Monday – Sunday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const HC_AUTO_2_MON_THU_1: TimeRangeParam = TimeRangeParam {
    id: 0x15b0,
    name: "HC_AUTO_2_MON_THU_1",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Montag – Donnerstag :: Schaltzyklus 1",
        en: "Heating circuit auto. 2 :: Monday – Thursday :: Switching cycle 1",
    },
    read: false,
    write: false,
    default: None,
};
pub const HC_AUTO_2_MON_THU_2: TimeRangeParam = TimeRangeParam {
    id: 0x15b1,
    name: "HC_AUTO_2_MON_THU_2",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Montag – Donnerstag :: Schaltzyklus 2",
        en: "Heating circuit auto. 2 :: Monday – Thursday :: Switching cycle 2",
    },
    read: false,
    write: false,
    default: None,
};
pub const HC_AUTO_2_MON_THU_3: TimeRangeParam = TimeRangeParam {
    id: 0x15b2,
    name: "HC_AUTO_2_MON_THU_3",
    label: MultilingualStr {
        de: "Heizkreis Automatik 2 :: Montag – Donnerstag :: Schaltzyklus 3",
        en: "Heating circuit auto. 2 :: Monday – Thursday :: Switching cycle 3",
    },
    read: false,
    write: false,
    default: None,
};
pub const DHW_AUTO_1: TimeRangeParam = TimeRangeParam {
    id: 0x1700,
    name: "DHW_AUTO_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1",
        en: "Hot water auto. 1",
    },
    read: false,
    write: false,
    default: None,
};
pub const DHW_AUTO_1_MON_1: TimeRangeParam = TimeRangeParam {
    id: 0x1710,
    name: "DHW_AUTO_1_MON_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Montag :: Schaltzeit 1",
        en: "Hot water auto. 1 :: Monday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(23, 45),
    }),
};
pub const DHW_AUTO_1_MON_2: TimeRangeParam = TimeRangeParam {
    id: 0x1711,
    name: "DHW_AUTO_1_MON_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Montag :: Schaltzeit 2",
        en: "Hot water auto. 1 :: Monday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_1_MON_3: TimeRangeParam = TimeRangeParam {
    id: 0x1712,
    name: "DHW_AUTO_1_MON_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Montag :: Schaltzeit 3",
        en: "Hot water auto. 1 :: Monday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_1_TUE_1: TimeRangeParam = TimeRangeParam {
    id: 0x1720,
    name: "DHW_AUTO_1_TUE_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Dienstag :: Schaltzeit 1",
        en: "Hot water auto. 1 :: Tuesday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(23, 45),
    }),
};
pub const DHW_AUTO_1_TUE_2: TimeRangeParam = TimeRangeParam {
    id: 0x1721,
    name: "DHW_AUTO_1_TUE_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Dienstag :: Schaltzeit 2",
        en: "Hot water auto. 1 :: Tuesday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_1_TUE_3: TimeRangeParam = TimeRangeParam {
    id: 0x1722,
    name: "DHW_AUTO_1_TUE_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Dienstag :: Schaltzeit 3",
        en: "Hot water auto. 1 :: Tuesday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_1_WED_1: TimeRangeParam = TimeRangeParam {
    id: 0x1730,
    name: "DHW_AUTO_1_WED_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Mittwoch :: Schaltzeit 1",
        en: "Hot water auto. 1 :: Wednesday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(23, 45),
    }),
};
pub const DHW_AUTO_1_WED_2: TimeRangeParam = TimeRangeParam {
    id: 0x1731,
    name: "DHW_AUTO_1_WED_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Mittwoch :: Schaltzeit 2",
        en: "Hot water auto. 1 :: Wednesday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_1_WED_3: TimeRangeParam = TimeRangeParam {
    id: 0x1732,
    name: "DHW_AUTO_1_WED_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Mittwoch :: Schaltzeit 3",
        en: "Hot water auto. 1 :: Wednesday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_1_THU_1: TimeRangeParam = TimeRangeParam {
    id: 0x1740,
    name: "DHW_AUTO_1_THU_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Donnerstag :: Schaltzeit 1",
        en: "Hot water auto. 1 :: Thursday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(23, 45),
    }),
};
pub const DHW_AUTO_1_THU_2: TimeRangeParam = TimeRangeParam {
    id: 0x1741,
    name: "DHW_AUTO_1_THU_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Donnerstag :: Schaltzeit 2",
        en: "Hot water auto. 1 :: Thursday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_1_THU_3: TimeRangeParam = TimeRangeParam {
    id: 0x1742,
    name: "DHW_AUTO_1_THU_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Donnerstag :: Schaltzeit 3",
        en: "Hot water auto. 1 :: Thursday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_1_FRI_1: TimeRangeParam = TimeRangeParam {
    id: 0x1750,
    name: "DHW_AUTO_1_FRI_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Freitag :: Schaltzeit 1",
        en: "Hot water auto. 1 :: Friday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(23, 45),
    }),
};
pub const DHW_AUTO_1_FRI_2: TimeRangeParam = TimeRangeParam {
    id: 0x1751,
    name: "DHW_AUTO_1_FRI_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Freitag :: Schaltzeit 2",
        en: "Hot water auto. 1 :: Friday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_1_FRI_3: TimeRangeParam = TimeRangeParam {
    id: 0x1752,
    name: "DHW_AUTO_1_FRI_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Freitag :: Schaltzeit 3",
        en: "Hot water auto. 1 :: Friday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_1_SAT_1: TimeRangeParam = TimeRangeParam {
    id: 0x1760,
    name: "DHW_AUTO_1_SAT_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Samstag :: Schaltzeit 1",
        en: "Hot water auto. 1 :: Saturday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(23, 45),
    }),
};
pub const DHW_AUTO_1_SAT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1761,
    name: "DHW_AUTO_1_SAT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Samstag :: Schaltzeit 2",
        en: "Hot water auto. 1 :: Saturday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_1_SAT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1762,
    name: "DHW_AUTO_1_SAT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Samstag :: Schaltzeit 3",
        en: "Hot water auto. 1 :: Saturday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_1_SUN_1: TimeRangeParam = TimeRangeParam {
    id: 0x1770,
    name: "DHW_AUTO_1_SUN_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Sonntag :: Schaltzeit 1",
        en: "Hot water auto. 1 :: Sunday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(23, 45),
    }),
};
pub const DHW_AUTO_1_SUN_2: TimeRangeParam = TimeRangeParam {
    id: 0x1771,
    name: "DHW_AUTO_1_SUN_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Sonntag :: Schaltzeit 2",
        en: "Hot water auto. 1 :: Sunday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_1_SUN_3: TimeRangeParam = TimeRangeParam {
    id: 0x1772,
    name: "DHW_AUTO_1_SUN_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Sonntag :: Schaltzeit 3",
        en: "Hot water auto. 1 :: Sunday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_1_MON_FRI_1: TimeRangeParam = TimeRangeParam {
    id: 0x1780,
    name: "DHW_AUTO_1_MON_FRI_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Montag – Freitag :: Schaltzeit 1",
        en: "Hot water auto. 1 :: Monday to Friday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_1_MON_FRI_2: TimeRangeParam = TimeRangeParam {
    id: 0x1781,
    name: "DHW_AUTO_1_MON_FRI_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Montag – Freitag :: Schaltzeit 2",
        en: "Hot water auto. 1 :: Monday to Friday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_1_MON_FRI_3: TimeRangeParam = TimeRangeParam {
    id: 0x1782,
    name: "DHW_AUTO_1_MON_FRI_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Montag – Freitag :: Schaltzeit 3",
        en: "Hot water auto. 1 :: Monday to Friday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_1_SAT_SUN_1: TimeRangeParam = TimeRangeParam {
    id: 0x1790,
    name: "DHW_AUTO_1_SAT_SUN_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Samstag und Sonntag :: Schaltzeit 1",
        en: "Hot water auto. 1 :: Saturday to Sunday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_1_SAT_SUN_2: TimeRangeParam = TimeRangeParam {
    id: 0x1791,
    name: "DHW_AUTO_1_SAT_SUN_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Samstag und Sonntag :: Schaltzeit 2",
        en: "Hot water auto. 1 :: Saturday to Sunday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_1_SAT_SUN_3: TimeRangeParam = TimeRangeParam {
    id: 0x1792,
    name: "DHW_AUTO_1_SAT_SUN_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Samstag und Sonntag :: Schaltzeit 3",
        en: "Hot water auto. 1 :: Saturday to Sunday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_1_MON_SUN_1: TimeRangeParam = TimeRangeParam {
    id: 0x17a0,
    name: "DHW_AUTO_1_MON_SUN_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Montag – Sonntag :: Schaltzeit 1",
        en: "Hot water auto. 1 :: Monday to Sunday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_1_MON_SUN_2: TimeRangeParam = TimeRangeParam {
    id: 0x17a1,
    name: "DHW_AUTO_1_MON_SUN_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Montag – Sonntag :: Schaltzeit 2",
        en: "Hot water auto. 1 :: Monday to Sunday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_1_MON_SUN_3: TimeRangeParam = TimeRangeParam {
    id: 0x17a2,
    name: "DHW_AUTO_1_MON_SUN_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Montag – Sonntag :: Schaltzeit 3",
        en: "Hot water auto. 1 :: Monday to Sunday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_1_MON_TUE_1: TimeRangeParam = TimeRangeParam {
    id: 0x17b0,
    name: "DHW_AUTO_1_MON_TUE_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Montag – Donnerstag :: Schaltzeit 1",
        en: "Hot water auto. 1 :: Monday to Thursday :: Switching cycle 1",
    },
    read: false,
    write: false,
    default: None,
};
pub const DHW_AUTO_1_MON_TUE_2: TimeRangeParam = TimeRangeParam {
    id: 0x17b1,
    name: "DHW_AUTO_1_MON_TUE_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Montag – Donnerstag :: Schaltzeit 2",
        en: "Hot water auto. 1 :: Monday to Thursday :: Switching cycle 2",
    },
    read: false,
    write: false,
    default: None,
};
pub const DHW_AUTO_1_MON_TUE_3: TimeRangeParam = TimeRangeParam {
    id: 0x17b2,
    name: "DHW_AUTO_1_MON_TUE_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 1 :: Montag – Donnerstag :: Schaltzeit 3",
        en: "Hot water auto. 1 :: Monday to Thursday :: Switching cycle 3",
    },
    read: false,
    write: false,
    default: None,
};
pub const DHW_AUTO_2: TimeRangeParam = TimeRangeParam {
    id: 0x1800,
    name: "DHW_AUTO_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2",
        en: "Hot water auto. 2",
    },
    read: false,
    write: false,
    default: None,
};
pub const DHW_AUTO_2_MON_1: TimeRangeParam = TimeRangeParam {
    id: 0x1810,
    name: "DHW_AUTO_2_MON_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Montag :: Schaltzeit 1",
        en: "Hot water auto. 2 :: Monday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(5, 0),
        end: Time::new_const(21, 0),
    }),
};
pub const DHW_AUTO_2_MON_2: TimeRangeParam = TimeRangeParam {
    id: 0x1811,
    name: "DHW_AUTO_2_MON_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Montag :: Schaltzeit 2",
        en: "Hot water auto. 2 :: Monday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_2_MON_3: TimeRangeParam = TimeRangeParam {
    id: 0x1812,
    name: "DHW_AUTO_2_MON_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Montag :: Schaltzeit 3",
        en: "Hot water auto. 2 :: Monday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_2_TUE_1: TimeRangeParam = TimeRangeParam {
    id: 0x1820,
    name: "DHW_AUTO_2_TUE_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Dienstag :: Schaltzeit 1",
        en: "Hot water auto. 2 :: Tuesday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(5, 0),
        end: Time::new_const(21, 0),
    }),
};
pub const DHW_AUTO_2_TUE_2: TimeRangeParam = TimeRangeParam {
    id: 0x1821,
    name: "DHW_AUTO_2_TUE_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Dienstag :: Schaltzeit 2",
        en: "Hot water auto. 2 :: Tuesday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_2_TUE_3: TimeRangeParam = TimeRangeParam {
    id: 0x1822,
    name: "DHW_AUTO_2_TUE_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Dienstag :: Schaltzeit 3",
        en: "Hot water auto. 2 :: Tuesday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_2_WED_1: TimeRangeParam = TimeRangeParam {
    id: 0x1830,
    name: "DHW_AUTO_2_WED_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Mittwoch :: Schaltzeit 1",
        en: "Hot water auto. 2 :: Wednesday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(5, 0),
        end: Time::new_const(21, 0),
    }),
};
pub const DHW_AUTO_2_WED_2: TimeRangeParam = TimeRangeParam {
    id: 0x1831,
    name: "DHW_AUTO_2_WED_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Mittwoch :: Schaltzeit 2",
        en: "Hot water auto. 2 :: Wednesday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_2_WED_3: TimeRangeParam = TimeRangeParam {
    id: 0x1832,
    name: "DHW_AUTO_2_WED_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Mittwoch :: Schaltzeit 3",
        en: "Hot water auto. 2 :: Wednesday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_2_THU_1: TimeRangeParam = TimeRangeParam {
    id: 0x1840,
    name: "DHW_AUTO_2_THU_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Donnerstag :: Schaltzeit 1",
        en: "Hot water auto. 2 :: Thursday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(5, 0),
        end: Time::new_const(21, 0),
    }),
};
pub const DHW_AUTO_2_THU_2: TimeRangeParam = TimeRangeParam {
    id: 0x1841,
    name: "DHW_AUTO_2_THU_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Donnerstag :: Schaltzeit 2",
        en: "Hot water auto. 2 :: Thursday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_2_THU_3: TimeRangeParam = TimeRangeParam {
    id: 0x1842,
    name: "DHW_AUTO_2_THU_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Donnerstag :: Schaltzeit 3",
        en: "Hot water auto. 2 :: Thursday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_2_FRI_1: TimeRangeParam = TimeRangeParam {
    id: 0x1850,
    name: "DHW_AUTO_2_FRI_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Freitag :: Schaltzeit 1",
        en: "Hot water auto. 2 :: Friday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(5, 0),
        end: Time::new_const(21, 0),
    }),
};
pub const DHW_AUTO_2_FRI_2: TimeRangeParam = TimeRangeParam {
    id: 0x1851,
    name: "DHW_AUTO_2_FRI_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Freitag :: Schaltzeit 2",
        en: "Hot water auto. 2 :: Friday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_2_FRI_3: TimeRangeParam = TimeRangeParam {
    id: 0x1852,
    name: "DHW_AUTO_2_FRI_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Freitag :: Schaltzeit 3",
        en: "Hot water auto. 2 :: Friday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_2_SAT_1: TimeRangeParam = TimeRangeParam {
    id: 0x1860,
    name: "DHW_AUTO_2_SAT_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Samstag :: Schaltzeit 1",
        en: "Hot water auto. 2 :: Saturday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(6, 0),
        end: Time::new_const(22, 0),
    }),
};
pub const DHW_AUTO_2_SAT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1861,
    name: "DHW_AUTO_2_SAT_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Samstag :: Schaltzeit 2",
        en: "Hot water auto. 2 :: Saturday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_2_SAT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1862,
    name: "DHW_AUTO_2_SAT_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Samstag :: Schaltzeit 3",
        en: "Hot water auto. 2 :: Saturday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_2_SUN_1: TimeRangeParam = TimeRangeParam {
    id: 0x1870,
    name: "DHW_AUTO_2_SUN_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Sonntag :: Schaltzeit 1",
        en: "Hot water auto. 2 :: Sunday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(6, 0),
        end: Time::new_const(22, 0),
    }),
};
pub const DHW_AUTO_2_SUN_2: TimeRangeParam = TimeRangeParam {
    id: 0x1871,
    name: "DHW_AUTO_2_SUN_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Sonntag :: Schaltzeit 2",
        en: "Hot water auto. 2 :: Sunday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_2_SUN_3: TimeRangeParam = TimeRangeParam {
    id: 0x1872,
    name: "DHW_AUTO_2_SUN_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Sonntag :: Schaltzeit 3",
        en: "Hot water auto. 2 :: Sunday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: Some(TimeRange {
        start: Time::new_const(0, 0),
        end: Time::new_const(0, 0),
    }),
};
pub const DHW_AUTO_2_MON_FRI_1: TimeRangeParam = TimeRangeParam {
    id: 0x1880,
    name: "DHW_AUTO_2_MON_FRI_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Montag – Freitag :: Schaltzeit 1",
        en: "Hot water auto. 2 :: Monday – Friday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_2_MON_FRI_2: TimeRangeParam = TimeRangeParam {
    id: 0x1881,
    name: "DHW_AUTO_2_MON_FRI_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Montag – Freitag :: Schaltzeit 2",
        en: "Hot water auto. 2 :: Monday – Friday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_2_MON_FRI_3: TimeRangeParam = TimeRangeParam {
    id: 0x1882,
    name: "DHW_AUTO_2_MON_FRI_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Montag – Freitag :: Schaltzeit 3",
        en: "Hot water auto. 2 :: Monday – Friday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_2_SAT_SUN_1: TimeRangeParam = TimeRangeParam {
    id: 0x1890,
    name: "DHW_AUTO_2_SAT_SUN_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Samstag, Sonntag :: Schaltzeit 1",
        en: "Hot water auto. 2 :: Saturday, Sunday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_2_SAT_SUN_2: TimeRangeParam = TimeRangeParam {
    id: 0x1891,
    name: "DHW_AUTO_2_SAT_SUN_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Samstag, Sonntag :: Schaltzeit 2",
        en: "Hot water auto. 2 :: Saturday, Sunday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_2_SAT_SUN_3: TimeRangeParam = TimeRangeParam {
    id: 0x1892,
    name: "DHW_AUTO_2_SAT_SUN_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Samstag, Sonntag :: Schaltzeit 3",
        en: "Hot water auto. 2 :: Saturday, Sunday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_2_MON_SUN_1: TimeRangeParam = TimeRangeParam {
    id: 0x18a0,
    name: "DHW_AUTO_2_MON_SUN_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Montag – Sonntag :: Schaltzeit 1",
        en: "Hot water auto. 2 :: Monday – Sunday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_2_MON_SUN_2: TimeRangeParam = TimeRangeParam {
    id: 0x18a1,
    name: "DHW_AUTO_2_MON_SUN_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Montag – Sonntag :: Schaltzeit 2",
        en: "Hot water auto. 2 :: Monday – Sunday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_2_MON_SUN_3: TimeRangeParam = TimeRangeParam {
    id: 0x18a2,
    name: "DHW_AUTO_2_MON_SUN_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Montag – Sonntag :: Schaltzeit 3",
        en: "Hot water auto. 2 :: Monday – Sunday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const DHW_AUTO_2_MON_TUE_1: TimeRangeParam = TimeRangeParam {
    id: 0x18b0,
    name: "DHW_AUTO_2_MON_TUE_1",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Montag – Donnerstag :: Schaltzeit 1",
        en: "Hot water auto. 2 :: Monday – Thursday :: Switching cycle 1",
    },
    read: false,
    write: false,
    default: None,
};
pub const DHW_AUTO_2_MON_TUE_2: TimeRangeParam = TimeRangeParam {
    id: 0x18b1,
    name: "DHW_AUTO_2_MON_TUE_2",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Montag – Donnerstag :: Schaltzeit 2",
        en: "Hot water auto. 2 :: Monday – Thursday :: Switching cycle 2",
    },
    read: false,
    write: false,
    default: None,
};
pub const DHW_AUTO_2_MON_TUE_3: TimeRangeParam = TimeRangeParam {
    id: 0x18b2,
    name: "DHW_AUTO_2_MON_TUE_3",
    label: MultilingualStr {
        de: "Warmwasserprogramm 2 :: Montag – Donnerstag :: Schaltzeit 3",
        en: "Hot water auto. 2 :: Monday – Thursday :: Switching cycle 3",
    },
    read: false,
    write: false,
    default: None,
};
pub const CIRCULATION_MON_1: TimeRangeParam = TimeRangeParam {
    id: 0x1a10,
    name: "CIRCULATION_MON_1",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Montag :: Schaltzyklus 1",
        en: "Circulation program :: Monday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_MON_2: TimeRangeParam = TimeRangeParam {
    id: 0x1a11,
    name: "CIRCULATION_MON_2",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Montag :: Schaltzyklus 2",
        en: "Circulation program :: Monday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_MON_3: TimeRangeParam = TimeRangeParam {
    id: 0x1a12,
    name: "CIRCULATION_MON_3",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Montag :: Schaltzyklus 3",
        en: "Circulation program :: Monday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_TUE_1: TimeRangeParam = TimeRangeParam {
    id: 0x1a20,
    name: "CIRCULATION_TUE_1",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Dienstag :: Schaltzyklus 1",
        en: "Circulation program :: Tuesday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_TUE_2: TimeRangeParam = TimeRangeParam {
    id: 0x1a21,
    name: "CIRCULATION_TUE_2",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Dienstag :: Schaltzyklus 2",
        en: "Circulation program :: Tuesday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_TUE_3: TimeRangeParam = TimeRangeParam {
    id: 0x1a22,
    name: "CIRCULATION_TUE_3",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Dienstag :: Schaltzyklus 3",
        en: "Circulation program :: Tuesday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_WED_1: TimeRangeParam = TimeRangeParam {
    id: 0x1a30,
    name: "CIRCULATION_WED_1",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Mittwoch :: Schaltzyklus 1",
        en: "Circulation program :: Wednesday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_WED_2: TimeRangeParam = TimeRangeParam {
    id: 0x1a31,
    name: "CIRCULATION_WED_2",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Mittwoch :: Schaltzyklus 2",
        en: "Circulation program :: Wednesday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_WED_3: TimeRangeParam = TimeRangeParam {
    id: 0x1a32,
    name: "CIRCULATION_WED_3",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Mittwoch :: Schaltzyklus 3",
        en: "Circulation program :: Wednesday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_THU_1: TimeRangeParam = TimeRangeParam {
    id: 0x1a40,
    name: "CIRCULATION_THU_1",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Donnerstag :: Schaltzyklus 1",
        en: "Circulation program :: Thursday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_THU_2: TimeRangeParam = TimeRangeParam {
    id: 0x1a41,
    name: "CIRCULATION_THU_2",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Donnerstag :: Schaltzyklus 2",
        en: "Circulation program :: Thursday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_THU_3: TimeRangeParam = TimeRangeParam {
    id: 0x1a42,
    name: "CIRCULATION_THU_3",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Donnerstag :: Schaltzyklus 3",
        en: "Circulation program :: Thursday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_FRI_1: TimeRangeParam = TimeRangeParam {
    id: 0x1a50,
    name: "CIRCULATION_FRI_1",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Friday :: Schaltzyklus 1",
        en: "Circulation program :: Friday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCUATLION_FRI_2: TimeRangeParam = TimeRangeParam {
    id: 0x1a51,
    name: "CIRCUATLION_FRI_2",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Friday :: Schaltzyklus 2",
        en: "Circulation program :: Friday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_FRI_3: TimeRangeParam = TimeRangeParam {
    id: 0x1a52,
    name: "CIRCULATION_FRI_3",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Friday :: Schaltzyklus 3",
        en: "Circulation program :: Friday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_SAT_1: TimeRangeParam = TimeRangeParam {
    id: 0x1a60,
    name: "CIRCULATION_SAT_1",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Samstag :: Schaltzyklus 1",
        en: "Circulation program :: Saturday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_SAT_2: TimeRangeParam = TimeRangeParam {
    id: 0x1a61,
    name: "CIRCULATION_SAT_2",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Samstag :: Schaltzyklus 2",
        en: "Circulation program :: Saturday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_SAT_3: TimeRangeParam = TimeRangeParam {
    id: 0x1a62,
    name: "CIRCULATION_SAT_3",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Samstag :: Schaltzyklus 3",
        en: "Circulation program :: Saturday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_SUN_1: TimeRangeParam = TimeRangeParam {
    id: 0x1a70,
    name: "CIRCULATION_SUN_1",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Sonntag :: Schaltzyklus 1",
        en: "Circulation program :: Sunday :: Switching cycle 1",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_SUN_2: TimeRangeParam = TimeRangeParam {
    id: 0x1a71,
    name: "CIRCULATION_SUN_2",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Sonntag :: Schaltzyklus 2",
        en: "Circulation program :: Sunday :: Switching cycle 2",
    },
    read: true,
    write: true,
    default: None,
};
pub const CIRCULATION_SUN_3: TimeRangeParam = TimeRangeParam {
    id: 0x1a72,
    name: "CIRCULATION_SUN_3",
    label: MultilingualStr {
        de: "Zirkulationsprogramm :: Sonntag :: Schaltzyklus 3",
        en: "Circulation program :: Sunday :: Switching cycle 3",
    },
    read: true,
    write: true,
    default: None,
};
pub const HOT_WATER_BLOCKING_TIME: I16Param = I16Param {
    id: 0x4e3f,
    name: "HOT_WATER_BLOCKING_TIME",
    label: MultilingualStr {
        de: "Warmwasser Sperrzeit",
        en: "Hot water blocking time",
    },
    read: true,
    write: true,
    unit: Some(Unit::Minutes),
    default: Some(30),
    min: Some(10),
    max: Some(180),
};
pub const VMIN_GCU: DecParam = DecParam {
    id: 0xc09d,
    name: "VMIN_GCU",
    label: MultilingualStr {
        de: "Durchfluss (VMIN_GCU)",
        en: "Volume flow",
    },
    read: true,
    write: false,
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
    read: true,
    write: false,
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
    read: true,
    write: false,
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
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const MYSTERY_C0B4: I16Param = I16Param {
    id: 0xc0b4,
    name: "MYSTERY_C0B4",
    label: MultilingualStr {
        de: "Mystery 0xc0b4",
        en: "Mystery 0xc0b4",
    },
    read: true,
    write: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const OUTSIDE_TEMP_SENSOR: Enum16Param<enums::OutsideTempSensor> = Enum16Param {
    id: 0xc0c4,
    name: "OUTSIDE_TEMP_SENSOR",
    label: MultilingualStr {
        de: "Außentemperaturfühler",
        en: "Outside temperature sensor",
    },
    read: true,
    write: true,
    default: None,
};
pub const MODE: Enum16Param<enums::Mode> = Enum16Param {
    id: 0xc0f6,
    name: "MODE",
    label: MultilingualStr {
        de: "Modus",
        en: "Mode",
    },
    read: true,
    write: false,
    default: None,
};
pub const PWM_SIGNAL: I8Param = I8Param {
    id: 0xc0f7,
    name: "PWM_SIGNAL",
    label: MultilingualStr {
        de: "PWM_SIGNAL",
        en: "PWM signal",
    },
    read: true,
    write: false,
    unit: None,
    default: None,
    min: None,
    max: None,
};
pub const EXTERNAL_REQUEST: Enum16Param<enums::ExternalRequest> = Enum16Param {
    id: 0xc0f8,
    name: "EXTERNAL_REQUEST",
    label: MultilingualStr {
        de: "Smart Grid Signal",
        en: "External request",
    },
    read: true,
    write: false,
    default: None,
};
pub const BUH_CURRENT_OUTPUT: DecParam = DecParam {
    id: 0xc0f9,
    name: "BUH_CURRENT_OUTPUT",
    label: MultilingualStr {
        de: "BUH Leistungsaufnahme",
        en: "BUH current output",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWatt),
    scale: 3,
    default: None,
    min: None,
    max: None,
};
pub const ROOM_THERMOSTAT_INTERLINK: Enum16Param<enums::RoomThermostatInterlink> = Enum16Param {
    id: 0xc0fa,
    name: "ROOM_THERMOSTAT_INTERLINK",
    label: MultilingualStr {
        de: "RT / Interlink",
        en: "Room thermostat/Interlink",
    },
    read: true,
    write: false,
    default: None,
};
pub const MIX_3UVB1: I16Param = I16Param {
    id: 0xc0fb,
    name: "MIX_3UVB1",
    label: MultilingualStr {
        de: "3UVB1 Position (B1)",
        en: "3UVB1 position (B1)",
    },
    read: true,
    write: false,
    unit: Some(Unit::Percent),
    default: None,
    min: Some(0),
    max: Some(100),
};
pub const FEED_TEMP_PHX: DecParam = DecParam {
    id: 0xc0fc,
    name: "FEED_TEMP_PHX",
    label: MultilingualStr {
        de: "Vorlauftemperatur PWT (t-V)",
        en: "Feed temperature PHX (t-V)",
    },
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const FEED_TEMP_BUH: DecParam = DecParam {
    id: 0xc0fe,
    name: "FEED_TEMP_BUH",
    label: MultilingualStr {
        de: "Vorlauftemperatur BUH (t-V,BH)",
        en: "Feed temperature BUH (t-V,BH)",
    },
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const V: I16Param = I16Param {
    id: 0xc101,
    name: "V",
    label: MultilingualStr {
        de: "Volumenstrom (V)",
        en: "Volume flow (V)",
    },
    read: true,
    write: false,
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
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const REFRIGERANT_TEMP: DecParam = DecParam {
    id: 0xc103,
    name: "REFRIGERANT_TEMP",
    label: MultilingualStr {
        de: "Kältemitteltemperatur (t-liq)",
        en: "Refrigerant temperature (t-liq)",
    },
    read: true,
    write: false,
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
        de: "Temperatur Heizung Rücklauf (t-R)",
        en: "Heating return flow temperature (t-R)",
    },
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const OUTDOOR_TEMP_OPT: DecParam = DecParam {
    id: 0xc105,
    name: "OUTDOOR_TEMP_OPT",
    label: MultilingualStr {
        de: "Aussentemp. (opt.)",
        en: "Outdoor temp. (opt.)",
    },
    read: true,
    write: false,
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
        de: "Temperatur im Warmwasserspeicher (t-DHW)",
        en: "Temperature in the hot water storage tank (t-DHW)",
    },
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const PWM_PUMP: I16Param = I16Param {
    id: 0xc10c,
    name: "PWM_PUMP",
    label: MultilingualStr {
        de: "PWM Pumpe",
        en: "PWM pump",
    },
    read: true,
    write: false,
    unit: Some(Unit::Percent),
    default: None,
    min: Some(25),
    max: Some(100),
};
pub const T_AU: DecParam = DecParam {
    id: 0xc176,
    name: "T_AU",
    label: MultilingualStr {
        de: "Außentemperaturfühler (t-AU)",
        en: "Outside temperature sensor (t-AU)",
    },
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const STATUS_HEATING_SUPPORT: BoolParam = BoolParam {
    id: 0xc179,
    name: "STATUS_HEATING_SUPPORT",
    label: MultilingualStr {
        de: "Status Heitzungsunterstützung",
        en: "Status, heating support",
    },
    read: true,
    write: false,
    default: None,
};
pub const T_TVBH: DecParam = DecParam {
    id: 0xc1bf,
    name: "T_TVBH",
    label: MultilingualStr {
        de: "TVBH2",
        en: "TVBH2",
    },
    read: true,
    write: false,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const SWITCH_TEMP_HC: DecParam = DecParam {
    id: 0xc1c3,
    name: "SWITCH_TEMP_HC",
    label: MultilingualStr {
        de: "Umschalttemperatur Heizkreis",
        en: "Switch temp. heating circuit",
    },
    read: true,
    write: true,
    unit: Some(Unit::DegCelsius),
    scale: 1,
    default: Some(dec!(0.0)),
    min: Some(dec!(10.0)),
    max: Some(dec!(40.0)),
};
pub const ENERGY_ELECTRICAL_01_LOW: I16Param = I16Param {
    id: 0xc26b,
    name: "ENERGY_ELECTRICAL_01_LOW",
    label: MultilingualStr {
        de: "Elektrische Energie 01 [Low]",
        en: "Electrical Energy 01 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_01_HIGH: I16Param = I16Param {
    id: 0xc26c,
    name: "ENERGY_ELECTRICAL_01_HIGH",
    label: MultilingualStr {
        de: "Elektrische Energie 01 [High]",
        en: "Electrical Energy 01 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_02_LOW: I16Param = I16Param {
    id: 0xc26d,
    name: "ENERGY_ELECTRICAL_02_LOW",
    label: MultilingualStr {
        de: "Elektrische Energie 02 [Low]",
        en: "Electrical Energy 02 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_02_HIGH: I16Param = I16Param {
    id: 0xc26e,
    name: "ENERGY_ELECTRICAL_02_HIGH",
    label: MultilingualStr {
        de: "Elektrische Energie 02 [High]",
        en: "Electrical Energy 02 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_03_LOW: I16Param = I16Param {
    id: 0xc26f,
    name: "ENERGY_ELECTRICAL_03_LOW",
    label: MultilingualStr {
        de: "Elektrische Energie 03 [Low]",
        en: "Electrical Energy 03 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_03_HIGH: I16Param = I16Param {
    id: 0xc270,
    name: "ENERGY_ELECTRICAL_03_HIGH",
    label: MultilingualStr {
        de: "Elektrische Energie 03 [High]",
        en: "Electrical Energy 03 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_04_LOW: I16Param = I16Param {
    id: 0xc271,
    name: "ENERGY_ELECTRICAL_04_LOW",
    label: MultilingualStr {
        de: "Elektrische Energie 04 [Low]",
        en: "Electrical Energy 04 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_04_HIGH: I16Param = I16Param {
    id: 0xc272,
    name: "ENERGY_ELECTRICAL_04_HIGH",
    label: MultilingualStr {
        de: "Elektrische Energie 04 [High]",
        en: "Electrical Energy 04 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_05_LOW: I16Param = I16Param {
    id: 0xc273,
    name: "ENERGY_ELECTRICAL_05_LOW",
    label: MultilingualStr {
        de: "Elektrische Energie 05 [Low]",
        en: "Electrical Energy 05 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_05_HIGH: I16Param = I16Param {
    id: 0xc274,
    name: "ENERGY_ELECTRICAL_05_HIGH",
    label: MultilingualStr {
        de: "Elektrische Energie 05 [High]",
        en: "Electrical Energy 05 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_06_LOW: I16Param = I16Param {
    id: 0xc275,
    name: "ENERGY_ELECTRICAL_06_LOW",
    label: MultilingualStr {
        de: "Elektrische Energie 06 [Low]",
        en: "Electrical Energy 06 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_06_HIGH: I16Param = I16Param {
    id: 0xc276,
    name: "ENERGY_ELECTRICAL_06_HIGH",
    label: MultilingualStr {
        de: "Elektrische Energie 06 [High]",
        en: "Electrical Energy 06 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_07_LOW: I16Param = I16Param {
    id: 0xc277,
    name: "ENERGY_ELECTRICAL_07_LOW",
    label: MultilingualStr {
        de: "Elektrische Energie 07 [Low]",
        en: "Electrical Energy 07 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_07_HIGH: I16Param = I16Param {
    id: 0xc278,
    name: "ENERGY_ELECTRICAL_07_HIGH",
    label: MultilingualStr {
        de: "Elektrische Energie 07 [High]",
        en: "Electrical Energy 07 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_08_LOW: I16Param = I16Param {
    id: 0xc279,
    name: "ENERGY_ELECTRICAL_08_LOW",
    label: MultilingualStr {
        de: "Elektrische Energie 08 [Low]",
        en: "Electrical Energy 08 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_08_HIGH: I16Param = I16Param {
    id: 0xc27a,
    name: "ENERGY_ELECTRICAL_08_HIGH",
    label: MultilingualStr {
        de: "Elektrische Energie 08 [High]",
        en: "Electrical Energy 08 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_09_LOW: I16Param = I16Param {
    id: 0xc27b,
    name: "ENERGY_ELECTRICAL_09_LOW",
    label: MultilingualStr {
        de: "Elektrische Energie 09 [Low]",
        en: "Electrical Energy 09 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_09_HIGH: I16Param = I16Param {
    id: 0xc27c,
    name: "ENERGY_ELECTRICAL_09_HIGH",
    label: MultilingualStr {
        de: "Elektrische Energie 09 [High]",
        en: "Electrical Energy 09 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_10_LOW: I16Param = I16Param {
    id: 0xc27d,
    name: "ENERGY_ELECTRICAL_10_LOW",
    label: MultilingualStr {
        de: "Elektrische Energie 10 [Low]",
        en: "Electrical Energy 10 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_10_HIGH: I16Param = I16Param {
    id: 0xc27e,
    name: "ENERGY_ELECTRICAL_10_HIGH",
    label: MultilingualStr {
        de: "Elektrische Energie 10 [High]",
        en: "Electrical Energy 10 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_11_LOW: I16Param = I16Param {
    id: 0xc27f,
    name: "ENERGY_ELECTRICAL_11_LOW",
    label: MultilingualStr {
        de: "Elektrische Energie 11 [Low]",
        en: "Electrical Energy 11 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_11_HIGH: I16Param = I16Param {
    id: 0xc280,
    name: "ENERGY_ELECTRICAL_11_HIGH",
    label: MultilingualStr {
        de: "Elektrische Energie 11 [High]",
        en: "Electrical Energy 11 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_12_LOW: I16Param = I16Param {
    id: 0xc281,
    name: "ENERGY_ELECTRICAL_12_LOW",
    label: MultilingualStr {
        de: "Elektrische Energie 12 [Low]",
        en: "Electrical Energy 12 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_ELECTRICAL_12_HIGH: I16Param = I16Param {
    id: 0xc282,
    name: "ENERGY_ELECTRICAL_12_HIGH",
    label: MultilingualStr {
        de: "Elektrische Energie 12 [High]",
        en: "Electrical Energy 12 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_01_LOW: I16Param = I16Param {
    id: 0xc283,
    name: "ENERGY_HP_COOLING_01_LOW",
    label: MultilingualStr {
        de: "Energie WP Kühlung 01 [Low]",
        en: "Energy HP cooling 01 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_01_HIGH: I16Param = I16Param {
    id: 0xc284,
    name: "ENERGY_HP_COOLING_01_HIGH",
    label: MultilingualStr {
        de: "Energie WP Kühlung 01 [High]",
        en: "Energy HP cooling 01 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_02_LOW: I16Param = I16Param {
    id: 0xc285,
    name: "ENERGY_HP_COOLING_02_LOW",
    label: MultilingualStr {
        de: "Energie WP Kühlung 02 [Low]",
        en: "Energy HP cooling 02 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_02_HIGH: I16Param = I16Param {
    id: 0xc286,
    name: "ENERGY_HP_COOLING_02_HIGH",
    label: MultilingualStr {
        de: "Energie WP Kühlung 02 [High]",
        en: "Energy HP cooling 02 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_03_LOW: I16Param = I16Param {
    id: 0xc287,
    name: "ENERGY_HP_COOLING_03_LOW",
    label: MultilingualStr {
        de: "Energie WP Kühlung 03 [Low]",
        en: "Energy HP cooling 03 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_03_HIGH: I16Param = I16Param {
    id: 0xc288,
    name: "ENERGY_HP_COOLING_03_HIGH",
    label: MultilingualStr {
        de: "Energie WP Kühlung 03 [High]",
        en: "Energy HP cooling 03 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_04_LOW: I16Param = I16Param {
    id: 0xc289,
    name: "ENERGY_HP_COOLING_04_LOW",
    label: MultilingualStr {
        de: "Energie WP Kühlung 04 [Low]",
        en: "Energy HP cooling 04 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_04_HIGH: I16Param = I16Param {
    id: 0xc28a,
    name: "ENERGY_HP_COOLING_04_HIGH",
    label: MultilingualStr {
        de: "Energie WP Kühlung 04 [High]",
        en: "Energy HP cooling 04 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_05_LOW: I16Param = I16Param {
    id: 0xc28b,
    name: "ENERGY_HP_COOLING_05_LOW",
    label: MultilingualStr {
        de: "Energie WP Kühlung 05 [Low]",
        en: "Energy HP cooling 05 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_05_HIGH: I16Param = I16Param {
    id: 0xc28c,
    name: "ENERGY_HP_COOLING_05_HIGH",
    label: MultilingualStr {
        de: "Energie WP Kühlung 05 [High]",
        en: "Energy HP cooling 05 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_06_LOW: I16Param = I16Param {
    id: 0xc28d,
    name: "ENERGY_HP_COOLING_06_LOW",
    label: MultilingualStr {
        de: "Energie WP Kühlung 06 [Low]",
        en: "Energy HP cooling 06 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_06_HIGH: I16Param = I16Param {
    id: 0xc28e,
    name: "ENERGY_HP_COOLING_06_HIGH",
    label: MultilingualStr {
        de: "Energie WP Kühlung 06 [High]",
        en: "Energy HP cooling 06 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_07_LOW: I16Param = I16Param {
    id: 0xc28f,
    name: "ENERGY_HP_COOLING_07_LOW",
    label: MultilingualStr {
        de: "Energie WP Kühlung 07 [Low]",
        en: "Energy HP cooling 07 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_07_HIGH: I16Param = I16Param {
    id: 0xc290,
    name: "ENERGY_HP_COOLING_07_HIGH",
    label: MultilingualStr {
        de: "Energie WP Kühlung 07 [High]",
        en: "Energy HP cooling 07 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_08_LOW: I16Param = I16Param {
    id: 0xc291,
    name: "ENERGY_HP_COOLING_08_LOW",
    label: MultilingualStr {
        de: "Energie WP Kühlung 08 [Low]",
        en: "Energy HP cooling 08 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_08_HIGH: I16Param = I16Param {
    id: 0xc292,
    name: "ENERGY_HP_COOLING_08_HIGH",
    label: MultilingualStr {
        de: "Energie WP Kühlung 08 [High]",
        en: "Energy HP cooling 08 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_09_LOW: I16Param = I16Param {
    id: 0xc293,
    name: "ENERGY_HP_COOLING_09_LOW",
    label: MultilingualStr {
        de: "Energie WP Kühlung 09 [Low]",
        en: "Energy HP cooling 09 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_09_HIGH: I16Param = I16Param {
    id: 0xc294,
    name: "ENERGY_HP_COOLING_09_HIGH",
    label: MultilingualStr {
        de: "Energie WP Kühlung 09 [High]",
        en: "Energy HP cooling 09 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_10_LOW: I16Param = I16Param {
    id: 0xc295,
    name: "ENERGY_HP_COOLING_10_LOW",
    label: MultilingualStr {
        de: "Energie WP Kühlung 10 [Low]",
        en: "Energy HP cooling 10 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_10_HIGH: I16Param = I16Param {
    id: 0xc296,
    name: "ENERGY_HP_COOLING_10_HIGH",
    label: MultilingualStr {
        de: "Energie WP Kühlung 10 [High]",
        en: "Energy HP cooling 10 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_11_LOW: I16Param = I16Param {
    id: 0xc297,
    name: "ENERGY_HP_COOLING_11_LOW",
    label: MultilingualStr {
        de: "Energie WP Kühlung 11 [Low]",
        en: "Energy HP cooling 11 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_11_HIGH: I16Param = I16Param {
    id: 0xc298,
    name: "ENERGY_HP_COOLING_11_HIGH",
    label: MultilingualStr {
        de: "Energie WP Kühlung 11 [High]",
        en: "Energy HP cooling 11 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_12_LOW: I16Param = I16Param {
    id: 0xc299,
    name: "ENERGY_HP_COOLING_12_LOW",
    label: MultilingualStr {
        de: "Energie WP Kühlung 12 [Low]",
        en: "Energy HP cooling 12 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_COOLING_12_HIGH: I16Param = I16Param {
    id: 0xc29a,
    name: "ENERGY_HP_COOLING_12_HIGH",
    label: MultilingualStr {
        de: "Energie WP Kühlung 12 [High]",
        en: "Energy HP cooling 12 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_01_LOW: I16Param = I16Param {
    id: 0xc29b,
    name: "ENERGY_HP_HEATING_01_LOW",
    label: MultilingualStr {
        de: "Energie WP Heizung 01 [Low]",
        en: "Energy HP heating 01 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_01_HIGH: I16Param = I16Param {
    id: 0xc29c,
    name: "ENERGY_HP_HEATING_01_HIGH",
    label: MultilingualStr {
        de: "Energie WP Heizung 01 [High]",
        en: "Energy HP heating 01 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_02_LOW: I16Param = I16Param {
    id: 0xc29d,
    name: "ENERGY_HP_HEATING_02_LOW",
    label: MultilingualStr {
        de: "Energie WP Heizung 02 [Low]",
        en: "Energy HP heating 02 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_02_HIGH: I16Param = I16Param {
    id: 0xc29e,
    name: "ENERGY_HP_HEATING_02_HIGH",
    label: MultilingualStr {
        de: "Energie WP Heizung 02 [High]",
        en: "Energy HP heating 02 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_03_LOW: I16Param = I16Param {
    id: 0xc29f,
    name: "ENERGY_HP_HEATING_03_LOW",
    label: MultilingualStr {
        de: "Energie WP Heizung 03 [Low]",
        en: "Energy HP heating 03 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_03_HIGH: I16Param = I16Param {
    id: 0xc2a0,
    name: "ENERGY_HP_HEATING_03_HIGH",
    label: MultilingualStr {
        de: "Energie WP Heizung 03 [High]",
        en: "Energy HP heating 03 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_04_LOW: I16Param = I16Param {
    id: 0xc2a1,
    name: "ENERGY_HP_HEATING_04_LOW",
    label: MultilingualStr {
        de: "Energie WP Heizung 04 [Low]",
        en: "Energy HP heating 04 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_04_HIGH: I16Param = I16Param {
    id: 0xc2a2,
    name: "ENERGY_HP_HEATING_04_HIGH",
    label: MultilingualStr {
        de: "Energie WP Heizung 04 [High]",
        en: "Energy HP heating 04 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_05_LOW: I16Param = I16Param {
    id: 0xc2a3,
    name: "ENERGY_HP_HEATING_05_LOW",
    label: MultilingualStr {
        de: "Energie WP Heizung 05 [Low]",
        en: "Energy HP heating 05 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_05_HIGH: I16Param = I16Param {
    id: 0xc2a4,
    name: "ENERGY_HP_HEATING_05_HIGH",
    label: MultilingualStr {
        de: "Energie WP Heizung 05 [High]",
        en: "Energy HP heating 05 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_06_LOW: I16Param = I16Param {
    id: 0xc2a5,
    name: "ENERGY_HP_HEATING_06_LOW",
    label: MultilingualStr {
        de: "Energie WP Heizung 06 [Low]",
        en: "Energy HP heating 06 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_06_HIGH: I16Param = I16Param {
    id: 0xc2a6,
    name: "ENERGY_HP_HEATING_06_HIGH",
    label: MultilingualStr {
        de: "Energie WP Heizung 06 [High]",
        en: "Energy HP heating 06 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_07_LOW: I16Param = I16Param {
    id: 0xc2a7,
    name: "ENERGY_HP_HEATING_07_LOW",
    label: MultilingualStr {
        de: "Energie WP Heizung 07 [Low]",
        en: "Energy HP heating 07 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_07_HIGH: I16Param = I16Param {
    id: 0xc2a8,
    name: "ENERGY_HP_HEATING_07_HIGH",
    label: MultilingualStr {
        de: "Energie WP Heizung 07 [High]",
        en: "Energy HP heating 07 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_08_LOW: I16Param = I16Param {
    id: 0xc2a9,
    name: "ENERGY_HP_HEATING_08_LOW",
    label: MultilingualStr {
        de: "Energie WP Heizung 08 [Low]",
        en: "Energy HP heating 08 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_08_HIGH: I16Param = I16Param {
    id: 0xc2aa,
    name: "ENERGY_HP_HEATING_08_HIGH",
    label: MultilingualStr {
        de: "Energie WP Heizung 08 [High]",
        en: "Energy HP heating 08 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_09_LOW: I16Param = I16Param {
    id: 0xc2ab,
    name: "ENERGY_HP_HEATING_09_LOW",
    label: MultilingualStr {
        de: "Energie WP Heizung 09 [Low]",
        en: "Energy HP heating 09 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_19_HIGH: I16Param = I16Param {
    id: 0xc2ac,
    name: "ENERGY_HP_HEATING_19_HIGH",
    label: MultilingualStr {
        de: "Energie WP Heizung 09 [High]",
        en: "Energy HP heating 09 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_10_LOW: I16Param = I16Param {
    id: 0xc2ad,
    name: "ENERGY_HP_HEATING_10_LOW",
    label: MultilingualStr {
        de: "Energie WP Heizung 10 [Low]",
        en: "Energy HP heating 10 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_10_HIGH: I16Param = I16Param {
    id: 0xc2ae,
    name: "ENERGY_HP_HEATING_10_HIGH",
    label: MultilingualStr {
        de: "Energie WP Heizung 10 [High]",
        en: "Energy HP heating 10 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_11_LOW: I16Param = I16Param {
    id: 0xc2af,
    name: "ENERGY_HP_HEATING_11_LOW",
    label: MultilingualStr {
        de: "Energie WP Heizung 11 [Low]",
        en: "Energy HP heating 11 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_11_HIGH: I16Param = I16Param {
    id: 0xc2b0,
    name: "ENERGY_HP_HEATING_11_HIGH",
    label: MultilingualStr {
        de: "Energie WP Heizung 11 [High]",
        en: "Energy HP heating 11 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_12_LOW: I16Param = I16Param {
    id: 0xc2b1,
    name: "ENERGY_HP_HEATING_12_LOW",
    label: MultilingualStr {
        de: "Energie WP Heizung 12 [Low]",
        en: "Energy HP heating 12 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_HEATING_12_HIGH: I16Param = I16Param {
    id: 0xc2b2,
    name: "ENERGY_HP_HEATING_12_HIGH",
    label: MultilingualStr {
        de: "Energie WP Heizung 12 [High]",
        en: "Energy HP heating 12 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_01_LOW: I16Param = I16Param {
    id: 0xc2b3,
    name: "ENERGY_HOT_WATER_01_LOW",
    label: MultilingualStr {
        de: "Energie Warmwasser 01 [Low]",
        en: "Energy hot water 01 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_01_HIGH: I16Param = I16Param {
    id: 0xc2b4,
    name: "ENERGY_HOT_WATER_01_HIGH",
    label: MultilingualStr {
        de: "Energie Warmwasser 01 [High]",
        en: "Energy hot water 01 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_02_LOW: I16Param = I16Param {
    id: 0xc2b5,
    name: "ENERGY_HOT_WATER_02_LOW",
    label: MultilingualStr {
        de: "Energie Warmwasser 02 [Low]",
        en: "Energy hot water 02 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_02_HIGH: I16Param = I16Param {
    id: 0xc2b6,
    name: "ENERGY_HOT_WATER_02_HIGH",
    label: MultilingualStr {
        de: "Energie Warmwasser 02 [High]",
        en: "Energy hot water 02 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_03_LOW: I16Param = I16Param {
    id: 0xc2b7,
    name: "ENERGY_HOT_WATER_03_LOW",
    label: MultilingualStr {
        de: "Energie Warmwasser 03 [Low]",
        en: "Energy hot water 03 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_03_HIGH: I16Param = I16Param {
    id: 0xc2b8,
    name: "ENERGY_HOT_WATER_03_HIGH",
    label: MultilingualStr {
        de: "Energie Warmwasser 03 [High]",
        en: "Energy hot water 03 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_04_LOW: I16Param = I16Param {
    id: 0xc2b9,
    name: "ENERGY_HOT_WATER_04_LOW",
    label: MultilingualStr {
        de: "Energie Warmwasser 04 [Low]",
        en: "Energy hot water 04 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_04_HIGH: I16Param = I16Param {
    id: 0xc2ba,
    name: "ENERGY_HOT_WATER_04_HIGH",
    label: MultilingualStr {
        de: "Energie Warmwasser 04 [High]",
        en: "Energy hot water 04 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_05_LOW: I16Param = I16Param {
    id: 0xc2bb,
    name: "ENERGY_HOT_WATER_05_LOW",
    label: MultilingualStr {
        de: "Energie Warmwasser 05 [Low]",
        en: "Energy hot water 05 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_05_HIGH: I16Param = I16Param {
    id: 0xc2bc,
    name: "ENERGY_HOT_WATER_05_HIGH",
    label: MultilingualStr {
        de: "Energie Warmwasser 05 [High]",
        en: "Energy hot water 05 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_06_LOW: I16Param = I16Param {
    id: 0xc2bd,
    name: "ENERGY_HOT_WATER_06_LOW",
    label: MultilingualStr {
        de: "Energie Warmwasser 06 [Low]",
        en: "Energy hot water 06 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_06_HIGH: I16Param = I16Param {
    id: 0xc2be,
    name: "ENERGY_HOT_WATER_06_HIGH",
    label: MultilingualStr {
        de: "Energie Warmwasser 06 [High]",
        en: "Energy hot water 06 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_07_LOW: I16Param = I16Param {
    id: 0xc2bf,
    name: "ENERGY_HOT_WATER_07_LOW",
    label: MultilingualStr {
        de: "Energie Warmwasser 07 [Low]",
        en: "Energy hot water 07 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_07_HIGH: I16Param = I16Param {
    id: 0xc2c0,
    name: "ENERGY_HOT_WATER_07_HIGH",
    label: MultilingualStr {
        de: "Energie Warmwasser 07 [High]",
        en: "Energy hot water 07 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_08_LOW: I16Param = I16Param {
    id: 0xc2c1,
    name: "ENERGY_HOT_WATER_08_LOW",
    label: MultilingualStr {
        de: "Energie Warmwasser 08 [Low]",
        en: "Energy hot water 08 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_08_HIGH: I16Param = I16Param {
    id: 0xc2c2,
    name: "ENERGY_HOT_WATER_08_HIGH",
    label: MultilingualStr {
        de: "Energie Warmwasser 08 [High]",
        en: "Energy hot water 08 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_09_LOW: I16Param = I16Param {
    id: 0xc2c3,
    name: "ENERGY_HOT_WATER_09_LOW",
    label: MultilingualStr {
        de: "Energie Warmwasser 09 [Low]",
        en: "Energy hot water 09 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_09_HIGH: I16Param = I16Param {
    id: 0xc2c4,
    name: "ENERGY_HOT_WATER_09_HIGH",
    label: MultilingualStr {
        de: "Energie Warmwasser 09 [High]",
        en: "Energy hot water 09 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_10_LOW: I16Param = I16Param {
    id: 0xc2c5,
    name: "ENERGY_HOT_WATER_10_LOW",
    label: MultilingualStr {
        de: "Energie Warmwasser 10 [Low]",
        en: "Energy hot water 10 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_10_HIGH: I16Param = I16Param {
    id: 0xc2c6,
    name: "ENERGY_HOT_WATER_10_HIGH",
    label: MultilingualStr {
        de: "Energie Warmwasser 10 [High]",
        en: "Energy hot water 10 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_11_LOW: I16Param = I16Param {
    id: 0xc2c7,
    name: "ENERGY_HOT_WATER_11_LOW",
    label: MultilingualStr {
        de: "Energie Warmwasser 11 [Low]",
        en: "Energy hot water 11 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_11_HIGH: I16Param = I16Param {
    id: 0xc2c8,
    name: "ENERGY_HOT_WATER_11_HIGH",
    label: MultilingualStr {
        de: "Energie Warmwasser 11 [High]",
        en: "Energy hot water 11 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_12_LOW: I16Param = I16Param {
    id: 0xc2c9,
    name: "ENERGY_HOT_WATER_12_LOW",
    label: MultilingualStr {
        de: "Energie Warmwasser 12 [Low]",
        en: "Energy hot water 12 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HOT_WATER_12_HIGH: I16Param = I16Param {
    id: 0xc2ca,
    name: "ENERGY_HOT_WATER_12_HIGH",
    label: MultilingualStr {
        de: "Energie Warmwasser 12 [High]",
        en: "Energy hot water 12 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_01_LOW: I16Param = I16Param {
    id: 0xc2cb,
    name: "ENERGY_HP_TOTAL_01_LOW",
    label: MultilingualStr {
        de: "Energie WP gesamt 01 [Low]",
        en: "Energy HP total 01 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_01_HIGH: I16Param = I16Param {
    id: 0xc2cc,
    name: "ENERGY_HP_TOTAL_01_HIGH",
    label: MultilingualStr {
        de: "Energie WP gesamt 01 [High]",
        en: "Energy HP total 01 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_02_LOW: I16Param = I16Param {
    id: 0xc2cd,
    name: "ENERGY_HP_TOTAL_02_LOW",
    label: MultilingualStr {
        de: "Energie WP gesamt 02 [Low]",
        en: "Energy HP total 02 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_02_HIGH: I16Param = I16Param {
    id: 0xc2ce,
    name: "ENERGY_HP_TOTAL_02_HIGH",
    label: MultilingualStr {
        de: "Energie WP gesamt 02 [High]",
        en: "Energy HP total 02 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_03_LOW: I16Param = I16Param {
    id: 0xc2cf,
    name: "ENERGY_HP_TOTAL_03_LOW",
    label: MultilingualStr {
        de: "Energie WP gesamt 03 [Low]",
        en: "Energy HP total 03 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_03_HIGH: I16Param = I16Param {
    id: 0xc2d0,
    name: "ENERGY_HP_TOTAL_03_HIGH",
    label: MultilingualStr {
        de: "Energie WP gesamt 03 [High]",
        en: "Energy HP total 03 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_04_LOW: I16Param = I16Param {
    id: 0xc2d1,
    name: "ENERGY_HP_TOTAL_04_LOW",
    label: MultilingualStr {
        de: "Energie WP gesamt 04 [Low]",
        en: "Energy HP total 04 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_04_HIGH: I16Param = I16Param {
    id: 0xc2d2,
    name: "ENERGY_HP_TOTAL_04_HIGH",
    label: MultilingualStr {
        de: "Energie WP gesamt 04 [High]",
        en: "Energy HP total 04 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_05_LOW: I16Param = I16Param {
    id: 0xc2d3,
    name: "ENERGY_HP_TOTAL_05_LOW",
    label: MultilingualStr {
        de: "Energie WP gesamt 05 [Low]",
        en: "Energy HP total 05 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_05_HIGH: I16Param = I16Param {
    id: 0xc2d4,
    name: "ENERGY_HP_TOTAL_05_HIGH",
    label: MultilingualStr {
        de: "Energie WP gesamt 05 [High]",
        en: "Energy HP total 05 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_06_LOW: I16Param = I16Param {
    id: 0xc2d5,
    name: "ENERGY_HP_TOTAL_06_LOW",
    label: MultilingualStr {
        de: "Energie WP gesamt 06 [Low]",
        en: "Energy HP total 06 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_06_HIGH: I16Param = I16Param {
    id: 0xc2d6,
    name: "ENERGY_HP_TOTAL_06_HIGH",
    label: MultilingualStr {
        de: "Energie WP gesamt 06 [High]",
        en: "Energy HP total 06 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_07_LOW: I16Param = I16Param {
    id: 0xc2d7,
    name: "ENERGY_HP_TOTAL_07_LOW",
    label: MultilingualStr {
        de: "Energie WP gesamt 07 [Low]",
        en: "Energy HP total 07 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_07_HIGH: I16Param = I16Param {
    id: 0xc2d8,
    name: "ENERGY_HP_TOTAL_07_HIGH",
    label: MultilingualStr {
        de: "Energie WP gesamt 07 [High]",
        en: "Energy HP total 07 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_08_LOW: I16Param = I16Param {
    id: 0xc2d9,
    name: "ENERGY_HP_TOTAL_08_LOW",
    label: MultilingualStr {
        de: "Energie WP gesamt 08 [Low]",
        en: "Energy HP total 08 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_08_HIGH: I16Param = I16Param {
    id: 0xc2da,
    name: "ENERGY_HP_TOTAL_08_HIGH",
    label: MultilingualStr {
        de: "Energie WP gesamt 08 [High]",
        en: "Energy HP total 08 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_09_LOW: I16Param = I16Param {
    id: 0xc2db,
    name: "ENERGY_HP_TOTAL_09_LOW",
    label: MultilingualStr {
        de: "Energie WP gesamt 09 [Low]",
        en: "Energy HP total 09 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_09_HIGH: I16Param = I16Param {
    id: 0xc2dc,
    name: "ENERGY_HP_TOTAL_09_HIGH",
    label: MultilingualStr {
        de: "Energie WP gesamt 09 [High]",
        en: "Energy HP total 09 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_10_LOW: I16Param = I16Param {
    id: 0xc2dd,
    name: "ENERGY_HP_TOTAL_10_LOW",
    label: MultilingualStr {
        de: "Energie WP gesamt 10 [Low]",
        en: "Energy HP total 10 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_10_HIGH: I16Param = I16Param {
    id: 0xc2de,
    name: "ENERGY_HP_TOTAL_10_HIGH",
    label: MultilingualStr {
        de: "Energie WP gesamt 10 [High]",
        en: "Energy HP total 10 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_11_LOW: I16Param = I16Param {
    id: 0xc2df,
    name: "ENERGY_HP_TOTAL_11_LOW",
    label: MultilingualStr {
        de: "Energie WP gesamt 11 [Low]",
        en: "Energy HP total 11 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_11_HIGH: I16Param = I16Param {
    id: 0xc2e0,
    name: "ENERGY_HP_TOTAL_11_HIGH",
    label: MultilingualStr {
        de: "Energie WP gesamt 11 [High]",
        en: "Energy HP total 11 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_12_LOW: I16Param = I16Param {
    id: 0xc2e1,
    name: "ENERGY_HP_TOTAL_12_LOW",
    label: MultilingualStr {
        de: "Energie WP gesamt 12 [Low]",
        en: "Energy HP total 12 [Low]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const ENERGY_HP_TOTAL_12_HIGH: I16Param = I16Param {
    id: 0xc2e2,
    name: "ENERGY_HP_TOTAL_12_HIGH",
    label: MultilingualStr {
        de: "Energie WP gesamt 12 [High]",
        en: "Energy HP total 12 [High]",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const SOLAR_FUNCTION: BoolParam = BoolParam {
    id: 0xc2f6,
    name: "SOLAR_FUNCTION",
    label: MultilingualStr {
        de: "Solar Funktion",
        en: "Solar function",
    },
    read: true,
    write: true,
    default: None,
};
pub const ENERGY_ELECTRICAL: I16Param = I16Param {
    id: 0xc2fa,
    name: "ENERGY_ELECTRICAL",
    label: MultilingualStr {
        de: "Elektr. Energie gesamt",
        en: "Electr. energy total",
    },
    read: true,
    write: false,
    unit: Some(Unit::KiloWattHours),
    default: None,
    min: None,
    max: None,
};
pub const OUTDOOR_UNIT: Enum16Param<enums::OutdoorUnit> = Enum16Param {
    id: 0xc34c,
    name: "OUTDOOR_UNIT",
    label: MultilingualStr {
        de: "Außengerät",
        en: "Outdoor Unit",
    },
    read: true,
    write: true,
    default: None,
};
pub const INDOOR_UNIT: Enum16Param<enums::IndoorUnit> = Enum16Param {
    id: 0xc34d,
    name: "INDOOR_UNIT",
    label: MultilingualStr {
        de: "Innengerät",
        en: "Indoor Unit",
    },
    read: true,
    write: true,
    default: None,
};
pub const G1_ANTILEG_START_ZEIT: DecParam = DecParam {
    id: 0xfd4f,
    name: "G1_ANTILEG_START_ZEIT",
    label: MultilingualStr {
        de: "Startzeit Antileginoellenprogramm",
        en: "Anti-Legionella time",
    },
    read: false,
    write: false,
    unit: None,
    scale: 1,
    default: None,
    min: None,
    max: None,
};
pub const STATUS_HEAT_CIRCULATION_PUMP: BoolParam = BoolParam {
    id: 0xfdac,
    name: "STATUS_HEAT_CIRCULATION_PUMP",
    label: MultilingualStr {
        de: "Status Heizungsumwälzpumpe",
        en: "Status, heat circulation pump",
    },
    read: true,
    write: false,
    default: None,
};
pub const ANTI_LEGIONELLA_TIME: TimeParam = TimeParam {
    id: 0xfdf4,
    name: "ANTI_LEGIONELLA_TIME",
    label: MultilingualStr {
        de: "Antilegionellen Startzeit",
        en: "Anti-legionella start time",
    },
    read: true,
    write: true,
    default: None,
};
pub static PARAMS: phf::Map<u16, &dyn Param> = phf::phf_map! {
    0x0002u16 => & FEED_TEMPERATURE_TARGET, 0x0003u16 => & HOT_WATER_TEMP_TARGET,
    0x0004u16 => & FEED_TEMP_HC_TARGET, 0x0005u16 => & ROOM_TEMP_TARGET_1, 0x0006u16 => &
    ROOM_TEMP_TARGET_2, 0x0007u16 => & ROOM_TEMP_TARGET_3, 0x0008u16 => &
    REDUCE_ROOM_TEMP, 0x000cu16 => & OUTSIDE_TEMP, 0x000du16 => &
    FEED_TEMPERATURE_CURRENT, 0x000eu16 => & HOT_WATER_TEMP_CURRENT, 0x000fu16 => &
    FEED_TEMP_HC_CURRENT, 0x0010u16 => & MYSTERY_0010, 0x0011u16 => & RAUMISTTEMP,
    0x0012u16 => & VERSTELLTE_RAUMSOLLTEMP, 0x0013u16 => & HOT_WATER_TEMP_TARGET_1,
    0x0016u16 => & RETURN_FLOW_TEMP, 0x001cu16 => & WATER_PRESSURE, 0x0028u16 => &
    MAX_FEED_TEMP, 0x0052u16 => & MIXER_PUMP_STATUS, 0x0056u16 => & MIXER_INFO_1,
    0x0057u16 => & MIXER_INFO_2, 0x0058u16 => & MIXER_INFO_3, 0x0059u16 => &
    MIXER_INFO_4, 0x005eu16 => & HOT_WATER_ACTIVE, 0x0061u16 => & STATUS_COMPRESSOR,
    0x0101u16 => & ANTI_LEGIONELLA_DAY, 0x0103u16 => & AUFHEIZOPTIMIERUNG, 0x010cu16 => &
    BUILDING_INSULATION, 0x010eu16 => & HEATING_CURVE, 0x010fu16 => &
    ROOM_SENSOR_ADAPTATION, 0x0110u16 => & MAX_AUFHEIZVORVERLEGUNG, 0x0111u16 => &
    SLOPE_OFFSET, 0x0112u16 => & OPERATING_MODE, 0x0115u16 => & HEATING_CURVE_ADAPTION,
    0x0116u16 => & HEAT_LIMIT_HEATING_MODE, 0x0117u16 => & HEAT_LIMIT_REDUCING_MODE,
    0x011au16 => & SCREED, 0x011bu16 => & VACATION_BEGIN_DAY, 0x011cu16 => &
    VACATION_BEGIN_MONTH, 0x011du16 => & VACATION_BEGIN_YEAR, 0x011eu16 => &
    VACATION_END_DAY, 0x011fu16 => & VACATION_END_MONTH, 0x0120u16 => &
    VACATION_END_YEAR, 0x0122u16 => & DATE_DAY, 0x0123u16 => & DATE_MONTH, 0x0124u16 => &
    DATE_YEAR, 0x0125u16 => & TIME_HOUR, 0x0126u16 => & TIME_MINUTE, 0x0129u16 => &
    FEED_TEMP_HEATING_MODE, 0x012au16 => & FEED_TEMP_REDUCING_MODE, 0x012bu16 => &
    MIN_FEED_TEMP, 0x012eu16 => & ABSENKOPTIMIERUNG, 0x012fu16 => & MAX_MIXER_VALVE_PUMP,
    0x0130u16 => & MIN_MIXER_VALVE_PUMP, 0x0131u16 => & MIXER_PUMP_PWM, 0x013du16 => &
    ROOM_TEMP_ABSENT, 0x013eu16 => & HOT_WATER_TEMP_TARGET_3, 0x0141u16 => &
    WEATHER_COMPENSATED, 0x0144u16 => & LOAD_HOT_WATER, 0x0148u16 => & GERAETE_KENNUNG,
    0x0180u16 => & MAX_HOT_WATER_LOADING_TIME, 0x0182u16 => & CIRCULATION_PUMP_CONTROL,
    0x0199u16 => & VERSION_MAJOR, 0x019au16 => & VERSION_MINOR, 0x01dau16 => &
    VOLUME_FLOW, 0x01ecu16 => & ABSENT, 0x024bu16 => & VERSION_PATCH, 0x03ddu16 => &
    FEED_TEMP_COOLING_MODE, 0x0489u16 => & PROGRAMMABLE_OUTPUT, 0x0587u16 => &
    ANTI_LEGIONELLA_TEMP, 0x065eu16 => & CIRCULATION_PUMP_INTERVAL, 0x0661u16 => &
    VMIN_A1, 0x0668u16 => & EXT_POWER_HOT_WATER, 0x0669u16 => & EXT_POWER_STAGE_1,
    0x066au16 => & EXT_POWER_STAGE_2, 0x066bu16 => & HEATING_SUPPORT_POWER, 0x066cu16 =>
    & HEATING_SUPPORT, 0x066du16 => & HEATING_SUPPORT_HYSTERESIS, 0x066eu16 => &
    HEATING_SUPPORT_MAX_TEMP, 0x066fu16 => & HT_NT_FUNCTION, 0x0670u16 => &
    HT_NT_CONTACT, 0x0671u16 => & AUX_SWITCHING_FUNCTION, 0x0672u16 => & AUX_WAIT_TIME,
    0x0673u16 => & AUX_SWITCHING_THRESHOLD_TDHW, 0x0678u16 => & ROOM_THERMOSTAT,
    0x0679u16 => & INTERLINK_FUNCTION, 0x067eu16 => & MIN_PUMP_POWER, 0x067fu16 => &
    MAX_PUMP_POWER, 0x0682u16 => & WP_MOD_HYST_DURCHFLUSS, 0x0683u16 => &
    WP_SPREIZUNG_HZ_BETRIEB, 0x0684u16 => & WP_SPREIZUNG_WW_BETRIEB, 0x0685u16 => &
    WP_VERZ_ZEIT_PUMPE, 0x0688u16 => & VMIN_WP, 0x068cu16 => & WP_START_MAX_TEMP,
    0x0691u16 => & HOT_WATER_HYSTERESIS, 0x0692u16 => & WAITING_TIME_EXT_HEAT_GENERATOR,
    0x0693u16 => & SMART_GRID, 0x0694u16 => & SMART_GRID_MODE, 0x0695u16 => &
    VENTILATION_FUNCTION, 0x0696u16 => & QUIET_MODE, 0x0699u16 => & WP_INNENGERAET,
    0x069au16 => & WP_AUSSENGERAET, 0x069bu16 => & MIX_3UVDHW, 0x06A6u16 => &
    ENERGY_HP_COOLING, 0x06A7u16 => & ENERGY_HP_HEATING, 0x06a0u16 => &
    INTERLINK_TEMP_INCREASE, 0x06a1u16 => & INTERLINK_TEMP_REDUCTION, 0x06a4u16 => &
    RUNTIME_PUMP, 0x06a5u16 => & RUNTIME_COMPRESSOR, 0x06d0u16 => & GLYCOL, 0x06d1u16 =>
    & HP_POWER_LIMITATION, 0x06d2u16 => & EXT_HEAT_SOURCE, 0x06d3u16 => &
    BIVALENCE_FUNCTION, 0x06d4u16 => & BIVALENCE_TEMP, 0x06dbu16 => & PUMP_DT_HEATING,
    0x06dcu16 => & PUMP_DT_COOLING, 0x06ddu16 => & HEATING_SYSTEM, 0x06e1u16 => &
    PUMP_LIMIT, 0x06e2u16 => & FEED_TEMP_OVERSHOOT, 0x06e3u16 => & CONTINUOUS_HEATING,
    0x06e5u16 => & COMFORT_HEATING, 0x06e6u16 => & FUNC_BURNER_BLOCKING_CONTACT,
    0x06e7u16 => & EMERGENCY, 0x0725u16 => & WATER_PRESSURE_TARGET, 0x0726u16 => &
    WATER_MAX_PRESSURE_LOSS, 0x0727u16 => & WATER_PRESSURE_MAX, 0x0728u16 => &
    WATER_PRESSURE_MIN, 0x091cu16 => & ENERGY_EXT_HOT_WATER, 0x0920u16 => &
    ENERGY_EXT_HEATING, 0x092cu16 => & ENERGY_HOT_WATER, 0x0930u16 => & ENERGY_HP_TOTAL,
    0x093cu16 => & MYSTERY_093C, 0x0961u16 => & WATER_SENSORS, 0x0a00u16 => &
    FROST_PROTECTION_TEMP, 0x0a06u16 => & HOT_WATER_TEMP_TARGET_2, 0x0a0cu16 => &
    AVERAGE_OUTSIDE_TEMP, 0x0a1fu16 => & ZEITMASTER, 0x0c1fu16 => &
    OUTSIDE_TEMP_CORRECTION, 0x1350u16 => & HOLIDAY_BEGIN_DAY, 0x1351u16 => &
    HOLIDAY_BEGIN_MONTH, 0x1352u16 => & HOLIDAY_BEGIN_YEAR, 0x1353u16 => &
    HOLIDAY_END_DAY, 0x1354u16 => & HOLIDAY_END_MONTH, 0x1355u16 => & HOLIDAY_END_YEAR,
    0x1357u16 => & TEMP_MANUAL_OPERATION, 0x1358u16 => & PARTY, 0x1359u16 => &
    COOLING_SETPOINT_CORRECTION, 0x135bu16 => & START_COOLING_OUTSIDE_TEMP, 0x135cu16 =>
    & MAX_COOLING_OUTSIDE_TEMP, 0x135du16 => & TARGET_FLOW_COOLING_START, 0x135eu16 => &
    TARGET_FLOW_COOLING_MAX, 0x1363u16 => & FEED_TEMP_LOWER_LIMIT, 0x1388u16 => &
    FEHLER_AKTUELL, 0x13b5u16 => & START_KUEHLEN_AUSSENTEMP_HZK0, 0x1400u16 => &
    HC_AUTO_1, 0x1410u16 => & HC_AUTO_1_MON_1, 0x1411u16 => & HC_AUTO_1_MON_2, 0x1412u16
    => & HC_AUTO_1_MON_3, 0x1420u16 => & HC_AUTO_1_TUE_1, 0x1421u16 => & HC_AUTO_1_TUE_2,
    0x1422u16 => & HC_AUTO_1_TUE_3, 0x1430u16 => & HC_AUTO_1_WED_1, 0x1431u16 => &
    HC_AUTO_1_WED_2, 0x1432u16 => & HC_AUTO_1_WED_3, 0x1440u16 => & HC_AUTO_1_THU_1,
    0x1441u16 => & HC_AUTO_1_THU_2, 0x1442u16 => & HC_AUTO_1_THU_3, 0x1450u16 => &
    HC_AUTO_1_FRI_1, 0x1451u16 => & HC_AUTO_1_FRI_2, 0x1452u16 => & HC_AUTO_1_FRI_3,
    0x1460u16 => & HC_AUTO_1_SAT_1, 0x1461u16 => & HC_AUTO_1_SAT_2, 0x1462u16 => &
    HC_AUTO_1_SAT_3, 0x1470u16 => & HC_AUTO_1_SUN_1, 0x1471u16 => & HC_AUTO_1_SUN_2,
    0x1472u16 => & HC_AUTO_1_SUN_3, 0x1480u16 => & HC_AUTO_1_MON_FRI_1, 0x1481u16 => &
    HC_AUTO_1_MON_FRI_2, 0x1482u16 => & HC_AUTO_1_MON_FRI_3, 0x1490u16 => &
    HC_AUTO_1_SAT_SUN_1, 0x1491u16 => & HC_AUTO_1_SAT_SUN_2, 0x1492u16 => &
    HC_AUTO_1_SAT_SUN_3, 0x14a0u16 => & HC_AUTO_1_MON_SUN_1, 0x14a1u16 => &
    HC_AUTO_1_MON_SUN_2, 0x14a2u16 => & HC_AUTO_1_MON_SUN_3, 0x14b0u16 => &
    HC_AUTO_1_MON_THU_1, 0x14b1u16 => & HC_AUTO_1_MON_THU_2, 0x14b2u16 => &
    HC_AUTO_1_MON_THU_3, 0x1500u16 => & HC_AUTO_2, 0x1510u16 => & HC_AUTO_2_MON_1,
    0x1511u16 => & HC_AUTO_2_MON_2, 0x1512u16 => & HC_AUTO_2_MON_3, 0x1520u16 => &
    HC_AUTO_2_TUE_1, 0x1521u16 => & HC_AUTO_2_TUE_2, 0x1522u16 => & HC_AUTO_2_TUE_3,
    0x1530u16 => & HC_AUTO_2_WED_1, 0x1531u16 => & HC_AUTO_2_WED_2, 0x1532u16 => &
    HC_AUTO_2_WED_3, 0x1540u16 => & HC_AUTO_2_THU_1, 0x1541u16 => & HC_AUTO_2_THU_2,
    0x1542u16 => & HC_AUTO_2_THU_3, 0x1550u16 => & HC_AUTO_2_FRI_1, 0x1551u16 => &
    HC_AUTO_2_FRI_2, 0x1552u16 => & HC_AUTO_2_FRI_3, 0x1560u16 => & HC_AUTO_2_SAT_1,
    0x1561u16 => & HC_AUTO_2_SAT_2, 0x1562u16 => & HC_AUTO_2_SAT_3, 0x1570u16 => &
    HC_AUTO_2_SUN_1, 0x1571u16 => & HC_AUTO_2_SUN_2, 0x1572u16 => & HC_AUTO_2_SUN_3,
    0x1580u16 => & HC_AUTO_2_MON_FRI_1, 0x1581u16 => & HC_AUTO_2_MON_FRI_2, 0x1582u16 =>
    & HC_AUTO_2_MON_FRI_3, 0x1590u16 => & HC_AUTO_2_SAT_SUN_1, 0x1591u16 => &
    HC_AUTO_2_SAT_SUN_2, 0x1592u16 => & HC_AUTO_2_SAT_SUN_3, 0x15a0u16 => &
    HC_AUTO_2_MON_SUN_1, 0x15a1u16 => & HC_AUTO_2_MON_SUN_2, 0x15a2u16 => &
    HC_AUTO_2_MON_SUN_3, 0x15b0u16 => & HC_AUTO_2_MON_THU_1, 0x15b1u16 => &
    HC_AUTO_2_MON_THU_2, 0x15b2u16 => & HC_AUTO_2_MON_THU_3, 0x1700u16 => & DHW_AUTO_1,
    0x1710u16 => & DHW_AUTO_1_MON_1, 0x1711u16 => & DHW_AUTO_1_MON_2, 0x1712u16 => &
    DHW_AUTO_1_MON_3, 0x1720u16 => & DHW_AUTO_1_TUE_1, 0x1721u16 => & DHW_AUTO_1_TUE_2,
    0x1722u16 => & DHW_AUTO_1_TUE_3, 0x1730u16 => & DHW_AUTO_1_WED_1, 0x1731u16 => &
    DHW_AUTO_1_WED_2, 0x1732u16 => & DHW_AUTO_1_WED_3, 0x1740u16 => & DHW_AUTO_1_THU_1,
    0x1741u16 => & DHW_AUTO_1_THU_2, 0x1742u16 => & DHW_AUTO_1_THU_3, 0x1750u16 => &
    DHW_AUTO_1_FRI_1, 0x1751u16 => & DHW_AUTO_1_FRI_2, 0x1752u16 => & DHW_AUTO_1_FRI_3,
    0x1760u16 => & DHW_AUTO_1_SAT_1, 0x1761u16 => & DHW_AUTO_1_SAT_2, 0x1762u16 => &
    DHW_AUTO_1_SAT_3, 0x1770u16 => & DHW_AUTO_1_SUN_1, 0x1771u16 => & DHW_AUTO_1_SUN_2,
    0x1772u16 => & DHW_AUTO_1_SUN_3, 0x1780u16 => & DHW_AUTO_1_MON_FRI_1, 0x1781u16 => &
    DHW_AUTO_1_MON_FRI_2, 0x1782u16 => & DHW_AUTO_1_MON_FRI_3, 0x1790u16 => &
    DHW_AUTO_1_SAT_SUN_1, 0x1791u16 => & DHW_AUTO_1_SAT_SUN_2, 0x1792u16 => &
    DHW_AUTO_1_SAT_SUN_3, 0x17a0u16 => & DHW_AUTO_1_MON_SUN_1, 0x17a1u16 => &
    DHW_AUTO_1_MON_SUN_2, 0x17a2u16 => & DHW_AUTO_1_MON_SUN_3, 0x17b0u16 => &
    DHW_AUTO_1_MON_TUE_1, 0x17b1u16 => & DHW_AUTO_1_MON_TUE_2, 0x17b2u16 => &
    DHW_AUTO_1_MON_TUE_3, 0x1800u16 => & DHW_AUTO_2, 0x1810u16 => & DHW_AUTO_2_MON_1,
    0x1811u16 => & DHW_AUTO_2_MON_2, 0x1812u16 => & DHW_AUTO_2_MON_3, 0x1820u16 => &
    DHW_AUTO_2_TUE_1, 0x1821u16 => & DHW_AUTO_2_TUE_2, 0x1822u16 => & DHW_AUTO_2_TUE_3,
    0x1830u16 => & DHW_AUTO_2_WED_1, 0x1831u16 => & DHW_AUTO_2_WED_2, 0x1832u16 => &
    DHW_AUTO_2_WED_3, 0x1840u16 => & DHW_AUTO_2_THU_1, 0x1841u16 => & DHW_AUTO_2_THU_2,
    0x1842u16 => & DHW_AUTO_2_THU_3, 0x1850u16 => & DHW_AUTO_2_FRI_1, 0x1851u16 => &
    DHW_AUTO_2_FRI_2, 0x1852u16 => & DHW_AUTO_2_FRI_3, 0x1860u16 => & DHW_AUTO_2_SAT_1,
    0x1861u16 => & DHW_AUTO_2_SAT_2, 0x1862u16 => & DHW_AUTO_2_SAT_3, 0x1870u16 => &
    DHW_AUTO_2_SUN_1, 0x1871u16 => & DHW_AUTO_2_SUN_2, 0x1872u16 => & DHW_AUTO_2_SUN_3,
    0x1880u16 => & DHW_AUTO_2_MON_FRI_1, 0x1881u16 => & DHW_AUTO_2_MON_FRI_2, 0x1882u16
    => & DHW_AUTO_2_MON_FRI_3, 0x1890u16 => & DHW_AUTO_2_SAT_SUN_1, 0x1891u16 => &
    DHW_AUTO_2_SAT_SUN_2, 0x1892u16 => & DHW_AUTO_2_SAT_SUN_3, 0x18a0u16 => &
    DHW_AUTO_2_MON_SUN_1, 0x18a1u16 => & DHW_AUTO_2_MON_SUN_2, 0x18a2u16 => &
    DHW_AUTO_2_MON_SUN_3, 0x18b0u16 => & DHW_AUTO_2_MON_TUE_1, 0x18b1u16 => &
    DHW_AUTO_2_MON_TUE_2, 0x18b2u16 => & DHW_AUTO_2_MON_TUE_3, 0x1a10u16 => &
    CIRCULATION_MON_1, 0x1a11u16 => & CIRCULATION_MON_2, 0x1a12u16 => &
    CIRCULATION_MON_3, 0x1a20u16 => & CIRCULATION_TUE_1, 0x1a21u16 => &
    CIRCULATION_TUE_2, 0x1a22u16 => & CIRCULATION_TUE_3, 0x1a30u16 => &
    CIRCULATION_WED_1, 0x1a31u16 => & CIRCULATION_WED_2, 0x1a32u16 => &
    CIRCULATION_WED_3, 0x1a40u16 => & CIRCULATION_THU_1, 0x1a41u16 => &
    CIRCULATION_THU_2, 0x1a42u16 => & CIRCULATION_THU_3, 0x1a50u16 => &
    CIRCULATION_FRI_1, 0x1a51u16 => & CIRCUATLION_FRI_2, 0x1a52u16 => &
    CIRCULATION_FRI_3, 0x1a60u16 => & CIRCULATION_SAT_1, 0x1a61u16 => &
    CIRCULATION_SAT_2, 0x1a62u16 => & CIRCULATION_SAT_3, 0x1a70u16 => &
    CIRCULATION_SUN_1, 0x1a71u16 => & CIRCULATION_SUN_2, 0x1a72u16 => &
    CIRCULATION_SUN_3, 0x4e3fu16 => & HOT_WATER_BLOCKING_TIME, 0xc09du16 => & VMIN_GCU,
    0xc0b1u16 => & SONDERFKT_SCHALTKONTAKT, 0xc0b2u16 => & WARTEZEIT_SONDERFKT, 0xc0b3u16
    => & SCHALTSCHWELLE_TDHW, 0xc0b4u16 => & MYSTERY_C0B4, 0xc0c4u16 => &
    OUTSIDE_TEMP_SENSOR, 0xc0f6u16 => & MODE, 0xc0f7u16 => & PWM_SIGNAL, 0xc0f8u16 => &
    EXTERNAL_REQUEST, 0xc0f9u16 => & BUH_CURRENT_OUTPUT, 0xc0fau16 => &
    ROOM_THERMOSTAT_INTERLINK, 0xc0fbu16 => & MIX_3UVB1, 0xc0fcu16 => & FEED_TEMP_PHX,
    0xc0feu16 => & FEED_TEMP_BUH, 0xc101u16 => & V, 0xc102u16 => & T_TVBH1, 0xc103u16 =>
    & REFRIGERANT_TEMP, 0xc104u16 => & T_R, 0xc105u16 => & OUTDOOR_TEMP_OPT, 0xc106u16 =>
    & T_DHW, 0xc10cu16 => & PWM_PUMP, 0xc176u16 => & T_AU, 0xc179u16 => &
    STATUS_HEATING_SUPPORT, 0xc1bfu16 => & T_TVBH, 0xc1c3u16 => & SWITCH_TEMP_HC,
    0xc26bu16 => & ENERGY_ELECTRICAL_01_LOW, 0xc26cu16 => & ENERGY_ELECTRICAL_01_HIGH,
    0xc26du16 => & ENERGY_ELECTRICAL_02_LOW, 0xc26eu16 => & ENERGY_ELECTRICAL_02_HIGH,
    0xc26fu16 => & ENERGY_ELECTRICAL_03_LOW, 0xc270u16 => & ENERGY_ELECTRICAL_03_HIGH,
    0xc271u16 => & ENERGY_ELECTRICAL_04_LOW, 0xc272u16 => & ENERGY_ELECTRICAL_04_HIGH,
    0xc273u16 => & ENERGY_ELECTRICAL_05_LOW, 0xc274u16 => & ENERGY_ELECTRICAL_05_HIGH,
    0xc275u16 => & ENERGY_ELECTRICAL_06_LOW, 0xc276u16 => & ENERGY_ELECTRICAL_06_HIGH,
    0xc277u16 => & ENERGY_ELECTRICAL_07_LOW, 0xc278u16 => & ENERGY_ELECTRICAL_07_HIGH,
    0xc279u16 => & ENERGY_ELECTRICAL_08_LOW, 0xc27au16 => & ENERGY_ELECTRICAL_08_HIGH,
    0xc27bu16 => & ENERGY_ELECTRICAL_09_LOW, 0xc27cu16 => & ENERGY_ELECTRICAL_09_HIGH,
    0xc27du16 => & ENERGY_ELECTRICAL_10_LOW, 0xc27eu16 => & ENERGY_ELECTRICAL_10_HIGH,
    0xc27fu16 => & ENERGY_ELECTRICAL_11_LOW, 0xc280u16 => & ENERGY_ELECTRICAL_11_HIGH,
    0xc281u16 => & ENERGY_ELECTRICAL_12_LOW, 0xc282u16 => & ENERGY_ELECTRICAL_12_HIGH,
    0xc283u16 => & ENERGY_HP_COOLING_01_LOW, 0xc284u16 => & ENERGY_HP_COOLING_01_HIGH,
    0xc285u16 => & ENERGY_HP_COOLING_02_LOW, 0xc286u16 => & ENERGY_HP_COOLING_02_HIGH,
    0xc287u16 => & ENERGY_HP_COOLING_03_LOW, 0xc288u16 => & ENERGY_HP_COOLING_03_HIGH,
    0xc289u16 => & ENERGY_HP_COOLING_04_LOW, 0xc28au16 => & ENERGY_HP_COOLING_04_HIGH,
    0xc28bu16 => & ENERGY_HP_COOLING_05_LOW, 0xc28cu16 => & ENERGY_HP_COOLING_05_HIGH,
    0xc28du16 => & ENERGY_HP_COOLING_06_LOW, 0xc28eu16 => & ENERGY_HP_COOLING_06_HIGH,
    0xc28fu16 => & ENERGY_HP_COOLING_07_LOW, 0xc290u16 => & ENERGY_HP_COOLING_07_HIGH,
    0xc291u16 => & ENERGY_HP_COOLING_08_LOW, 0xc292u16 => & ENERGY_HP_COOLING_08_HIGH,
    0xc293u16 => & ENERGY_HP_COOLING_09_LOW, 0xc294u16 => & ENERGY_HP_COOLING_09_HIGH,
    0xc295u16 => & ENERGY_HP_COOLING_10_LOW, 0xc296u16 => & ENERGY_HP_COOLING_10_HIGH,
    0xc297u16 => & ENERGY_HP_COOLING_11_LOW, 0xc298u16 => & ENERGY_HP_COOLING_11_HIGH,
    0xc299u16 => & ENERGY_HP_COOLING_12_LOW, 0xc29au16 => & ENERGY_HP_COOLING_12_HIGH,
    0xc29bu16 => & ENERGY_HP_HEATING_01_LOW, 0xc29cu16 => & ENERGY_HP_HEATING_01_HIGH,
    0xc29du16 => & ENERGY_HP_HEATING_02_LOW, 0xc29eu16 => & ENERGY_HP_HEATING_02_HIGH,
    0xc29fu16 => & ENERGY_HP_HEATING_03_LOW, 0xc2a0u16 => & ENERGY_HP_HEATING_03_HIGH,
    0xc2a1u16 => & ENERGY_HP_HEATING_04_LOW, 0xc2a2u16 => & ENERGY_HP_HEATING_04_HIGH,
    0xc2a3u16 => & ENERGY_HP_HEATING_05_LOW, 0xc2a4u16 => & ENERGY_HP_HEATING_05_HIGH,
    0xc2a5u16 => & ENERGY_HP_HEATING_06_LOW, 0xc2a6u16 => & ENERGY_HP_HEATING_06_HIGH,
    0xc2a7u16 => & ENERGY_HP_HEATING_07_LOW, 0xc2a8u16 => & ENERGY_HP_HEATING_07_HIGH,
    0xc2a9u16 => & ENERGY_HP_HEATING_08_LOW, 0xc2aau16 => & ENERGY_HP_HEATING_08_HIGH,
    0xc2abu16 => & ENERGY_HP_HEATING_09_LOW, 0xc2acu16 => & ENERGY_HP_HEATING_19_HIGH,
    0xc2adu16 => & ENERGY_HP_HEATING_10_LOW, 0xc2aeu16 => & ENERGY_HP_HEATING_10_HIGH,
    0xc2afu16 => & ENERGY_HP_HEATING_11_LOW, 0xc2b0u16 => & ENERGY_HP_HEATING_11_HIGH,
    0xc2b1u16 => & ENERGY_HP_HEATING_12_LOW, 0xc2b2u16 => & ENERGY_HP_HEATING_12_HIGH,
    0xc2b3u16 => & ENERGY_HOT_WATER_01_LOW, 0xc2b4u16 => & ENERGY_HOT_WATER_01_HIGH,
    0xc2b5u16 => & ENERGY_HOT_WATER_02_LOW, 0xc2b6u16 => & ENERGY_HOT_WATER_02_HIGH,
    0xc2b7u16 => & ENERGY_HOT_WATER_03_LOW, 0xc2b8u16 => & ENERGY_HOT_WATER_03_HIGH,
    0xc2b9u16 => & ENERGY_HOT_WATER_04_LOW, 0xc2bau16 => & ENERGY_HOT_WATER_04_HIGH,
    0xc2bbu16 => & ENERGY_HOT_WATER_05_LOW, 0xc2bcu16 => & ENERGY_HOT_WATER_05_HIGH,
    0xc2bdu16 => & ENERGY_HOT_WATER_06_LOW, 0xc2beu16 => & ENERGY_HOT_WATER_06_HIGH,
    0xc2bfu16 => & ENERGY_HOT_WATER_07_LOW, 0xc2c0u16 => & ENERGY_HOT_WATER_07_HIGH,
    0xc2c1u16 => & ENERGY_HOT_WATER_08_LOW, 0xc2c2u16 => & ENERGY_HOT_WATER_08_HIGH,
    0xc2c3u16 => & ENERGY_HOT_WATER_09_LOW, 0xc2c4u16 => & ENERGY_HOT_WATER_09_HIGH,
    0xc2c5u16 => & ENERGY_HOT_WATER_10_LOW, 0xc2c6u16 => & ENERGY_HOT_WATER_10_HIGH,
    0xc2c7u16 => & ENERGY_HOT_WATER_11_LOW, 0xc2c8u16 => & ENERGY_HOT_WATER_11_HIGH,
    0xc2c9u16 => & ENERGY_HOT_WATER_12_LOW, 0xc2cau16 => & ENERGY_HOT_WATER_12_HIGH,
    0xc2cbu16 => & ENERGY_HP_TOTAL_01_LOW, 0xc2ccu16 => & ENERGY_HP_TOTAL_01_HIGH,
    0xc2cdu16 => & ENERGY_HP_TOTAL_02_LOW, 0xc2ceu16 => & ENERGY_HP_TOTAL_02_HIGH,
    0xc2cfu16 => & ENERGY_HP_TOTAL_03_LOW, 0xc2d0u16 => & ENERGY_HP_TOTAL_03_HIGH,
    0xc2d1u16 => & ENERGY_HP_TOTAL_04_LOW, 0xc2d2u16 => & ENERGY_HP_TOTAL_04_HIGH,
    0xc2d3u16 => & ENERGY_HP_TOTAL_05_LOW, 0xc2d4u16 => & ENERGY_HP_TOTAL_05_HIGH,
    0xc2d5u16 => & ENERGY_HP_TOTAL_06_LOW, 0xc2d6u16 => & ENERGY_HP_TOTAL_06_HIGH,
    0xc2d7u16 => & ENERGY_HP_TOTAL_07_LOW, 0xc2d8u16 => & ENERGY_HP_TOTAL_07_HIGH,
    0xc2d9u16 => & ENERGY_HP_TOTAL_08_LOW, 0xc2dau16 => & ENERGY_HP_TOTAL_08_HIGH,
    0xc2dbu16 => & ENERGY_HP_TOTAL_09_LOW, 0xc2dcu16 => & ENERGY_HP_TOTAL_09_HIGH,
    0xc2ddu16 => & ENERGY_HP_TOTAL_10_LOW, 0xc2deu16 => & ENERGY_HP_TOTAL_10_HIGH,
    0xc2dfu16 => & ENERGY_HP_TOTAL_11_LOW, 0xc2e0u16 => & ENERGY_HP_TOTAL_11_HIGH,
    0xc2e1u16 => & ENERGY_HP_TOTAL_12_LOW, 0xc2e2u16 => & ENERGY_HP_TOTAL_12_HIGH,
    0xc2f6u16 => & SOLAR_FUNCTION, 0xc2fau16 => & ENERGY_ELECTRICAL, 0xc34cu16 => &
    OUTDOOR_UNIT, 0xc34du16 => & INDOOR_UNIT, 0xfd4fu16 => & G1_ANTILEG_START_ZEIT,
    0xfdacu16 => & STATUS_HEAT_CIRCULATION_PUMP, 0xfdf4u16 => & ANTI_LEGIONELLA_TIME
};
