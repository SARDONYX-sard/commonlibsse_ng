use quote::ToTokens as _;
use syn::{Expr, Lit};

/// Parses the discriminant value into integer
pub(crate) fn parse_discriminant(expr: &Expr) -> syn::Result<i64> {
    match expr {
        Expr::Lit(lit) => {
            if let Lit::Int(int) = &lit.lit {
                if let Ok(value) = int.base10_parse::<i64>() {
                    return Ok(value);
                }
            }
            Err(syn::Error::new_spanned(
                lit,
                format!("Failed to parse as integer: {}", lit.to_token_stream()),
            ))
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
