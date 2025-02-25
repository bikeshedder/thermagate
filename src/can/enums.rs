use crate::model::{
    r#enum::{Enum, EnumVariant},
    string::MultilingualStr,
};

use num_enum::FromPrimitive;
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum OperatingMode {
    Standby = 1,
    Heating = 3,
    Reduce = 4,
    Summer = 5,
    Auto1 = 11,
    Auto2 = 12,
    Manual = 14,
    Cooling = 17,
    Party = 201,
    Vacation = 202,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for OperatingMode {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 1,
            code: "Standby",
            label: MultilingualStr {
                de: "Bereitschaft",
                en: "Standby",
            },
        },
        EnumVariant {
            value: 3,
            code: "Heating",
            label: MultilingualStr {
                de: "Heizen",
                en: "Heating",
            },
        },
        EnumVariant {
            value: 4,
            code: "Reduce",
            label: MultilingualStr {
                de: "Absenken",
                en: "Reduce",
            },
        },
        EnumVariant {
            value: 5,
            code: "Summer",
            label: MultilingualStr {
                de: "Sommer",
                en: "Summer",
            },
        },
        EnumVariant {
            value: 11,
            code: "Auto1",
            label: MultilingualStr {
                de: "Automatik 1",
                en: "Automatic 1",
            },
        },
        EnumVariant {
            value: 12,
            code: "Auto2",
            label: MultilingualStr {
                de: "Automatik 2",
                en: "Automatic 2",
            },
        },
        EnumVariant {
            value: 14,
            code: "Manual",
            label: MultilingualStr {
                de: "Handbetrieb",
                en: "Manual",
            },
        },
        EnumVariant {
            value: 17,
            code: "Cooling",
            label: MultilingualStr {
                de: "Kühlen",
                en: "Cooling",
            },
        },
        EnumVariant {
            value: 201,
            code: "Party",
            label: MultilingualStr {
                de: "Party",
                en: "Party",
            },
        },
        EnumVariant {
            value: 202,
            code: "Vacation",
            label: MultilingualStr {
                de: "Vacation",
                en: "Vacation",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum OutdoorUnit {
    NoSelection = 0,
    KW14 = 1,
    KW16 = 2,
    KW18 = 3,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for OutdoorUnit {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "NoSelection",
            label: MultilingualStr {
                de: "Keine Auswahl",
                en: "No selection",
            },
        },
        EnumVariant {
            value: 1,
            code: "KW14",
            label: MultilingualStr {
                de: "14 kW",
                en: "14 kW",
            },
        },
        EnumVariant {
            value: 2,
            code: "KW16",
            label: MultilingualStr {
                de: "16 kW",
                en: "16 kW",
            },
        },
        EnumVariant {
            value: 3,
            code: "KW18",
            label: MultilingualStr {
                de: "18 kW",
                en: "18 kW",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum IndoorUnit {
    NoSelection = 0,
    ETSHXB16P30D = 1,
    ETSHXB16P50D = 2,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for IndoorUnit {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "NoSelection",
            label: MultilingualStr {
                de: "Keine Auswahl",
                en: "No selection",
            },
        },
        EnumVariant {
            value: 1,
            code: "ETSHXB16P30D",
            label: MultilingualStr {
                de: "ETS(H/X)(B)16P30D",
                en: "ETS(H/X)(B)16P30D",
            },
        },
        EnumVariant {
            value: 2,
            code: "ETSHXB16P50D",
            label: MultilingualStr {
                de: "ETS(H/X)(B)16P50D",
                en: "ETS(H/X)(B)16P50D",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum HeatingSystem {
    FloorHeating = 0,
    Convector = 1,
    Radiator = 2,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for HeatingSystem {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "FloorHeating",
            label: MultilingualStr {
                de: "Fußbodenheizung",
                en: "Floor heating",
            },
        },
        EnumVariant {
            value: 1,
            code: "Convector",
            label: MultilingualStr {
                de: "Konvektor",
                en: "Convector",
            },
        },
        EnumVariant {
            value: 2,
            code: "Radiator",
            label: MultilingualStr {
                de: "Radiator",
                en: "Radiator",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum PumpLimit {
    NoLimit = 0,
    Perc90IndependentOfOperationStatus = 1,
    Perc75IndependentOfOperationStatus = 2,
    Perc65IndependentOfOperationStatus = 3,
    Perc55IndependentOfOperationStatus = 4,
    Perc90NoHeatingOrCoolingRequirement = 5,
    Perc75NoHeatingOrCoolingRequirement = 6,
    Perc65NoHeatingOrCoolingRequirement = 7,
    Perc55NoHeatingOrCoolingRequirement = 8,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for PumpLimit {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "NoLimit",
            label: MultilingualStr {
                de: "Keine Begrezung",
                en: "No limit",
            },
        },
        EnumVariant {
            value: 1,
            code: "Perc90IndependentOfOperationStatus",
            label: MultilingualStr {
                de: "90% (unabhängig vom Betriebszustand)",
                en: "90% (independent of operation status)",
            },
        },
        EnumVariant {
            value: 2,
            code: "Perc75IndependentOfOperationStatus",
            label: MultilingualStr {
                de: "75% (unabhängig vom Betriebszustand)",
                en: "75% (independent of operation status)",
            },
        },
        EnumVariant {
            value: 3,
            code: "Perc65IndependentOfOperationStatus",
            label: MultilingualStr {
                de: "65% (unabhängig vom Betriebszustand)",
                en: "65% (independent of operation status)",
            },
        },
        EnumVariant {
            value: 4,
            code: "Perc55IndependentOfOperationStatus",
            label: MultilingualStr {
                de: "55% (unabhängig vom Betriebszustand)",
                en: "55% (independent of operation status)",
            },
        },
        EnumVariant {
            value: 5,
            code: "Perc90NoHeatingOrCoolingRequirement",
            label: MultilingualStr {
                de: "90% (keine Heiz- oder Kühlanforderung)",
                en: "90% (no heating or cooling requirement)",
            },
        },
        EnumVariant {
            value: 6,
            code: "Perc75NoHeatingOrCoolingRequirement",
            label: MultilingualStr {
                de: "75% (keine Heiz- oder Kühlanforderung)",
                en: "75% (no heating or cooling requirement)",
            },
        },
        EnumVariant {
            value: 7,
            code: "Perc65NoHeatingOrCoolingRequirement",
            label: MultilingualStr {
                de: "65% (keine Heiz- oder Kühlanforderung)",
                en: "65% (no heating or cooling requirement)",
            },
        },
        EnumVariant {
            value: 8,
            code: "Perc55NoHeatingOrCoolingRequirement",
            label: MultilingualStr {
                de: "55% (keine Heiz- oder Kühlanforderung)",
                en: "55% (no heating or cooling requirement)",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum ExtHeatSource {
    NoExternalHeatGenerator = 0,
    BackupHeater = 1,
    DhwPlusHeatingSupport = 2,
    TwoExternalHeatGenerators = 3,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for ExtHeatSource {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "NoExternalHeatGenerator",
            label: MultilingualStr {
                de: "Kein externer Wärmeerzeuger",
                en: "No external heat generator",
            },
        },
        EnumVariant {
            value: 1,
            code: "BackupHeater",
            label: MultilingualStr {
                de: "Backup-Heater BUH",
                en: "Backup heater BUH",
            },
        },
        EnumVariant {
            value: 2,
            code: "DhwPlusHeatingSupport",
            label: MultilingualStr {
                de: "WW + Heiz-Unterstützung",
                en: "DHW + heating support",
            },
        },
        EnumVariant {
            value: 3,
            code: "TwoExternalHeatGenerators",
            label: MultilingualStr {
                de: "Zwei externe Wärmeerzeuger",
                en: "Two external heat generators",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum BivalenceFunction {
    AuxHeatingAlwaysPossible = 0,
    AuxHeatingTBivDependent = 1,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for BivalenceFunction {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "AuxHeatingAlwaysPossible",
            label: MultilingualStr {
                de: "Zuheizen immer möglich",
                en: "Aux. heating always possible",
            },
        },
        EnumVariant {
            value: 1,
            code: "AuxHeatingTBivDependent",
            label: MultilingualStr {
                de: "Zuheizen T-Bivalenz abhängig",
                en: "Aux. heating T-biv. dependent",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum SmartGridMode {
    Comfort = 0,
    Standard = 1,
    Eco = 2,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for SmartGridMode {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "Comfort",
            label: MultilingualStr {
                de: "Komfort",
                en: "Comfort",
            },
        },
        EnumVariant {
            value: 1,
            code: "Standard",
            label: MultilingualStr {
                de: "Standard",
                en: "Standard",
            },
        },
        EnumVariant {
            value: 2,
            code: "Eco",
            label: MultilingualStr {
                de: "Eco",
                en: "Eco",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum HtNtFunction {
    Inactive = 0,
    SwitchOffCompressor = 1,
    SwitchOffCompressorAndBuh = 2,
    SwitchAllOff = 3,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for HtNtFunction {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "Inactive",
            label: MultilingualStr {
                de: "Inaktiv",
                en: "Inactive",
            },
        },
        EnumVariant {
            value: 1,
            code: "SwitchOffCompressor",
            label: MultilingualStr {
                de: "Verdichter ausschalten",
                en: "Switch off compressor",
            },
        },
        EnumVariant {
            value: 2,
            code: "SwitchOffCompressorAndBuh",
            label: MultilingualStr {
                de: "Verdichter + BUH ausschalten",
                en: "Switch off compressor + BUH",
            },
        },
        EnumVariant {
            value: 3,
            code: "SwitchAllOff",
            label: MultilingualStr {
                de: "Alles ausschalten",
                en: "Switch all off",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum HtNtContact {
    NormallyOpenContact = 0,
    NormallyClosedContact = 1,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for HtNtContact {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "NormallyOpenContact",
            label: MultilingualStr {
                de: "Schließerkontakt",
                en: "Normally open contact",
            },
        },
        EnumVariant {
            value: 1,
            code: "NormallyClosedContact",
            label: MultilingualStr {
                de: "Öffnerkontakt",
                en: "Normally closed contact",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum ProgrammableOutput {
    Inactive = 0,
    HeatingCircuitRequest = 1,
    CirculationRequest = 2,
    DirectHeatingCircuitRequest = 3,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for ProgrammableOutput {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "Inactive",
            label: MultilingualStr {
                de: "Inaktiv",
                en: "Inactive",
            },
        },
        EnumVariant {
            value: 1,
            code: "HeatingCircuitRequest",
            label: MultilingualStr {
                de: "Anforderung Heizkreis",
                en: "Heating circuit request",
            },
        },
        EnumVariant {
            value: 2,
            code: "CirculationRequest",
            label: MultilingualStr {
                de: "Anforderung Zirkulation",
                en: "Circulation request",
            },
        },
        EnumVariant {
            value: 3,
            code: "DirectHeatingCircuitRequest",
            label: MultilingualStr {
                de: "Anforderung direkter Heizkreis",
                en: "Direct heating circuit request\"",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum FuncBurnerBlockingContact {
    ResistanceValues = 0,
    BurnerBlockingContact = 1,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for FuncBurnerBlockingContact {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "ResistanceValues",
            label: MultilingualStr {
                de: "Widerstandswerte",
                en: "Resistance values",
            },
        },
        EnumVariant {
            value: 1,
            code: "BurnerBlockingContact",
            label: MultilingualStr {
                de: "Brennersperrkontakt",
                en: "Burner blocking contact",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum AuxSwitchingFunction {
    Inactive = 0,
    SwitchingThresholdTdhw = 1,
    HeatingCoolingRequest = 2,
    BuhRequest = 3,
    Error = 4,
    Tvbh60 = 5,
    OutsideTemperature = 6,
    OutsideTempPlusDhwHeating = 7,
    DhwRequest = 8,
    OutsideTempPlusHeating = 9,
    MultiOil = 10,
    CoolingMode = 11,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for AuxSwitchingFunction {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "Inactive",
            label: MultilingualStr {
                de: "Inaktiv",
                en: "Inactive",
            },
        },
        EnumVariant {
            value: 1,
            code: "SwitchingThresholdTdhw",
            label: MultilingualStr {
                de: "Schaltschwelle TDHW (AUX)",
                en: "Switching threshold TDHW (AUX)",
            },
        },
        EnumVariant {
            value: 2,
            code: "HeatingCoolingRequest",
            label: MultilingualStr {
                de: "Anforderung Heizen/Kühlen",
                en: "Heating/cooling request",
            },
        },
        EnumVariant {
            value: 3,
            code: "BuhRequest",
            label: MultilingualStr {
                de: "Anforderung BUH",
                en: "BUH request",
            },
        },
        EnumVariant {
            value: 4,
            code: "Error",
            label: MultilingualStr {
                de: "Fehler",
                en: "Error",
            },
        },
        EnumVariant {
            value: 5,
            code: "Tvbh60",
            label: MultilingualStr {
                de: "TVBH > 60°C",
                en: "TVBH > 60°C",
            },
        },
        EnumVariant {
            value: 6,
            code: "OutsideTemperature",
            label: MultilingualStr {
                de: "Außentemperatur",
                en: "Outside temperature",
            },
        },
        EnumVariant {
            value: 7,
            code: "OutsideTempPlusDhwHeating",
            label: MultilingualStr {
                de: "Außentemperatur + WW/Heizen",
                en: "Outside temp. + DHW/heating",
            },
        },
        EnumVariant {
            value: 8,
            code: "DhwRequest",
            label: MultilingualStr {
                de: "Anforderung WW",
                en: "DHW request",
            },
        },
        EnumVariant {
            value: 9,
            code: "OutsideTempPlusHeating",
            label: MultilingualStr {
                de: "Außentemperatur + Heizen",
                en: "Outside temperature + heating",
            },
        },
        EnumVariant {
            value: 10,
            code: "MultiOil",
            label: MultilingualStr {
                de: "Multi-Oil",
                en: "Multi-oil",
            },
        },
        EnumVariant {
            value: 11,
            code: "CoolingMode",
            label: MultilingualStr {
                de: "Betriebsart Kühlen",
                en: "Cooling mode",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum OutsideTempSensor {
    IntegratedSensor = 0,
    OptionalSensor = 1,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for OutsideTempSensor {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "IntegratedSensor",
            label: MultilingualStr {
                de: "Integrierter Sensor",
                en: "Integrated sensor",
            },
        },
        EnumVariant {
            value: 1,
            code: "OptionalSensor",
            label: MultilingualStr {
                de: "Optionaler Sensor",
                en: "Optional sensor",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum WeatherCompensated {
    FeedTempFixed = 0,
    WeatherCompensated = 1,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for WeatherCompensated {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "FeedTempFixed",
            label: MultilingualStr {
                de: "Witterungsgeführt",
                en: "Weather-compensated",
            },
        },
        EnumVariant {
            value: 1,
            code: "WeatherCompensated",
            label: MultilingualStr {
                de: "Vorlauftemperatur fix",
                en: "Feed temperature, fixed",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum BuildingInsulation {
    Off = 0,
    Low = 2,
    Normal = 4,
    Good = 8,
    VeryGood = 12,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for BuildingInsulation {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "Off",
            label: MultilingualStr {
                de: "Aus",
                en: "Off",
            },
        },
        EnumVariant {
            value: 2,
            code: "Low",
            label: MultilingualStr {
                de: "Gering",
                en: "Low",
            },
        },
        EnumVariant {
            value: 4,
            code: "Normal",
            label: MultilingualStr {
                de: "Normal",
                en: "Normal",
            },
        },
        EnumVariant {
            value: 8,
            code: "Good",
            label: MultilingualStr {
                de: "Gut",
                en: "Good",
            },
        },
        EnumVariant {
            value: 12,
            code: "VeryGood",
            label: MultilingualStr {
                de: "Sehr gut",
                en: "Very good",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum AntiLegionellaDay {
    Off = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
    Daily = 8,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for AntiLegionellaDay {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "Off",
            label: MultilingualStr {
                de: "Aus",
                en: "Off",
            },
        },
        EnumVariant {
            value: 1,
            code: "Monday",
            label: MultilingualStr {
                de: "Montag",
                en: "Monday",
            },
        },
        EnumVariant {
            value: 2,
            code: "Tuesday",
            label: MultilingualStr {
                de: "Dienstag",
                en: "Tuesday",
            },
        },
        EnumVariant {
            value: 3,
            code: "Wednesday",
            label: MultilingualStr {
                de: "Mittwoch",
                en: "Wednesday",
            },
        },
        EnumVariant {
            value: 4,
            code: "Thursday",
            label: MultilingualStr {
                de: "Donnerstag",
                en: "Thursday",
            },
        },
        EnumVariant {
            value: 5,
            code: "Friday",
            label: MultilingualStr {
                de: "Freitag",
                en: "Friday",
            },
        },
        EnumVariant {
            value: 6,
            code: "Saturday",
            label: MultilingualStr {
                de: "Samstag",
                en: "Saturday",
            },
        },
        EnumVariant {
            value: 7,
            code: "Sunday",
            label: MultilingualStr {
                de: "Sonntag",
                en: "Sunday",
            },
        },
        EnumVariant {
            value: 8,
            code: "Daily",
            label: MultilingualStr {
                de: "Täglich",
                en: "Daily",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum Mode {
    NoRequest = 0,
    Heating = 1,
    Cooling = 2,
    Defrost = 3,
    HotWater = 4,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for Mode {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "NoRequest",
            label: MultilingualStr {
                de: "Keine Anforderung",
                en: "No request",
            },
        },
        EnumVariant {
            value: 1,
            code: "Heating",
            label: MultilingualStr {
                de: "Heizen",
                en: "Heating",
            },
        },
        EnumVariant {
            value: 2,
            code: "Cooling",
            label: MultilingualStr {
                de: "Kühlen",
                en: "Cooling",
            },
        },
        EnumVariant {
            value: 3,
            code: "Defrost",
            label: MultilingualStr {
                de: "Abtauen",
                en: "Defrost",
            },
        },
        EnumVariant {
            value: 4,
            code: "HotWater",
            label: MultilingualStr {
                de: "Warmwasserbereitung",
                en: "Hot water",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum ExternalRequest {
    NoExternalMode = 0,
    LowRate = 1,
    HighRate = 2,
    SGN = 3,
    SG1 = 4,
    SG2 = 5,
    SG3 = 6,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for ExternalRequest {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "NoExternalMode",
            label: MultilingualStr {
                de: "Kein externer Modus",
                en: "No external request",
            },
        },
        EnumVariant {
            value: 1,
            code: "LowRate",
            label: MultilingualStr {
                de: "Niedertarif",
                en: "Low rate",
            },
        },
        EnumVariant {
            value: 2,
            code: "HighRate",
            label: MultilingualStr {
                de: "Hochtarif",
                en: "High rate",
            },
        },
        EnumVariant {
            value: 3,
            code: "SGN",
            label: MultilingualStr {
                de: "SGN",
                en: "SGN",
            },
        },
        EnumVariant {
            value: 4,
            code: "SG1",
            label: MultilingualStr {
                de: "SG1",
                en: "SG1",
            },
        },
        EnumVariant {
            value: 5,
            code: "SG2",
            label: MultilingualStr {
                de: "SG2",
                en: "SG2",
            },
        },
        EnumVariant {
            value: 6,
            code: "SG3",
            label: MultilingualStr {
                de: "SG3",
                en: "SG3",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum RoomThermostatInterlink {
    Off = 0,
    Request = 1,
    NoHeatRequest = 2,
    IL1 = 3,
    IL2 = 4,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for RoomThermostatInterlink {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "Off",
            label: MultilingualStr {
                de: "Aus",
                en: "Off",
            },
        },
        EnumVariant {
            value: 1,
            code: "Request",
            label: MultilingualStr {
                de: "Anforderung",
                en: "Request",
            },
        },
        EnumVariant {
            value: 2,
            code: "NoHeatRequest",
            label: MultilingualStr {
                de: "Keine Wärmeanforderung",
                en: "No heat request",
            },
        },
        EnumVariant {
            value: 3,
            code: "IL1",
            label: MultilingualStr {
                de: "IL1",
                en: "IL1",
            },
        },
        EnumVariant {
            value: 4,
            code: "IL2",
            label: MultilingualStr {
                de: "IL2",
                en: "IL2",
            },
        },
    ];
}
#[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
#[repr(i16)]
pub enum HeatGeneratorType {
    HPSU = 0,
    A1Gb = 1,
    A1Bo = 2,
    Gcu3xx = 4,
    Gcu5xx = 8,
    #[num_enum(catch_all)]
    Unknown(i16),
}
impl Enum for HeatGeneratorType {
    const VARIANTS: &'static [EnumVariant] = &[
        EnumVariant {
            value: 0,
            code: "HPSU",
            label: MultilingualStr {
                de: "HPSU",
                en: "HPSU",
            },
        },
        EnumVariant {
            value: 1,
            code: "A1Gb",
            label: MultilingualStr {
                de: "A1_GB",
                en: "A1_GB",
            },
        },
        EnumVariant {
            value: 2,
            code: "A1Bo",
            label: MultilingualStr {
                de: "A1_BO",
                en: "A1_BO",
            },
        },
        EnumVariant {
            value: 4,
            code: "Gcu3xx",
            label: MultilingualStr {
                de: "GCU_3xx",
                en: "GCU_3xx",
            },
        },
        EnumVariant {
            value: 8,
            code: "Gcu5xx",
            label: MultilingualStr {
                de: "GCU_5xx",
                en: "GCU_5xx",
            },
        },
    ];
}
