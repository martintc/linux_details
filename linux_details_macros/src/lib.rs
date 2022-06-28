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

struct ParenPunctuated {
    _paren: Paren,
    types: Punctuated<Ident, Token![,]>,
}

impl Parse for ParenPunctuated {
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
        let paren = parenthesized!(name in input);

        Ok(Self {
            _paren: paren,
            name: name.parse()?,
        })
    }
}

struct LinuxDetailsEnum {
    name: Ident,
    os_type_variants: HashMap<Ident, Vec<Ident>>,
    main_in_os_types: HashMap<Ident, Vec<Ident>>,
    default_ident: Ident,
    default_display: proc_macro2::TokenStream,
}

#[proc_macro_derive(
    LinuxDetailsEnum,
    attributes(default_variant, os_types, display_name, main_in_os_types)
)]
pub fn linux_details_enum(input: TokenStream) -> TokenStream {
    let LinuxDetailsEnum {
        default_display, ..
    } = linux_details_enum_internal(input);

    (quote! {
        #default_display
    })
    .into()
}

fn linux_details_enum_internal(input: TokenStream) -> LinuxDetailsEnum {
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
    let mut os_type_variants: HashMap<Ident, Vec<Ident>> = HashMap::new();
    let mut main_in_os_types: HashMap<Ident, Vec<Ident>> = HashMap::new();
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
                            let types: ParenPunctuated = syn::parse(tokens.into()).unwrap();

                            for os_type in types.types {
                                os_type_variants
                                    .entry(os_type)
                                    .or_default()
                                    .push(ident.clone());
                            }
                        }
                        "display_name" => {
                            let DisplayName { name, .. } = syn::parse(tokens.into()).unwrap();

                            display_names.insert(ident_str.clone(), name.value());
                        }
                        "main_in_os_types" => {
                            let types: ParenPunctuated = syn::parse(tokens.into()).unwrap();

                            for os_type in types.types {
                                main_in_os_types
                                    .entry(ident.clone())
                                    .or_default()
                                    .push(os_type);
                            }
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

        os_type_variants,
        default_ident,
        default_display,
        main_in_os_types,
    }
}

#[proc_macro_derive(
    PackageManager,
    attributes(default_variant, os_types, display_name, main_in_os_types)
)]
pub fn package_manager(input: TokenStream) -> TokenStream {
    let LinuxDetailsEnum {
        name,
        os_type_variants,
        main_in_os_types,
        default_ident,
        default_display,
    } = linux_details_enum_internal(input);

    let available_package_managers = {
        let arms = os_type_variants.iter().map(|(os_type, package_managers)| {
            quote! {
                os_info::Type::#os_type => vec![#(#name::#package_managers),*],
            }
        });

        quote! {
            impl #name {
                pub fn available_package_managers(os_type: os_info::Type) -> Vec<Self> {
                    match os_type {
                        #(#arms),*,
                        _ => vec![],
                    }
                }
            }
        }
    };

    let main_package_manager = {
        let arms = main_in_os_types.iter().map(|(package_manager, os_types)| {
            quote! {
                #(#os_types)|* => #name::#package_manager
            }
        });

        quote! {
            impl #name {
                pub fn main_package_manager(os_type: os_info::Type) -> Self {
                    match os_type {
                        #(#arms),*,
                        _ => #name::#default_ident,
                    }
                }
            }
        }
    };

    (quote! {
        #available_package_managers
        #main_package_manager
        #default_display
    })
    .into()
}

#[proc_macro_derive(Family, attributes(default_variant, os_types, display_name))]
pub fn family(input: TokenStream) -> TokenStream {
    let LinuxDetailsEnum {
        name,
        os_type_variants,
        default_ident,
        default_display,
        ..
    } = linux_details_enum_internal(input);

    let get_family = {
        let arms = os_type_variants.iter().flat_map(|(os_type, variants)| {
            variants.iter().cloned().map(|variant| {
                let os_type = os_type.clone();

                quote! {
                    os_info::Type::#os_type => #name::#variant
                }
            })
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
