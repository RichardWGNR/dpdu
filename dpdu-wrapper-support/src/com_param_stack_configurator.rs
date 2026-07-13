use proc_macro::TokenStream;
use convert_case::{Boundary};
use proc_macro2::Ident;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, GenericArgument, PathArguments, Token, Type};
use syn::punctuated::Punctuated;

struct ParamDef {
    name: Ident,
    ty: Type
}

impl Parse for ParamDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        Ok(ParamDef {
            name,
            ty
        })
    }
}

pub fn impl_configure_from_serde_json_map_for_com_param_stack(input: TokenStream) -> TokenStream {
    use convert_case::{Case, Casing};

    let args = parse_macro_input!(input with Punctuated::<ParamDef, Token![,]>::parse_terminated);
    let branches = args.iter().map(|param| {
        let name_str = param.name.to_string();

        let maybe_raw_field = {
            name_str.chars()
                .skip(3)
                .collect::<String>()
                .from_case(Case::UpperCamel)
                .without_boundaries(&[Boundary::UPPER_DIGIT])
                .to_case(Case::Snake)
        };

        let field = match ["as"].contains(&maybe_raw_field.as_str()) {
            true => Ident::new_raw(&maybe_raw_field, param.name.span()),
            false => Ident::new(&maybe_raw_field, param.name.span())
        };

        let extract = if let Type::Path(ref path) = param.ty {
            let t = path.path.segments.iter()
                .map(|v| {
                    let mut append = String::new();

                    match &v.arguments {
                        PathArguments::AngleBracketed(args) => {
                            for arg in args.args.iter() {
                                match arg {
                                    GenericArgument::Type(ty) => {
                                        if let Type::Path(path) = ty {
                                            let str = path.path.segments.last().unwrap().ident.to_string();
                                            append = format!("<{str}>");
                                            break;
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        },
                        _ => {}
                    }

                    format!("{}{append}", v.ident.to_string())
                })
                .collect::<String>();

            match t.as_str() {
                "u32" => quote! {
                    match value.as_u64() {
                        Some(v) if v <= u32::MAX as u64 => self.#field = v as u32,
                        _ => error!(com_param = name, value = format!("{value:?}"), "Invalid ComParam value"),
                    }
                },
                "Vec<u8>" => quote! {
                    match value.as_array() {
                        Some(array) => {
                            let mut bytes = Vec::new();
                            for item in array {
                                match item.as_u64() {
                                    Some(v) if v <= u8::MAX as u64 => bytes.push(v as u8),
                                    _ => {
                                        error!(com_param = name, value = format!("{value:?}"), "Invalid ComParam value");
                                        continue 'ml;
                                    }
                                }
                            }
                            self.#field = bytes;
                        },
                        None => error!(com_param = name, value = format!("{value:?}"), "Invalid ComParam value"),
                    }
                },
                "Vec<u32>" => quote! {
                    match value.as_array() {
                        Some(array) => {
                            let mut bytes = Vec::new();
                            for item in array {
                                match item.as_u64() {
                                    Some(v) if v <= u32::MAX as u64 => bytes.push(v as u32),
                                    _ => {
                                        error!(com_param = name, value = format!("{value:?}"), "Invalid ComParam value");
                                        continue 'ml;
                                    }
                                }
                            }
                            self.#field = bytes;
                        },
                        None => error!(com_param = name, value = format!("{value:?}"), "Invalid ComParam value"),
                    }
                },
                _ => panic!("Unaccepted type")
            }
        } else {
            panic!("Unsupported syntax");
        };

        quote! {
            #name_str => {
                #extract
            }
        }
    });

    let output = quote! {
        fn configure_from_serde_json_map(&mut self, hash_map: &std::collections::HashMap<String, serde_json::Value>) {
            use tracing::error;

            'ml: for (name, value) in hash_map {
                match name.as_str() {
                    #(#branches,)*
                    _ => {
                        //error!(com_param = name, "Unsupported ComParam in JSON map");
                    }
                }
            }
        }
    };

    output.into()
}