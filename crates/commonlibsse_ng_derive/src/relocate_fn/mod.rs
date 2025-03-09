mod attr_args;

use proc_macro::TokenStream;

pub(crate) fn gen_relocate_fn(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let args = {
        let attr_args = match darling::ast::NestedMeta::parse_meta_list(attrs.into()) {
            Ok(v) => v,
            Err(e) => {
                return TokenStream::from(darling::Error::from(e).write_errors());
            }
        };

        match <attr_args::MacroArgs as darling::FromMeta>::from_list(&attr_args) {
            Ok(v) => v,
            Err(e) => {
                return TokenStream::from(e.write_errors());
            }
        }
    };
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    generate_code(args, item_fn)
}

fn generate_code(args: attr_args::MacroArgs, item_fn: syn::ItemFn) -> TokenStream {
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

    let fn_call_args = {
        let fn_args = fn_inputs.iter().filter_map(|arg| match arg {
            syn::FnArg::Typed(pat_type) => Some(&pat_type.pat), // `n: usize` → `n`
            syn::FnArg::Receiver(_) => None,                    // TODO: Support `Self`'s method call
        });
        if variadic.is_some() {
            quote::quote! { #(#fn_args),*, ... }
        } else {
            quote::quote! { #(#fn_args),* }
        }
    };

    let fn_inputs = if let Some(variadic) = variadic {
        let args = fn_inputs.iter();
        quote::quote! { #(#args),*, #variadic }
    } else {
        quote::quote! { #fn_inputs }
    };
    let fn_type = quote::quote! { #constness #asyncness #unsafety #abi fn(#fn_inputs) #fn_output };

    let fn_sig = quote::quote! { #vis #constness #asyncness #unsafety #abi fn #ident(#fn_inputs) #fn_output };
    let stmts = &block.stmts;

    #[cfg(feature = "tracing")]
    let (invalid_ptr_err_log, database_err_log) = (
        quote::quote! { commonlibsse_ng::__private::tracing::error!("The target function of the relocation is null or not aligned and will probably crash soon after this."); },
        quote::quote! { commonlibsse_ng::__private::tracing::error!("Relocation fn critical error: {err}"); },
    );
    #[cfg(not(feature = "tracing"))]
    let (invalid_ptr_err_log, database_err_log) = (quote::quote! {}, quote::quote! {});

    quote::quote! {
        #(#attrs)*
        #fn_sig {
            #(#stmts)*

            let func = commonlibsse_ng::rel::ResolvableAddress::address(
                &commonlibsse_ng::rel::id::RelocationID::new(#se_id, #ae_id, #vr_id),
            );

            match func {
                Ok(func) => {
                    if func.is_null() || !func.is_aligned() {
                        #invalid_ptr_err_log
                    };
                    (unsafe { &*func.cast::<#fn_type>() })(#fn_call_args)
                }
                Err(err) => {
                    #database_err_log
                    panic!("This function has a relocation error: {err}");
                }
            }
        }
    }
    .into()
}
