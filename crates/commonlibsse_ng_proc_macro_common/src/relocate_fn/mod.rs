mod attr_args;

use proc_macro2::TokenStream;
use syn::{FnArg, punctuated::Punctuated, token::Comma};

pub fn gen_relocate_fn(
    attrs: TokenStream,
    item_fn: syn::ItemFn,
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
    generate_code(args, item_fn, crate_root_name)
}

fn generate_code(
    args: attr_args::MacroArgs,
    item_fn: syn::ItemFn,
    crate_root_name: TokenStream,
) -> TokenStream {
    let attr_args::MacroArgs { se_id, ae_id, vr_id } = args;
    let vr_id = vr_id.unwrap_or(se_id);

    let syn::ItemFn { attrs, vis, sig, block } = &item_fn;
    let syn::Signature {
        constness,
        asyncness,
        unsafety,
        abi,
        ident,
        inputs: fn_inputs,
        variadic,
        output: fn_output,
        ..
    } = &sig;

    let FnArgs { call_args, type_args, self_type, cast_self } = create_fn_args(fn_inputs, variadic);

    let fn_type =
        quote::quote! { #constness #asyncness #unsafety #abi fn(#self_type #type_args) #fn_output };

    let fn_sig = quote::quote! { #vis #constness #asyncness #unsafety #abi fn #ident(#fn_inputs) #fn_output };
    let stmts = &block.stmts;

    #[cfg(feature = "tracing")]
    let database_err_log = quote::quote! { #crate_root_name::__private::tracing::error!("[Critical Error] Failed to resolve address: {err}") };
    #[cfg(not(feature = "tracing"))]
    let database_err_log = quote::quote! {};

    quote::quote! {
        #(#attrs)*
        #fn_sig {
            #(#stmts)*

            /// Function signature for self.
            /// `self` is automatically `this: *const ()`, `this: *mut ()`, etc..
            ///
            /// This is created because Rust does not have a function equivalent to `decltype(T)` in C++.
            type SelfSignature = #fn_type;

            {
                static FUNC: std::sync::LazyLock<SelfSignature> = std::sync::LazyLock::new(|| {
                    use core::ptr::NonNull;

                    use #crate_root_name::rel::id::RelocationID;
                    use #crate_root_name::rel::ResolvableAddress as _;

                    let fn_ptr = RelocationID::new(#se_id, #ae_id, #vr_id).address().unwrap_or_else(|err| {
                        #database_err_log;
                        panic!("Failed to resolve address: {err}");
                    });
                    unsafe { core::mem::transmute::<NonNull<core::ffi::c_void>, SelfSignature>(fn_ptr) }
                });
                FUNC(#cast_self #call_args)
            }
        }
    }
}

struct FnArgs {
    call_args: proc_macro2::TokenStream,
    type_args: proc_macro2::TokenStream,
    self_type: Option<proc_macro2::TokenStream>,
    cast_self: Option<proc_macro2::TokenStream>,
}

/// Helper function to create function arguments based on argument lists and variable arguments
fn create_fn_args(
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
