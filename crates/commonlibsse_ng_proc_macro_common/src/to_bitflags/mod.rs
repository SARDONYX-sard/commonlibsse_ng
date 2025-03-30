//! generate bitflags from enum
use core::str::FromStr as _;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, ItemEnum, Meta};

use crate::{discriminant_parser::parse_discriminant, ffi_enum::attr_args};

pub fn to_bitflags(attrs: TokenStream, item_enum: ItemEnum) -> TokenStream {
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
    to_bitflags_inner(args, item_enum).unwrap_or_else(syn::Error::into_compile_error)
}

fn to_bitflags_inner(_args: attr_args::MacroArgs, item_enum: ItemEnum) -> syn::Result<TokenStream> {
    let enum_ident = &item_enum.ident;
    let vis = &item_enum.vis;

    let (other_attr, repr_attr) = filter_repr_attr(&item_enum.attrs);
    let bitflags_type = match repr_attr {
        Some(repr_attr) => select_bitflags_type(repr_attr)?,
        None => quote! { usize },
    };

    // Generate bitflags and match arms
    let DiscriminantData { bitflags, default_value, .. } =
        DiscriminantData::from_item_enum(&item_enum)?;
    let docs = format!("- size type: [`{bitflags_type}`]");

    let expanded = quote! {
        bitflags::bitflags! {
            #(#other_attr)*
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

/// Select the appropriate bitflags type based on the `repr` attribute.
pub(crate) fn select_bitflags_type(repr_attr: &Attribute) -> syn::Result<TokenStream> {
    let mut repr = quote! { usize };
    if let Meta::List(meta) = &repr_attr.meta {
        meta.parse_nested_meta(|nested_meta| {
            let path = &nested_meta.path;

            if path.is_ident("u32") {
                repr = quote! { u32 };
            } else if path.is_ident("i32") {
                repr = quote! { i32 };
            } else if path.is_ident("u64") {
                repr = quote! { u64 };
            } else if path.is_ident("i64") {
                repr = quote! { i64 };
            } else if path.is_ident("u8") {
                repr = quote! { u8 };
            } else if path.is_ident("i8") {
                repr = quote! { i8 };
            } else if path.is_ident("u16") {
                repr = quote! { u16 };
            } else if path.is_ident("i16") {
                repr = quote! { i16 };
            } else if path.is_ident("usize") {
                repr = quote! { usize };
            } else if path.is_ident("isize") {
                repr = quote! { isize };
            } else if path.is_ident("C") {
                repr = quote! { i32 }; // c_int
            } else {
                return Err(syn::Error::new_spanned(
                    path,
                    format!(
                        "Unsupported repr type: {}",
                        path.get_ident().map(|i| i.to_string()).unwrap_or_default()
                    ),
                ));
            }

            Ok(())
        })?;
    }

    Ok(repr)
}

pub(crate) fn filter_default_attr(attrs: &[Attribute]) -> (Vec<&Attribute>, bool) {
    let mut not_default = false;
    let v = attrs
        .iter()
        .filter(|attr| {
            // Remove `#[default]`
            not_default =
                if let Meta::Path(path) = &attr.meta { !path.is_ident("default") } else { true };
            not_default
        })
        .collect();

    (v, not_default)
}

pub(crate) fn filter_repr_attr(attrs: &[Attribute]) -> (Vec<&Attribute>, Option<&Attribute>) {
    let mut default_derive_attr = None;
    let mut repr_attr = None;
    let v = attrs
        .iter()
        .filter(|attr| {
            let is_repr = attr.meta.path().is_ident("repr");
            if is_repr {
                repr_attr = Some(*attr);
            }
            let is_derive = attr.meta.path().is_ident("derive");

            if is_derive {
                if let Meta::List(meta) = &attr.meta {
                    let _ = meta.parse_nested_meta(|nested_meta| {
                        let path = &nested_meta.path;

                        if path.is_ident("Default") {
                            default_derive_attr = Some(*attr);
                        }
                        Ok(())
                    });
                }
            }

            !(is_repr || default_derive_attr.is_some()) // `repr` と `derive(Default)` を除外
        })
        .collect();

    (v, repr_attr)
}
