use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Ident, Literal, TokenStream as TokenStream2, TokenTree};
use quote::quote;
use std::collections::HashMap;
use venial::{parse_declaration, AttributeValue, Declaration, Enum, Punctuated};

fn get_punctuated_idents(tokens: TokenStream2) -> Punctuated<Ident> {
    let mut res = Punctuated::new();

    let mut tokens = tokens.into_iter().peekable();

    loop {
        if tokens.peek().is_none() {
            break;
        }

        let ident = tokens.next().unwrap();

        match ident {
            TokenTree::Ident(ident) => res.push(
                ident,
                tokens.next().map(|tokens| match tokens {
                    TokenTree::Punct(punct) => punct,
                    _ => panic!("unexpected token {:?}", tokens),
                }),
            ),
            _ => panic!("Has to be ident"),
        }
    }

    res
}

#[derive(Debug)]
struct LDEnumVarConf {
    name: Ident,
    is_default_variant: bool,
    os_types: Vec<Ident>,
    default_in_os_types: Vec<Ident>,
    display_name: Literal,
}

#[derive(PartialEq)]
enum LDEnumVarConfFields {
    IsDefaultVariant,
    OsTypes,
    DefaultInOsTypes,
    DisplayName,
    None,
}

impl<'b> From<&'b str> for LDEnumVarConfFields {
    fn from(str: &'b str) -> Self {
        match str {
            "is_default_variant" => LDEnumVarConfFields::IsDefaultVariant,
            "os_types" => LDEnumVarConfFields::OsTypes,
            "default_in_os_types" => LDEnumVarConfFields::DefaultInOsTypes,
            "display_name" => LDEnumVarConfFields::DisplayName,
            _ => panic!("Unknown Field: {}", str),
        }
    }
}

impl From<(Ident, AttributeValue)> for LDEnumVarConf {
    fn from((variant_name, attr_val): (Ident, AttributeValue)) -> Self {
        let mut is_default_variant = false;
        let mut os_types = vec![];
        let mut default_in_os_types = vec![];
        let mut display_name = None;

        match attr_val {
            AttributeValue::Group(_, token_trees) => {
                let mut current_field = LDEnumVarConfFields::None;
                let mut current_field_opened = false;
                let mut current_field_value = false;

                //let mut i = 0;
                for token_tree in token_trees.iter() {
                    /*if token_trees.len() > 1 && i == 1 {
                        panic!("{}", token_tree);
                    }

                    i += 1;*/

                    match token_tree {
                        TokenTree::Group(group) => match current_field {
                            LDEnumVarConfFields::OsTypes
                            | LDEnumVarConfFields::DefaultInOsTypes => {
                                current_field_opened = false;
                                current_field_value = false;

                                if group.delimiter() != Delimiter::Parenthesis {
                                    panic!("Expected group delimiter to be Parenthesis");
                                }

                                let idents: Vec<Ident> = get_punctuated_idents(group.stream())
                                    .iter()
                                    .map(|(ident, _)| ident.clone())
                                    .collect();

                                if current_field == LDEnumVarConfFields::OsTypes {
                                    os_types = idents;
                                } else {
                                    default_in_os_types = idents;
                                }
                            }

                            _ => panic!("Unexpected Group: {}", group),
                        },
                        TokenTree::Ident(ident) => {
                            let ident_str = ident.to_string();

                            match current_field {
                                LDEnumVarConfFields::None => {
                                    current_field = ident_str.as_str().into();

                                    if current_field == LDEnumVarConfFields::IsDefaultVariant {
                                        is_default_variant = true;

                                        current_field_value = true;
                                        current_field_opened = false;
                                    }
                                }

                                LDEnumVarConfFields::IsDefaultVariant => {
                                    panic!("is_default_variant should be empty");
                                }
                                LDEnumVarConfFields::OsTypes => {
                                    os_types.push(ident.clone());
                                    current_field_value = true;
                                }
                                LDEnumVarConfFields::DefaultInOsTypes => {
                                    default_in_os_types.push(ident.clone());
                                    current_field_value = true;
                                }
                                LDEnumVarConfFields::DisplayName => {
                                    panic!("display_name should be a string literal, not an Ident");
                                }
                            }
                        }
                        TokenTree::Literal(literal) => match current_field {
                            LDEnumVarConfFields::DisplayName => {
                                display_name = Some(literal.clone());
                                current_field_value = true;
                                current_field_opened = false;
                            }
                            _ => panic!("Unexpected literal: {}", literal),
                        },
                        TokenTree::Punct(punct) => match punct.as_char() {
                            '=' => match current_field {
                                LDEnumVarConfFields::DisplayName if !current_field_opened => {
                                    current_field_opened = true;
                                }
                                _ => panic!("Unexpected '='"),
                            },
                            ',' => match current_field {
                                LDEnumVarConfFields::IsDefaultVariant => {
                                    current_field_value = false;
                                    current_field = LDEnumVarConfFields::None;
                                }
                                LDEnumVarConfFields::OsTypes
                                | LDEnumVarConfFields::DefaultInOsTypes => {
                                    if !current_field_opened {
                                        current_field_value = false;
                                        current_field = LDEnumVarConfFields::None;
                                    } else if current_field_opened && !current_field_value {
                                        panic!("Unexpected ','");
                                    }
                                }
                                LDEnumVarConfFields::DisplayName => {
                                    if !current_field_opened {
                                        current_field_value = false;
                                        current_field = LDEnumVarConfFields::None;
                                    } else {
                                        panic!("Unexpected ','");
                                    }
                                }
                                LDEnumVarConfFields::None => panic!("Unexpected ','"),
                            },
                            _ => panic!("Unexpected punct: {}", punct),
                        },
                    }
                }
            }
            _ => panic!("LDEnumVarConf::from: expected AttributeValue::Group"),
        }

        Self {
            name: variant_name.clone(),
            is_default_variant,
            os_types,
            default_in_os_types,
            display_name: display_name.unwrap_or_else(|| {
                Literal::string(variant_name.to_string().to_lowercase().as_str())
            }),
        }
    }
}

