use std::{collections::BTreeMap, fs::File, io::Write};

use anyhow::Result;
use heck::ToShoutySnakeCase;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use rust_decimal::Decimal;

use crate::model::{
    BoolParam, DecParam, EnumParam, I16Param, I8Param, Param, TimeParam, TimeRangeParam, Type, Unit,
};

pub fn cmd() -> Result<()> {
    let doc = crate::csv::read("../data/params.csv")?;

    let mut stream = quote! {
        use rust_decimal_macros::dec;
        use super::{
            enums,
            param::{
                BoolParam, DecParam, Enum16Param, Enum8Param, I16Param, I8Param,
                MultilingualStr, Param, Time, TimeRange, TimeRangeParam, TimeParam, Unit
            }
        };
    };
    for (id, param) in &doc.params {
        let id = u16::from_str_radix(id, 16)?;
        stream.extend(gen_param(id, param))
    }
    stream.extend(gen_params_map(&doc.params));

    let file = syn::parse_file(&stream.to_string()).unwrap();

    let code = prettyplease::unparse(&file);
    let mut output = File::create("../src/can/params.rs")?;
    output.write_all(&code.into_bytes())?;

    Ok(())
}

fn gen_param(id: u16, param: &Param) -> TokenStream {
    match &param.r#type {
        Type::Bool(p) => gen_param_bool(id, param, p),
        Type::I8(p) => gen_param_i8(id, param, p),
        Type::I16(p) => gen_param_i16(id, param, p),
        Type::Dec(p) => gen_param_dec(id, param, p),
        Type::Enum8(p) => gen_param_enum(id, param, p, 8),
        Type::Enum16(p) => gen_param_enum(id, param, p, 16),
        Type::TimeRange(p) => gen_param_time_range(id, param, p),
        Type::Time(p) => gen_param_time(id, param, p),
    }
}

fn gen_simple_param(
    id: u16,
    param: &Param,
    ptype_with_generics: TokenStream,
    ptype: TokenStream,
    extra: TokenStream,
) -> TokenStream {
    let name = param.name.to_shouty_snake_case();
    let name_ident = quote::format_ident!("{}", name);
    let label_de = &param.label.de;
    let label_en = &param.label.en;
    let id = format!("0x{id:04x}").parse::<Literal>().unwrap();
    let read = param.read;
    let write = param.write;
    quote! {
        pub const #name_ident: #ptype_with_generics = #ptype {
            id: #id,
            name: #name,
            label: MultilingualStr {
                de: #label_de,
                en: #label_en,
            },
            read: #read,
            write: #write,
            #extra
        };
    }
}

