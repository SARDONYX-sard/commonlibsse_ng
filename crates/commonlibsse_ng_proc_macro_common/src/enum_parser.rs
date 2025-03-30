use proc_macro2::TokenStream;
use quote::ToTokens as _;
use quote::quote;
use syn::{Attribute, Expr, Lit, Meta};

/// Parses the discriminant value into integer
pub(crate) fn parse_discriminant(expr: &Expr) -> syn::Result<i64> {
    match expr {
        Expr::Lit(lit) => {
            match &lit.lit {
                Lit::Int(int) => int.base10_parse::<i64>(),
                Lit::Byte(byte) => Ok(byte.value() as i64), // e.g. `b'l'`
                _ => Err(syn::Error::new_spanned(
                    lit,
                    format!("Failed to parse as integer: {}", lit.to_token_stream()),
                )),
            }
        }
        Expr::Binary(binary) => {
            let left = parse_discriminant(&binary.left)?;
            let right = parse_discriminant(&binary.right)?;

            match binary.op {
                syn::BinOp::Add(_) => Ok(left + right), // Addition
                syn::BinOp::Sub(_) => Ok(left - right), // Subtraction
                syn::BinOp::Mul(_) => Ok(left * right), // Multiplication
                syn::BinOp::Div(_) => {
                    if right == 0 {
                        Err(syn::Error::new_spanned(
                            binary,
                            "Division by zero is not allowed".to_string(),
                        ))
                    } else {
                        Ok(left / right) // Division
                    }
                }
                syn::BinOp::Rem(_) => {
                    if right == 0 {
                        Err(syn::Error::new_spanned(
                            binary,
                            "Modulo by zero is not allowed".to_string(),
                        ))
                    } else {
                        Ok(left % right) // Modulus (remainder)
                    }
                }
                syn::BinOp::Shl(_) => Ok(left << right), // Left shift
                syn::BinOp::Shr(_) => Ok(left >> right), // Right shift
                syn::BinOp::BitAnd(_) => Ok(left & right), // Bitwise AND
                syn::BinOp::BitOr(_) => Ok(left | right), // Bitwise OR
                syn::BinOp::BitXor(_) => Ok(left ^ right), // Bitwise XOR
                _ => Err(syn::Error::new_spanned(
                    binary,
                    format!("Unsupported operator: {}", binary.to_token_stream()),
                )),
            }
        }
        unknown => Err(syn::Error::new_spanned(
            unknown,
            format!("Unsupported expression type: {}", unknown.to_token_stream()),
        )),
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

pub(crate) fn filter_repr_default_attr(attrs: &[Attribute]) -> (Vec<&Attribute>, Option<&Attribute>) {
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

            !(is_repr || default_derive_attr.is_some()) // Exclude `repr` & `derive(Default)`
        })
        .collect();

    (v, repr_attr)
}
