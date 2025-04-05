use proc_macro2::TokenStream;
use syn::{FnArg, punctuated::Punctuated, token::Comma};

pub struct FnArgs {
   pub call_args: proc_macro2::TokenStream,
   pub type_args: proc_macro2::TokenStream,
   pub self_type: Option<proc_macro2::TokenStream>,
   pub cast_self: Option<proc_macro2::TokenStream>,
}

/// Helper function to create function arguments based on argument lists and variable arguments
pub fn create_fn_args(
    fn_inputs: &Punctuated<FnArg, Comma>,
    variadic: &Option<syn::Variadic>,
) -> FnArgs {
    let mut call_args = vec![];
    let mut type_args = vec![];
    let mut self_type = None;
    let mut cast_self = None;

    for arg in fn_inputs {
        match arg {
            syn::FnArg::Typed(pat_type) => {
                let pat = &pat_type.pat;
                if let syn::Pat::Ident(ident) = &**pat {
                    if ident.ident == "self" {
                        let syn::PatIdent { by_ref, mutability, .. } = ident;
                        let (self_type_, cast_self_) =
                            create_self_and_cast(by_ref.is_some(), mutability.is_some());
                        self_type = Some(self_type_);
                        cast_self = Some(cast_self_);
                        continue;
                    }
                }

                let ident = &pat_type.pat;
                let ty = &pat_type.ty;
                call_args.push(quote::quote! { #ident  });
                type_args.push(quote::quote! { #ty });
            }
            syn::FnArg::Receiver(receiver) => {
                let reference = receiver.reference.as_ref().map(|(and, _lifetime)| and);
                let mutability = &receiver.mutability;

                let (self_type_, cast_self_) =
                    create_self_and_cast(reference.is_some(), mutability.is_some());
                self_type = Some(self_type_);
                cast_self = Some(cast_self_);
            }
        }
    }

    if variadic.is_some() {
        call_args.push(quote::quote! { ... });
    }

    FnArgs {
        call_args: quote::quote! { #(#call_args),* },
        type_args: quote::quote! { #(#type_args),* },
        self_type,
        cast_self,
    }
}

fn create_self_and_cast(by_ref: bool, mutability: bool) -> (TokenStream, TokenStream) {
    match (by_ref, mutability) {
        (true, true) => (quote::quote! { *mut (), }, quote::quote! { (self as *mut Self).cast(), }),
        (true, false) => {
            (quote::quote! { *const (), }, quote::quote! { (self as *const Self).cast(), })
        }
        (false, _) => (
            quote::quote! { *mut (), },
            quote::quote! { Box::into_raw(Box::new(self) as *mut ()), }, // NOTE: mem leak
        ),
    }
}
