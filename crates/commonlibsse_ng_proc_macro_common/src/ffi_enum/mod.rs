//! generate bitflags from enum
pub(crate) mod attr_args;

use core::str::FromStr as _;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ItemEnum;

use crate::enum_parser::{filter_default_attr, parse_discriminant, select_bitflags_type};

pub fn ffi_enum(attrs: TokenStream, item_enum: ItemEnum) -> TokenStream {
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
    ffi_enum_(args, item_enum).unwrap_or_else(syn::Error::into_compile_error)
}

fn ffi_enum_(args: attr_args::MacroArgs, item_enum: ItemEnum) -> syn::Result<TokenStream> {
    let enum_ident = &item_enum.ident;
    let vis = &item_enum.vis;

    let repr_attr = item_enum.attrs.iter().find(|attr| attr.meta.path().is_ident("repr"));
    let bitflags_type = match repr_attr {
        Some(repr_attr) => select_bitflags_type(repr_attr)?,
        None => quote! { usize },
    };

    // Generate flag struct name: MyEnum -> MyEnumFlags
    let flags_ident = match args.flag_name {
        Some(name) => format_ident!("{name}"),
        None => format_ident!("{enum_ident}_CEnum"),
    };

    // Generate bitflags and match arms
    let DiscriminantData { bitflags, to_enum_arms, from_enum_arms, default_value } =
        DiscriminantData::from_item_enum(&item_enum);
    let discriminant_count = to_enum_arms.len();
    let discriminant_count_doc = format!("Returns `{discriminant_count}`");

    let struct_doc = format!("Auto-generated FFI type for `{enum_ident}`.");
    let to_enum_doc =
        format!("Returns `Some({enum_ident})` if the value is valid, otherwise `None`.");

    let expanded = quote! {
        #item_enum

        #[doc = #struct_doc]
        /// # When use this?
        /// C's enum is really just a number and there is no guarantee that it will fit within the enum.
        /// Therefore, it is used for the following cases that **cannot be controlled** by Rust.
        /// - C++ members
        /// - Function return values.
        ///
        /// # When not to use it?
        /// Mainly those that can be controlled for safety on the Rust side.
        /// - Function Arguments
        ///
        /// # Convenient methods
        /// - `to_enum`/`from_enum`: To inter-convert enums.
        /// - `count`: Returns the number of defined discriminants.
        ///
        /// # Memory Layout
        /// It will always have `#[repr(transparent)]`.
        /// In other words, it is equivalent to the size specified in `repr`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        #vis struct #flags_ident(#vis #bitflags_type);

        impl Default for #flags_ident {
            #[inline]
            fn default() -> Self {
                Self(#default_value)
            }
        }

        impl #flags_ident {
            #(#bitflags;)*

            /// Converts to the corresponding `enum` variant.
            ///
            #[doc = #to_enum_doc]
            #[inline]
            pub const fn to_enum(self) -> Option<#enum_ident> {
                match self.0 {
                    #(#to_enum_arms,)*
                    _ => None,
                }
            }

            /// Creates the struct from the `enum`.
            ///
            /// This allows for easy conversion back to the FFI-friendly representation.
            #[inline]
            pub const fn from_enum(e: #enum_ident) -> Self {
                match e {
                    #(#from_enum_arms,)*
                }
            }

            /// Number of discriminant in enum.
            ///
            #[doc = #discriminant_count_doc]
            #[inline]
            pub const fn count() -> usize {
                #discriminant_count
            }
        }

        impl TryFrom<#flags_ident> for #enum_ident {
            type Error = &'static str;

            #[inline]
            fn try_from(value: #flags_ident) -> Result<Self, Self::Error> {
                #flags_ident::to_enum(value).ok_or("Couldn't convert value to enum.")
            }
        }
        impl From<#enum_ident> for #flags_ident {
            #[inline]
            fn from(value: #enum_ident) -> Self {
                Self::from_enum(value)
            }
        }
    };

    Ok(expanded)
}

/// Struct to store discriminant information and current value
pub(crate) struct DiscriminantData {
    /// .e.g `pub const #var_name: Self = Self(#value)`
    pub(crate) bitflags: Vec<TokenStream>,
    pub(crate) to_enum_arms: Vec<TokenStream>,
    pub(crate) from_enum_arms: Vec<TokenStream>,
    pub(crate) default_value: TokenStream,
}

macro_rules! to_non_suffix_num_token {
    ($value:expr) => {
        TokenStream::from_str(&format!("{}", $value)).unwrap()
    };
}

/// Generates the discriminants for the enum and prepares the corresponding quote items
impl DiscriminantData {
    pub(crate) fn from_item_enum(item_enum: &ItemEnum) -> Self {
        let enum_ident = &item_enum.ident;

        let mut current_value = 0;
        let mut bitflags = Vec::new();
        let mut to_enum_arms = Vec::new();
        let mut from_enum_arms = Vec::new();
        let mut default_value = quote! { 0 };

        for variant in &item_enum.variants {
            let var_name = &variant.ident;
            let (variant_attrs, found_default) = filter_default_attr(&variant.attrs);

            // If use explicit discriminant, change from current discriminant.
            if let Some((_, expr)) = &variant.discriminant {
                if let Ok(parsed) = parse_discriminant(expr) {
                    current_value = parsed; // Set the current value
                    if found_default {
                        default_value = to_non_suffix_num_token!(current_value);
                    }
                };
            };
            let value = to_non_suffix_num_token!(current_value);

            // Add bitflags constant
            bitflags.push(quote! {
                #(#variant_attrs)*
                #[allow(non_upper_case_globals)]
                pub const #var_name: Self = Self(#value)
            });

            // Add to_enum match arms
            to_enum_arms.push(quote! {
                #value => Some(#enum_ident::#var_name)
            });

            // Add from_enum match arms
            from_enum_arms.push(quote! {
                #enum_ident::#var_name => Self::#var_name
            });

            current_value += 1;
        }

        Self { bitflags, to_enum_arms, from_enum_arms, default_value }
    }
}
