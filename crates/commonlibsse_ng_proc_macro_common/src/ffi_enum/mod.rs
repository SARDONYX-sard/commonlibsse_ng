//! generate bitflags from enum
mod attr_args;

use core::str::FromStr as _;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Expr, ItemEnum, Meta};

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
        None => format_ident!("{enum_ident}Flags"),
    };

    // Generate bitflags and match arms
    let DiscriminantData { bitflags, to_enum_arms, from_enum_arms } =
        DiscriminantData::from_item_enum(&item_enum);

    let struct_doc = format!("Bitflags representation of `{enum_ident}` for FFI usage.");
    let to_enum_doc =
        format!("Returns `Some({enum_ident})` if the value is valid, otherwise `None`.");

    let expanded = quote! {
        /// Auto-generated bitflags for FFI compatibility.
        #item_enum

        bitflags::bitflags! {
            #[doc = #struct_doc]
            ///
            /// Provides conversion between the FFI-friendly flag struct and the `enum`.
            ///
            /// # Why bitflags?
            ///
            /// Because C enum is actually a number, and it is dangerous to put enum directly into struct because it may contain numbers other than the variant defined in Rust.
            ///
            /// Use `to_enum`/`from_enum` to inter-convert enums.
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[repr(transparent)]
            #vis struct #flags_ident: #bitflags_type {
                #(#bitflags)*
            }
        }

        impl #flags_ident {
            /// Converts the flag to the corresponding `enum` variant.
            ///
            #[doc = #to_enum_doc]
            #[inline]
            pub const fn to_enum(self) -> Option<#enum_ident> {
                match self {
                    #(#to_enum_arms,)*
                    _ => None,
                }
            }

            /// Creates the flag struct from the `enum`.
            ///
            /// This allows for easy conversion back to the FFI-friendly representation.
            #[inline]
            pub const fn from_enum(e: #enum_ident) -> Self {
                match e {
                    #(#from_enum_arms,)*
                }
            }
        }
    };

    Ok(expanded)
}

/// Struct to store discriminant information and current value
struct DiscriminantData {
    bitflags: Vec<TokenStream>,
    to_enum_arms: Vec<TokenStream>,
    from_enum_arms: Vec<TokenStream>,
}

/// Generates the discriminants for the enum and prepares the corresponding quote items
impl DiscriminantData {
    fn from_item_enum(item_enum: &ItemEnum) -> Self {
        let enum_ident = &item_enum.ident;

        let mut current_value = 0;
        let mut bitflags = Vec::new();
        let mut to_enum_arms = Vec::new();
        let mut from_enum_arms = Vec::new();

        for variant in &item_enum.variants {
            let var_name = &variant.ident;
            let variant_attrs = &variant.attrs;

            let value = if let Some((_, expr)) = &variant.discriminant {
                // Use explicit discriminant
                if let Ok(parsed) = parse_discriminant(expr) {
                    current_value = parsed; // Set the current value
                    TokenStream::from_str(&format!("{current_value}")).unwrap()
                } else {
                    TokenStream::from_str(&format!("{current_value}")).unwrap()
                }
            } else {
                TokenStream::from_str(&format!("{current_value}")).unwrap()
            };

            // Add bitflags constant
            bitflags.push(quote! {
                #(#variant_attrs)*
                const #var_name = #value;
            });

            // Add to_enum match arms
            to_enum_arms.push(quote! {
                Self::#var_name => Some(#enum_ident::#var_name)
            });

            // Add from_enum match arms
            from_enum_arms.push(quote! {
                #enum_ident::#var_name => Self::#var_name
            });

            current_value += 1;
        }

        Self { bitflags, to_enum_arms, from_enum_arms }
    }
}

/// Parses the discriminant value into `i32`.
fn parse_discriminant(expr: &Expr) -> Result<i32, ()> {
    if let Expr::Lit(lit) = expr {
        if let syn::Lit::Int(int) = &lit.lit {
            if let Ok(value) = int.base10_parse::<i32>() {
                return Ok(value);
            }
        }
    }
    Err(())
}

/// Select the appropriate bitflags type based on the `repr` attribute.
fn select_bitflags_type(repr_attr: &Attribute) -> syn::Result<TokenStream> {
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