fn gen_param_bool(id: u16, param: &Param, p: &BoolParam) -> TokenStream {
    let default = p
        .default
        .map(|v| {
            quote! { Some(#v) }
        })
        .unwrap_or(quote! { None });
    gen_simple_param(
        id,
        param,
        quote! { BoolParam },
        quote! { BoolParam },
        quote! {
            default: #default,
        },
    )
}

fn gen_param_i8(
    id: u16,
    param: &Param,
    I8Param {
        unit,
        default,
        min,
        max,
    }: &I8Param,
) -> TokenStream {
    let unit = unit_to_stream(unit);
    let default = opt_i8(default);
    let min = opt_i8(min);
    let max = opt_i8(max);
    gen_simple_param(
        id,
        param,
        quote! { I8Param },
        quote! { I8Param },
        quote! {
            unit: #unit,
            default: #default,
            min: #min,
            max: #max,
        },
    )
}

fn gen_param_i16(
    id: u16,
    param: &Param,
    I16Param {
        unit,
        default,
        min,
        max,
    }: &I16Param,
) -> TokenStream {
    let unit = unit_to_stream(unit);
    let default = opt_i16(default);
    let min = opt_i16(min);
    let max = opt_i16(max);
    gen_simple_param(
        id,
        param,
        quote! { I16Param },
        quote! { I16Param },
        quote! {
            unit: #unit,
            default: #default,
            min: #min,
            max: #max,
        },
    )
}

fn gen_param_dec(
    id: u16,
    param: &Param,
    DecParam {
        unit,
        scale,
        default,
        min,
        max,
    }: &DecParam,
) -> TokenStream {
    let scale = Literal::u32_unsuffixed(*scale);
    let unit = unit_to_stream(unit);
    let default = opt_dec(default);
    let min = opt_dec(min);
    let max = opt_dec(max);
    gen_simple_param(
        id,
        param,
        quote! { DecParam },
        quote! { DecParam },
        quote! {
            unit: #unit,
            scale: #scale,
            default: #default,
            min: #min,
            max: #max,
        },
    )
}

fn gen_param_time_range(
    id: u16,
    param: &Param,
    TimeRangeParam { default }: &TimeRangeParam,
) -> TokenStream {
    let default = match default {
        Some(range) => {
            let start_hour = Literal::u8_unsuffixed(range.start.hour());
            let start_minute = Literal::u8_unsuffixed(range.start.minute());
            let end_hour = Literal::u8_unsuffixed(range.end.hour());
            let end_minute = Literal::u8_unsuffixed(range.end.minute());
            quote! {
                Some(TimeRange {
                    start: Time::new_const(#start_hour, #start_minute),
                    end: Time::new_const(#end_hour, #end_minute),
                })
            }
        }
        None => quote! { None },
    };
    // FIXME optional default
    gen_simple_param(
        id,
        param,
        quote! { TimeRangeParam },
        quote! { TimeRangeParam },
        quote! {
            default: #default,
        },
    )
}

fn gen_param_time(id: u16, param: &Param, TimeParam { default }: &TimeParam) -> TokenStream {
    let default = match default {
        Some(default) => {
            let default_hour = Literal::u8_unsuffixed(default.hour());
            let default_minute = Literal::u8_unsuffixed(default.minute());
            quote! {
                Some(Time::new_const(#default_hour, #default_minute))
            }
        }
        None => quote! { None },
    };
    // FIXME optional default
    gen_simple_param(
        id,
        param,
        quote! { TimeParam },
        quote! { TimeParam },
        quote! {
            default: #default,
        },
    )
}

fn gen_param_enum(
    id: u16,
    param: &Param,
    EnumParam { r#enum, default }: &EnumParam,
    size: u8,
) -> TokenStream {
    let param_type = quote::format_ident!("Enum{}Param", size);
    let enum_type = quote::format_ident!("{}", r#enum);
    let default = default
        .as_ref()
        .map(|v| {
            let variant = quote::format_ident!("{}", v);
            quote! { Some(enums::#enum_type::#variant) }
        })
        .unwrap_or(quote! { None });
    gen_simple_param(
        id,
        param,
        quote! { #param_type <enums::#enum_type> },
        quote! { #param_type },
        // FIXME
        quote! {
            default: #default,
        },
    )
}

fn gen_params_map(params: &BTreeMap<String, Param>) -> TokenStream {
    let entries = params.iter().map(|(id, param)| {
        let id = format!("0x{id}u16").parse::<Literal>().unwrap();
        let param_name = quote::format_ident!("{}", param.name.to_shouty_snake_case());
        quote! { #id => & #param_name }
    });
    quote! {
        pub static PARAMS: phf::Map<u16, &dyn Param> = phf::phf_map! {
            #( #entries ),*
        };
    }
}

fn unit_to_stream(unit: &Option<Unit>) -> TokenStream {
    if let Some(unit) = unit {
        let variant = quote::format_ident!(
            "{}",
            match unit {
                Unit::DecCelsius => "DegCelsius",
                Unit::Kelvin => "Kelvin",
                Unit::Hours => "Hours",
                Unit::KiloWatt => "KiloWatt",
                Unit::KiloWattHours => "KiloWattHours",
                Unit::Bar => "Bar",
                Unit::Percent => "Percent",
                Unit::LitersPerHour => "LitersPerHour",
                Unit::Ampere => "Ampere",
                Unit::Minutes => "Minutes",
                Unit::Seconds => "Seconds",
            }
        );
        quote! {
            Some(Unit::#variant)
        }
    } else {
        quote! {
            None
        }
    }
}

fn opt_i8(opt: &Option<i8>) -> TokenStream {
    opt.map(|v| {
        let lit = Literal::i8_unsuffixed(v);
        quote! { Some(#lit) }
    })
    .unwrap_or(quote! { None })
}

fn opt_i16(opt: &Option<i16>) -> TokenStream {
    opt.map(|v| {
        let lit = Literal::i16_unsuffixed(v);
        quote! { Some(#lit) }
    })
    .unwrap_or(quote! { None })
}

fn opt_dec(opt: &Option<Decimal>) -> TokenStream {
    opt.map(|v| {
        let v = Literal::f64_unsuffixed(v.try_into().unwrap());
        quote! { Some(dec!(#v)) }
    })
    .unwrap_or(quote! { None })
}
