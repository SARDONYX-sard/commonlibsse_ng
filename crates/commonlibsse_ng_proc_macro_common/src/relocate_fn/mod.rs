mod attr_args;

use crate::fn_args_parser::{FnArgs, create_fn_args};
use proc_macro2::TokenStream;

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
        generics,
        inputs: fn_inputs,
        variadic,
        output: fn_output,
        ..
    } = &sig;

    let FnArgs { call_args, type_args, self_type, cast_self } = create_fn_args(fn_inputs, variadic);

    let fn_type = quote::quote! { #constness #asyncness #unsafety #abi fn #generics (#self_type #type_args) #fn_output };

    let fn_sig = quote::quote! { #vis #constness #asyncness #unsafety #abi fn #ident #generics (#fn_inputs) #fn_output };
    let stmts = &block.stmts;

    #[cfg(feature = "tracing")]
    let database_err_log = quote::quote! { #crate_root_name::__private::tracing::error!("[Critical Error] Failed to resolve address: {err}") };
    #[cfg(not(feature = "tracing"))]
    let database_err_log = quote::quote! {};

    #[cfg(feature = "tracing")]
    let ptr_err_log = quote::quote! {
        #crate_root_name::__private::tracing::error!(
            "Resolved Address, but no permission permission to access this address: {:#?} (se_id: {}, ae_id: {}, vr_id: {})",
            fn_ptr.as_ptr(),
            #se_id,
            #ae_id,
            #vr_id
        );
    };
    #[cfg(not(feature = "tracing"))]
    let ptr_err_log = quote::quote! {};

    quote::quote! {
        #(#attrs)*
        #[allow(clippy::use_self)]
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
                    if !#crate_root_name::rex::win32::is_valid_range(fn_ptr.as_ptr().cast(), core::mem::size_of::<usize>()) {
                        #ptr_err_log;
                    }
                    unsafe { core::mem::transmute::<NonNull<core::ffi::c_void>, SelfSignature>(fn_ptr) }
                });
                FUNC(#cast_self #call_args)
            }
        }
    }
}
