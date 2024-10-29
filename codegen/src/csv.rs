use std::path::Path;

use anyhow::Result;
use num_traits::Num;
use serde::{de, Deserialize, Deserializer};

use crate::model::{
    BoolParam, DecParam, Document, EnumParam, I16Param, I8Param, Label, Param, TimeParam,
    TimeRangeParam, Type, Unit,
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

pub fn read(filename: impl AsRef<Path>) -> Result<Document> {
    let mut csv = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(filename)?;
    let mut doc = Document::default();
    for result in csv.deserialize::<ParamRow>() {
        let row = result?;
        let param = Param {
            name: row.name,
            label: Label {
                de: row.label_de,
                en: row.label_en,
            },
            r#type: type_from_parts(
                row.r#type,
                row.unit,
                row.scale,
                row.values.as_str(),
                row.default.as_str(),
            ),
            read: row.read,
            write: row.write,
        };
        if doc
            .params
            .insert(row.id.strip_prefix("0x").unwrap().to_owned(), param)
            .is_some()
        {
            panic!("Duplicate param: {}", row.id);
        }
    }
    Ok(doc)
}

fn type_from_parts(
    r#type: TypeName,
    unit: Option<Unit>,
    scale: Option<u32>,
    values: &str,
    default: &str,
) -> Type {
    match r#type {
        TypeName::Bool => Type::Bool(BoolParam {
            default: parse_bool(default),
        }),
        TypeName::Dec => {
            let (min, max) = parse_range(values);
            Type::Dec(DecParam {
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
            Type::I8(I8Param {
                unit,
                min,
                max,
                default: parse_opt_num(default),
            })
        }
        TypeName::I16 => {
            assert_eq!(scale, None);
            let (min, max) = parse_range(values);
            Type::I16(I16Param {
                unit,
                min,
                max,
                default: parse_opt_num(default),
            })
        }
        TypeName::Enum8 => {
            assert_eq!(scale, None);
            assert_eq!(unit, None);
            Type::Enum8(EnumParam {
                r#enum: values.to_owned(),
                default: None,
                /* TODO
                default: match default {
                    "" => None,
                    x => Some(x.parse().unwrap()),
                },
                 */
            })
        }
        TypeName::Enum16 => {
            assert_eq!(scale, None);
            assert_eq!(unit, None);
            Type::Enum16(EnumParam {
                r#enum: values.to_owned(),
                default: None,
                /* TODO
                default: match default {
                    "" => None,
                    x => Some(x.parse().unwrap()),
                },
                 */
            })
        }
        TypeName::TimeRange => {
            assert_eq!(unit, None);
            assert_eq!(values, "");
            Type::TimeRange(TimeRangeParam {
                default: match default {
                    "" => None,
                    x => Some(x.parse().unwrap()),
                },
            })
        }
        TypeName::Time => {
            assert_eq!(unit, None);
            assert_eq!(values, "");
            Type::Time(TimeParam {
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
    }
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
