pub(crate) mod attr_args;
mod logger;
pub(crate) mod plugin_entry;

use proc_macro::TokenStream;

pub(crate) fn gen_skse_plugin_main(attrs: TokenStream, item: TokenStream) -> TokenStream {
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

    plugin_entry::generate_plugin_code(args, item_fn)
}
