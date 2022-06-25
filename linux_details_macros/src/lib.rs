use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    AttrStyle, Attribute, Data, DataEnum, DeriveInput, LitStr, Token,
};

struct OsTypes {
    _paren: Paren,
    types: Punctuated<Ident, Token![,]>,
}

impl Parse for OsTypes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let types;

        Ok(Self {
            _paren: parenthesized!(types in input),
            types: Punctuated::<Ident, Token![,]>::parse_terminated(&types)?,
        })
    }
}

struct DisplayName {
    _paren: Paren,
    name: LitStr,
}

impl Parse for DisplayName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name;

        Ok(Self {
            _paren: parenthesized!(name in input),
            name: name.parse()?,
        })
    }
}

struct LinuxDetailsEnum {
    name: Ident,
    os_type_variant: Vec<(Ident, Ident)>,
    default_ident: Ident,
    default_display: proc_macro2::TokenStream,
}

fn linux_details_enum(input: TokenStream) -> LinuxDetailsEnum {
    let DeriveInput {
        ident: enum_name,
        data,
        ..
    } = syn::parse(input).unwrap();
    let DataEnum { variants, .. } = match data {
        Data::Enum(enum_data) => enum_data,
        _ => panic!("Only Enums are supported"),
    };

    let mut idents = vec![];
    let mut strs = vec![];
    let mut variant_os_types: HashMap<Ident, Vec<Ident>> = HashMap::new();
    let mut default_ident = None;
    let mut display_names = HashMap::new();

    for variant in variants {
        if !variant.fields.is_empty() {
            panic!("Variants with fields are not supported");
        }

        let ident = variant.ident;
        let ident_str = ident.to_string();

        idents.push(ident.clone());
        strs.push(ident_str.clone());

        for attr in variant.attrs {
            if let Attribute {
                style: AttrStyle::Outer,
                path,
                tokens,
                ..
            } = attr
            {
                let segments = path.segments;

                if let Some(attr_name) = segments.first() {
                    let attr_name_str = attr_name.ident.to_string();

                    match attr_name_str.as_str() {
                        "default_variant" => {
                            if default_ident.is_none() {
                                default_ident = Some(ident.clone());
                            }
                        }
                        "os_types" => {
                            let types: OsTypes = syn::parse(tokens.into()).unwrap();

                            for os_type in types.types {
                                let vec = variant_os_types.entry(ident.clone()).or_default();

                                if !vec.contains(&os_type) {
                                    vec.push(os_type);
                                }
                            }
                        }
                        "display_name" => {
                            let DisplayName { name, .. } = syn::parse(tokens.into()).unwrap();

                            display_names.insert(ident_str.clone(), name.value());
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if default_ident.is_none() {
        panic!("No default variant marked for {}", enum_name);
    }

    let default_ident = default_ident.unwrap();

    let default = quote! {
        impl Default for #enum_name {
            #[inline]
            fn default() -> Self {
                #enum_name::#default_ident
            }
        }
    };

    let display = {
        let package_manager_display_str = strs.iter().map(|str| {
            display_names
                .entry(str.clone())
                .or_insert_with(|| str.to_lowercase())
                .clone()
        });

        quote! {
            impl std::fmt::Display for #enum_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    match self {
                        #(
                            #enum_name::#idents => write!(f, "{}", #package_manager_display_str),
                        )*
                    }
                }
            }
        }
    };

    let default_display = quote! {
        #default
        #display
    };

    LinuxDetailsEnum {
        name: enum_name,
        os_type_variant: if variant_os_types.is_empty() {
            vec![]
        } else {
            variant_os_types
                .iter()
                .flat_map(|(variant, os_types)| {
                    os_types
                        .iter()
                        .map(|os_type| (os_type.clone(), variant.clone()))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        },
        default_ident,
        default_display,
    }
}

#[proc_macro_derive(PackageManager, attributes(default_variant, os_types, display_name))]
pub fn package_manager(input: TokenStream) -> TokenStream {
    let LinuxDetailsEnum {
        name,
        os_type_variant,
        default_ident,
        default_display,
    } = linux_details_enum(input);

    let get_package_manager = {
        let arms = os_type_variant.iter().map(|(os_type, variant)| {
            quote! {
                os_info::Type::#os_type => #name::#variant
            }
        });

        quote! {
            impl #name {
                pub fn get_package_manager(os_type: os_info::Type) -> Self {
                    match os_type {
                        #(#arms),*,
                        _ => #name::#default_ident
                    }
                }
            }
        }
    };

    (quote! {
        #get_package_manager
        #default_display
    })
    .into()
}

#[proc_macro_derive(Init, attributes(default_variant, os_types, display_name))]
pub fn init(input: TokenStream) -> TokenStream {
    let LinuxDetailsEnum {
        name,
        default_ident,
        default_display,
        ..
    } = linux_details_enum(input);

    let get_init = quote! {
        impl #name {
            pub fn get_init() -> #name {
                #name::#default_ident
            }
        }
    };

    (quote! {
        #get_init
        #default_display
    })
    .into()
}

#[proc_macro_derive(Family, attributes(default_variant, os_types, display_name))]
pub fn family(input: TokenStream) -> TokenStream {
    let LinuxDetailsEnum {
        name,
        os_type_variant,
        default_ident,
        default_display,
    } = linux_details_enum(input);

    let get_family = {
        let arms = os_type_variant.iter().map(|(os_type, variant)| {
            quote! {
                os_info::Type::#os_type => #name::#variant
            }
        });

        quote! {
            impl #name {
                pub fn get_family(os_type: os_info::Type) -> Self {
                    match os_type {
                        #(#arms),*,
                        _ => #name::#default_ident
                    }
                }
            }
        }
    };

    (quote! {
        #get_family
        #default_display
    })
    .into()
}
