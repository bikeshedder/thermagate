use std::io::Read;

use itertools::Itertools;
use num_traits::Num;
use serde::{de, Deserialize, Deserializer};

use crate::{
    can::param::CanParam,
    catalog::param::{EnumParam, I16Param, I8Param, TimeParam, TimeRangeParam},
    model::{r#enum::EnumVariant, string::MultilingualString, unit::Unit, value::Value},
};

use super::{
    enums_csv::EnumsCsv,
    error::CatalogError,
    param::{BoolParam, DecParam, Param, ParamType},
    Catalog,
};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum TypeName {
    Bool,
    I8,
    I16,
    Dec,
    Enum8,
    Enum16,
    TimeRange,
    Time,
}

#[derive(Debug, serde::Deserialize)]
enum Device {
    HG1,
    HC1,
    HC2,
    HCM2,
    Outdoor,
}

#[derive(Debug, serde::Deserialize)]
struct ParamRow {
    id: String,
    name: String,
    label_en: String,
    label_de: String,
    r#type: TypeName,
    unit: Option<Unit>,
    scale: Option<u32>,
    device: Option<Device>,
    mixer_device: Option<Device>,
    values: String,
    default: String,
    #[serde(deserialize_with = "deserialize_bool")]
    read: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    write: bool,
}

pub fn read_params(reader: impl Read, enums: EnumsCsv) -> Result<Vec<Param>, CatalogError> {
    let mut csv = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);
    let mut params = Vec::new();
    for result in csv.deserialize::<ParamRow>() {
        let row = result?;
        let param = Param {
            // FIXME proper error handling
            id: parse_id(&row.id),
            name: row.name,
            label: MultilingualString {
                de: row.label_de,
                en: row.label_en,
            },
            r#type: type_from_parts(
                row.r#type,
                row.unit,
                row.scale,
                row.values.as_str(),
                row.default.as_str(),
                &enums,
            )?,
            read: row.read,
            write: row.write,
        };
        params.push(param)
    }
    Ok(params)
}

fn type_from_parts(
    r#type: TypeName,
    unit: Option<Unit>,
    scale: Option<u32>,
    values: &str,
    default: &str,
    enums: &EnumsCsv,
) -> Result<ParamType, CatalogError> {
    Ok(match r#type {
        TypeName::Bool => ParamType::Bool(BoolParam {
            default: parse_bool(default),
        }),
        TypeName::Dec => {
            let (min, max) = parse_range(values);
            ParamType::Dec(DecParam {
                unit,
                scale: scale
                    .map(|s| match s {
                        1000 => 3,
                        100 => 2,
                        10 => 1,
                        _ => panic!("Unsupported scale: {s}"),
                    })
                    .unwrap_or_default(),
                min,
                max,
                default: parse_opt_num(default),
            })
        }
        TypeName::I8 => {
            assert_eq!(scale, None);
            let (min, max) = parse_range(values);
            ParamType::I8(I8Param {
                unit,
                min,
                max,
                default: parse_opt_num(default),
            })
        }
        TypeName::I16 => {
            assert_eq!(scale, None);
            let (min, max) = parse_range(values);
            ParamType::I16(I16Param {
                unit,
                min,
                max,
                default: parse_opt_num(default),
            })
        }
        TypeName::Enum8 => {
            assert_eq!(scale, None);
            assert_eq!(unit, None);
            let variants: Vec<EnumVariant<u8>> = enums
                .enums
                .get(values)
                .ok_or_else(|| CatalogError::UnknownEnum(values.to_owned()))?
                .iter()
                .map(|v| v.clone().try_into())
                .try_collect()?;
            ParamType::Enum8(EnumParam::<u8> {
                default: get_default_enum_value(&variants, default)?,
                variants: variants,
            })
        }
        TypeName::Enum16 => {
            assert_eq!(scale, None);
            assert_eq!(unit, None);
            let variants = enums
                .enums
                .get(values)
                .ok_or_else(|| CatalogError::UnknownEnum(values.to_owned()))?
                .iter()
                .map(|v| v.clone().into())
                .collect::<Vec<_>>();
            ParamType::Enum16(EnumParam {
                default: get_default_enum_value(&variants, default)?,
                variants: variants,
            })
        }
        TypeName::TimeRange => {
            assert_eq!(unit, None);
            assert_eq!(values, "");
            ParamType::TimeRange(TimeRangeParam {
                default: None,
                /*
                default: match default {
                    "" => None,
                    x => Some(x.parse().unwrap()),
                },
                */
            })
        }
        TypeName::Time => {
            assert_eq!(unit, None);
            assert_eq!(values, "");
            ParamType::Time(TimeParam {
                default: None,
                // FIXME
                /*
                default: match default {
                    "" => None,
                    x => Some(x.parse().unwrap()),
                },
                 */
            })
        }
    })
}

fn parse_id(s: &str) -> u16 {
    assert!(s.starts_with("0x"));
    u16::from_str_radix(&s[2..], 16).unwrap()
}

fn parse_opt_num<T: Num>(s: &str) -> Option<T> {
    if s.is_empty() {
        None
    } else {
        Some(parse_num(s))
    }
}

fn parse_num<T: Num>(s: &str) -> T {
    if let Ok(v) = T::from_str_radix(s, 10) {
        v
    } else {
        panic!("Invalid value: {s}");
    }
}

fn parse_range<T: Num>(s: &str) -> (Option<T>, Option<T>) {
    if s.is_empty() {
        return (None, None);
    }
    if let Some((min, max)) = s.rsplit_once("-") {
        (Some(parse_num(min.trim())), Some(parse_num(max.trim())))
    } else {
        panic!("Invalid range: {s}")
    }
}

fn parse_bool(s: &str) -> Option<bool> {
    match s {
        "TRUE" | "true" | "1" => Some(true),
        "FALSE" | "false " | "0" => Some(false),
        _ => None,
    }
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    match buf.as_ref() {
        "TRUE" | "true" | "1" => Ok(true),
        "FALSE" | "false " | "0" => Ok(false),
        other => Err(de::Error::invalid_value(
            de::Unexpected::Str(other),
            &"TRUE or FALSE",
        )),
    }
}

fn get_default_enum_value<T: Clone>(
    variants: &[EnumVariant<T>],
    code: &str,
) -> Result<Option<EnumVariant<T>>, CatalogError> {
    if code == "" {
        return Ok(None);
    }
    variants
        .iter()
        .find(|v| v.code == code)
        .ok_or_else(|| CatalogError::UnknownEnumVariant(code.to_owned()))
        .map(|v| Some(v.clone()))
}

#[test]
fn test_enum8_decode() {
    let catalog = Catalog::load().unwrap();
    let param = catalog.param_by_name("operating_mode").unwrap();
    assert_eq!(
        param.decode([1, 0]),
        Some(Value::Enum8(1, Some("Standby".into()))),
    );
    assert_eq!(
        param.decode([11, 0]),
        Some(Value::Enum8(11, Some("Auto1".into()))),
    );
    assert_eq!(param.decode([99, 0]), Some(Value::Enum8(99, None)),);
}

#[test]
fn test_enum16_decode() {
    let catalog = Catalog::load().unwrap();
    let param = catalog.param_by_name("mode").unwrap();
    assert_eq!(
        param.decode([0, 0]),
        Some(Value::Enum16(0, Some("NoRequest".into()))),
    );
    assert_eq!(
        param.decode([0, 1]),
        Some(Value::Enum16(1, Some("Heating".into()))),
    );
    assert_eq!(param.decode([0, 99]), Some(Value::Enum16(99, None)),);
}
