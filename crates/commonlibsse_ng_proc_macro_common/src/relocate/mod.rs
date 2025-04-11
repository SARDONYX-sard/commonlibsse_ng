mod attr_args;

use crate::fn_args_parser::{FnArgs, create_fn_args};
use core::str::FromStr;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

pub fn gen_relocate(
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
    generate_code(args, item_fn, crate_root_name).unwrap_or_else(syn::Error::into_compile_error)
}

fn generate_code(
    args: attr_args::MacroArgs,
    item_fn: syn::ItemFn,
    crate_root_name: TokenStream,
) -> syn::Result<TokenStream> {
    let attr_args::MacroArgs { cast_as, default, deref_once, id } = args;
    let attr_args::RelocationId { se: se_id, ae: ae_id, vr: vr_id } = id;
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

    let FnArgs { type_args, self_type, .. } = create_fn_args(fn_inputs, variadic);

    let fn_type = quote! { #constness #asyncness #unsafety #abi fn #generics (#self_type #type_args) #fn_output };

    let fn_sig = quote! { #vis #constness #asyncness #unsafety #abi fn #ident #generics (#fn_inputs) #fn_output };
    let closure = extract_closure_expr(block)?;
    let closure_arg = &closure.inputs;
    let body = &closure.body;

    #[cfg(feature = "tracing")]
    let database_err_log = quote! { #crate_root_name::__private::tracing::error!("[Critical Error] Failed to resolve address: {err}") };
    #[cfg(not(feature = "tracing"))]
    let database_err_log = quote! {};

    let cast_type: Type = syn::parse2(TokenStream::from_str(&cast_as)?)?; // from cast_as string
    let (deref_type_alias, deref_code) = if deref_once.is_some_and(|b| b) {
        let deref_type = peel_pointer(&cast_type); // returns Option<Type>
        let deref_type = deref_type.as_ref().map(|inner| {
            quote! {
                /// Type dereferenced once(by `read_unaligned`) from `cast_as`.
                type DerefType = #inner
            }
        });

        (
            deref_type,
            quote! {
                let ptr: AsType = core::mem::transmute(v.as_ptr());
                ptr.read_unaligned()
            },
        )
    } else {
        (
            None,
            quote! {
                let ptr: AsType = core::mem::transmute(v.as_ptr());
                ptr
            },
        )
    };

    let default = TokenStream::from_str(&default)?;

    Ok(quote! {
        #(#attrs)*
        #[allow(clippy::unnecessary_map_or)]
        #[allow(clippy::use_self)]
        #fn_sig {
            /// Function signature for self.
            /// `self` is automatically `this: *const ()`, `this: *mut ()`, etc..
            ///
            /// This is created because Rust does not have a function equivalent to `decltype(T)` in C++.
            type SelfSignature = #fn_type;
            /// Casted type.
            type AsType = #cast_type;
            #deref_type_alias;

            {
                static ADDRESS: #crate_root_name::__private::OnceCell<#crate_root_name::__private::Unique<::core::ffi::c_void>> =
                    #crate_root_name::__private::OnceCell::new();
                ADDRESS
                    .get_or_try_init(|| {
                        use #crate_root_name::__private::Unique;
                        use #crate_root_name::rel::id::RelocationID;
                        use #crate_root_name::rel::ResolvableAddress as _;

                        let address = match RelocationID::new(#se_id, #ae_id, #vr_id).address() {
                            Ok(addr) => addr,
                            Err(err) => {
                                #database_err_log;
                                return Err(err);
                            }
                        };

                        unsafe { Ok(Unique::new_unchecked(address.as_ptr())) }
                    })
                    .ok()
                    .map(|v| unsafe { #deref_code })
                    .map_or(#default, |#closure_arg| { #body }) // intended stmts: `|ptr| unsafe { ptr.read_unaligned() }`
            }
        }
    })
}

fn extract_closure_expr(block: &syn::Block) -> syn::Result<&syn::ExprClosure> {
    use syn::{Expr, Stmt};

    if block.stmts.len() != 1 {
        return Err(syn::Error::new_spanned(
            block,
            "expected a single closure expression inside the function body",
        ));
    }

    match &block.stmts[0] {
        Stmt::Expr(Expr::Closure(closure), _) => Ok(closure),
        Stmt::Expr(expr, _) => {
            Err(syn::Error::new_spanned(expr, "expected a closure expression like `|x| x`"))
        }
        stmt => Err(syn::Error::new_spanned(stmt, "expected an expression statement (closure)")),
    }
}

fn peel_pointer(ty: &Type) -> Option<Type> {
    if let Type::Ptr(syn::TypePtr { elem, .. }) = ty { Some(*elem.clone()) } else { None }
}
