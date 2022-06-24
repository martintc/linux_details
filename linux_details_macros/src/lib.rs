use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Paren,
    AttrStyle, Attribute, Data, DataEnum, DeriveInput, Token,
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

#[proc_macro_derive(PackageManager, attributes(default_variant, os_types))]
pub fn package_manager(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: enum_name,
        data,
        ..
    } = parse_macro_input!(input);
    let DataEnum {
        variants: package_manager_variants,
        ..
    } = match data {
        Data::Enum(enum_data) => enum_data,
        _ => panic!("Only Enums are supported"),
    };

    let mut package_manager_idents = vec![];
    let mut package_manager_strs = vec![];
    let mut os_type_package_manager: HashMap<Ident, Vec<Ident>> = HashMap::new();
    let mut default = None;

    for package_manager_variant in package_manager_variants {
        if !package_manager_variant.fields.is_empty() {
            panic!("Variants with fields are not supported");
        }

        let ident = package_manager_variant.ident;

        package_manager_idents.push(ident.clone());
        package_manager_strs.push(ident.to_string());

        for attr in package_manager_variant.attrs {
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
                            if default.is_none() {
                                default = Some(ident.clone());
                            }
                        }
                        "os_types" => {
                            let tokens: TokenStream = tokens.into();
                            let types: OsTypes = parse_macro_input!(tokens);

                            for os_type in types.types {
                                let vec = os_type_package_manager.entry(ident.clone()).or_default();

                                if !vec.contains(&os_type) {
                                    vec.push(os_type);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if default.is_none() {
        panic!("No default variant marked for {}", enum_name);
    }

    let default = default.unwrap();

    let get_package_manager = {
        let mut arms = vec![];

        for (package_manager, os_types_with_pm) in os_type_package_manager.iter() {
            for os_type in os_types_with_pm {
                arms.push(quote! {
                    os_info::Type::#os_type => #enum_name::#package_manager
                });
            }
        }

        quote! {
            impl #enum_name {
                pub fn get_package_manager(os_type: os_info::Type) -> Self {
                    match os_type {
                        #(#arms),*,
                        _ => #enum_name::#default
                    }
                }
            }
        }
    };

    let default = quote! {
        impl Default for #enum_name {
            #[inline]
            fn default() -> Self {
                #enum_name::#default
            }
        }
    };

    let display = {
        let package_manager_display_str = package_manager_strs.iter().map(|str| str.to_lowercase());

        quote! {
            impl std::fmt::Display for #enum_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    match self {
                        #(
                            #enum_name::#package_manager_idents => write!(f, "{}", #package_manager_display_str),
                        )*
                    }
                }
            }
        }
    };

    (quote! {
        #get_package_manager
        #default
        #display
    })
    .into()
}
