pub(crate) mod attr_args;
mod logger;
pub(crate) mod plugin_entry;

use proc_macro2::TokenStream;

pub fn gen_skse_plugin_main(attrs: TokenStream, item_fn: syn::ItemFn) -> TokenStream {
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

    plugin_entry::generate_plugin_code(args, item_fn)
}
