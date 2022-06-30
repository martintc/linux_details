use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Ident, Literal, TokenStream as TokenStream2, TokenTree};
use quote::quote;
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

impl From<(String, AttributeValue)> for LDEnumVarConf {
    fn from((attr_name, attr_val): (String, AttributeValue)) -> Self {
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
            is_default_variant,
            os_types,
            default_in_os_types,
            display_name: display_name
                .unwrap_or_else(|| Literal::string(attr_name.to_lowercase().as_str())),
        }
    }
}

#[proc_macro_derive(LDEnum, attributes(ld_enum_conf))]
pub fn ld_enum(input: TokenStream) -> TokenStream {
    let type_decl = parse_declaration(input.into()).expect("Has to be an enum declaration");

    match type_decl {
        Declaration::Enum(Enum { name, variants, .. }) => {
            for (variant, _) in variants.iter() {
                let attributes = &variant.attributes;

                let ld_conf = attributes.iter().find_map(|attr| {
                    let attr_name = attr.path.last().unwrap().to_string();

                    if attr_name.as_str() == "ld_enum_conf" {
                        let val = &attr.value;
                        let var_conf = LDEnumVarConf::from((attr_name, val.clone()));

                        Some(var_conf)
                    } else {
                        None
                    }
                });

                if let Some(ld_conf) = ld_conf {
                    todo!("Implement LDEnum trait for {}", name);
                }
            }

            (quote! {}).into()
        }
        _ => panic!("Has to be an enum declaration"),
    }
}