#[proc_macro_derive(LDEnum, attributes(ld_enum_conf))]
pub fn ld_enum(input: TokenStream) -> TokenStream {
    let type_decl = parse_declaration(input.into()).expect("Has to be an enum declaration");

    match type_decl {
        Declaration::Enum(Enum { name, variants, .. }) => {
            let ld_confs = variants
                .iter()
                .map(|(variant, _)| {
                    let attributes = &variant.attributes;
                    let variant_name = &variant.name;

                    let ld_conf = attributes.iter().find_map(|attr| {
                        let attr_name = attr.path.last().unwrap().to_string();

                        if attr_name.as_str() == "ld_enum_conf" {
                            let val = &attr.value;
                            let var_conf = LDEnumVarConf::from((variant_name.clone(), val.clone()));

                            Some(var_conf)
                        } else {
                            None
                        }
                    });

                    ld_conf.unwrap_or(LDEnumVarConf {
                        name: variant_name.clone(),
                        is_default_variant: false,
                        os_types: vec![],
                        default_in_os_types: vec![],
                        display_name: Literal::string(
                            variant_name.to_string().to_lowercase().as_str(),
                        ),
                    })
                })
                .collect::<Vec<_>>();

            let default = {
                let default_value = ld_confs
                    .iter()
                    .find_map(|ld_conf| {
                        if ld_conf.is_default_variant {
                            let variant_name = ld_conf.name.clone();

                            Some(quote! {
                                Self::#variant_name
                            })
                        } else {
                            None
                        }
                    })
                    .unwrap_or(quote! {
                        enum_iterator::all::<#name>().first().unwrap()
                    });

                quote! {
                    impl Default for #name {
                        #[inline]
                        fn default() -> Self {
                            #default_value
                        }
                    }
                }
            };

            let display = {
                let arms = ld_confs.iter().map(|ld_conf| {
                    let variant_name = ld_conf.name.clone();
                    let display_name = ld_conf.display_name.clone();

                    quote! {
                        Self::#variant_name => #display_name,
                    }
                });

                quote! {
                    impl std::fmt::Display for #name {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f, "{}", match self {
                                #(#arms)*
                            })
                        }
                    }
                }
            };

            let ld_enum = {
                let name_str = name.to_string();

                let funcs = match name_str.as_str() {
                    "Family" => {
                        let mut family_of_os_types: HashMap<Ident, Ident> = HashMap::new();

                        for ld_conf in &ld_confs {
                            for os_type in &ld_conf.os_types {
                                family_of_os_types.insert(os_type.clone(), ld_conf.name.clone());
                            }
                        }

                        let arms = family_of_os_types.iter().map(|(os_type, family)| {
                            quote! {
                                os_info::Type::#os_type => Self::#family,
                            }
                        });

                        quote! {
                            pub fn family_of_os_type(os_type: os_info::Type) -> Self {
                                match os_type {
                                    #(#arms)*
                                    _ => Self::default()
                                }
                            }
                        }
                    }
                    _ => {
                        let supported_in_os_type = {
                            let mut supported_in_os_type: HashMap<Ident, Vec<Ident>> =
                                HashMap::new();

                            for ld_conf in &ld_confs {
                                for os_type in &ld_conf.os_types {
                                    supported_in_os_type
                                        .entry(os_type.clone())
                                        .or_default()
                                        .push(ld_conf.name.clone());
                                }
                            }

                            let arms = supported_in_os_type.iter().map(|(os_type, variants)| {
                                quote! {
                                    os_info::Type::#os_type => vec![#(Self::#variants),*],
                                }
                            });

                            quote! {
                                pub fn supported_in_os_type(os_type: os_info::Type) -> Vec<Self> {
                                    match os_type {
                                        #(#arms)*
                                        _ => vec![],
                                    }
                                }
                            }
                        };

                        let available_in_os_type = {
                            quote! {
                                pub fn available_in_os_type(os_type: os_info::Type) -> Vec<Self> {
                                    Self::supported_in_os_type(os_type).iter().filter_map(|variant| {
                                        which::which(&format!("{}", variant)).ok()
                                            .map(|_| variant.clone())
                                    }).collect()
                                }
                            }
                        };

                        let default_in_os_type = {
                            let mut default_in_os_type: HashMap<Ident, Ident> = HashMap::new();

                            for ld_conf in &ld_confs {
                                for os_type in &ld_conf.default_in_os_types {
                                    default_in_os_type
                                        .insert(os_type.clone(), ld_conf.name.clone());
                                }
                            }

                            let arms = default_in_os_type.iter().map(|(os_type, variant)| {
                                quote! {
                                    os_info::Type::#os_type => Self::#variant,
                                }
                            });

                            quote! {
                                pub fn default_in_os_type(os_type: os_info::Type) -> Self {
                                    match os_type {
                                        #(#arms)*
                                        _ => Self::default(),
                                    }
                                }
                            }
                        };

                        quote! {
                            #supported_in_os_type
                            #available_in_os_type
                            #default_in_os_type
                        }
                    }
                };

                quote! {
                    impl #name {
                        #funcs
                    }
                }
            };

            (quote! {
                #default
                #display
                #ld_enum
            })
            .into()
        }
        _ => panic!("Has to be an enum declaration"),
    }
}
