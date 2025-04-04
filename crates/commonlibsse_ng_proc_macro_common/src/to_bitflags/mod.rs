//! generate bitflags from enum
use core::str::FromStr as _;

use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemEnum;

use crate::{
    enum_parser::{
        filter_default_attr, filter_repr_default_attr, parse_discriminant, select_bitflags_type,
    },
    ffi_enum::attr_args,
};

pub fn to_bitflags(
    attrs: TokenStream,
    item_enum: ItemEnum,
    crate_root_name: TokenStream,
) -> TokenStream {
    let args = {
        let attr_args = match darling::ast::NestedMeta::parse_meta_list(attrs) {
            Ok(v) => v,
            Err(e) => {
                return darling::Error::from(e).write_errors();
            }
        };

        match <attr_args::MacroArgs as darling::FromMeta>::from_list(&attr_args) {
            Ok(v) => v,
            Err(e) => {
                return e.write_errors();
            }
        }
    };
    to_bitflags_inner(args, item_enum, crate_root_name)
        .unwrap_or_else(syn::Error::into_compile_error)
}

fn to_bitflags_inner(
    _args: attr_args::MacroArgs,
    item_enum: ItemEnum,
    crate_root_name: TokenStream,
) -> syn::Result<TokenStream> {
    let enum_ident = &item_enum.ident;
    let vis = &item_enum.vis;

    let (others_attr, repr_attr) = filter_repr_default_attr(&item_enum.attrs);
    let bitflags_type = match repr_attr {
        Some(repr_attr) => select_bitflags_type(repr_attr)?,
        None => quote! { usize },
    };

    // Generate bitflags and match arms
    let DiscriminantData { bitflags, default_value, .. } =
        DiscriminantData::from_item_enum(&item_enum)?;
    let docs = format!("- size type: [`{bitflags_type}`]");

    let expanded = quote! {
        #crate_root_name::__private::bitflags::bitflags! {
            #(#others_attr)*
            ///
            #[doc = #docs]
            #vis struct #enum_ident: #bitflags_type {
                #(#bitflags;)*
            }
        }

        impl Default for #enum_ident {
            #[inline]
            fn default() -> Self {
                Self::from_bits_retain(#default_value)
            }
        }
    };

    Ok(expanded)
}

/// Struct to store discriminant information and current value
pub(crate) struct DiscriminantData {
    /// .e.g `pub const #var_name: Self = Self(#value)`
    pub(crate) bitflags: Vec<TokenStream>,
    #[allow(unused)]
    pub(crate) default_value: TokenStream,
}

macro_rules! to_non_suffix_num_token {
    ($value:expr) => {
        TokenStream::from_str(&format!("{}", $value)).unwrap()
    };
}

/// Generates the discriminants for the enum and prepares the corresponding quote items
impl DiscriminantData {
    pub(crate) fn from_item_enum(item_enum: &ItemEnum) -> syn::Result<Self> {
        let mut current_value = 0;
        let mut bitflags = Vec::new();
        let mut default_value = quote! { 0 };

        for variant in &item_enum.variants {
            let var_name = &variant.ident;
            let (variant_attrs, found_default) = filter_default_attr(&variant.attrs);

            // If use explicit discriminant, change from current discriminant.
            let value = if let Some((_, expr)) = &variant.discriminant {
                let parsed = parse_discriminant(expr)?;
                current_value = parsed; // Set the current value
                if found_default {
                    default_value = quote! { #expr };
                };

                quote! { #expr }
            } else {
                to_non_suffix_num_token!(current_value)
            };

            // Add bitflags constant
            bitflags.push(quote! {
                #(#variant_attrs)*

                #[allow(non_upper_case_globals)]
                const #var_name = #value
            });

            current_value += 1;
        }

        Ok(Self { bitflags, default_value })
    }
}
