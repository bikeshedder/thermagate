use std::{fs::File, io::Write};

use anyhow::Result;
use itertools::Itertools;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub values: Vec<EnumValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumValue {
    pub value: i16,
    pub name: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Row {
    pub name: String,
    pub value: i16,
    pub code: String,
    pub label_en: String,
    pub label_de: String,
}

pub fn cmd() -> Result<()> {
    let mut stream = quote! {
        use num_enum::FromPrimitive;
        use super::r#enum::{Enum,EnumVariant};
        use super::param::MultilingualStr;
    };

    let mut csv = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("../data/enums.csv")?;

    let rows: Vec<Row> = csv.deserialize::<Row>().try_collect()?;
    for (name, values) in &rows.iter().chunk_by(|row| &row.name) {
        stream.extend(gen_enum(name, values));
    }

    let file = syn::parse_file(&stream.to_string()).unwrap();
    let code = prettyplease::unparse(&file);
    let mut output = File::create("../src/can/enums.rs")?;
    output.write_all(&code.into_bytes())?;

    Ok(())
}

fn gen_enum<'a>(name: &str, rows: impl Iterator<Item = &'a Row>) -> TokenStream {
    let rows = rows.collect::<Vec<_>>();
    let name_ident = quote::format_ident!("{}", name);
    let variants = rows.iter().map(|v| {
        let variant_ident = quote::format_ident!("{}", v.code);
        let value = Literal::i16_unsuffixed(v.value);
        quote! {
            #variant_ident = #value,
        }
    });
    let choices = rows.iter().map(|v| {
        let value = Literal::i16_unsuffixed(v.value);
        let code = &v.code;
        let label_de = &v.label_de;
        let label_en = &v.label_en;
        quote! {
            EnumVariant {
                value: #value,
                code: #code,
                label: MultilingualStr {
                    de: #label_de,
                    en: #label_en,
                },
            }
        }
    });
    quote! {
        #[derive(Debug, FromPrimitive, strum::Display, strum::IntoStaticStr)]
        #[repr(i16)]
        pub enum #name_ident {
            #( #variants )*
            #[num_enum(catch_all)]
            Unknown(i16),
        }
        impl Enum for #name_ident {
            const VARIANTS: &'static [EnumVariant] = &[
                #( #choices ),*
            ];
        }
    }
}
