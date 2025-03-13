use super::attr_args::MacroArgs;
use super::logger::LoggerTokenStream;
use quote::quote;

struct PluginMetadata {
    name: proc_macro2::TokenStream,
    author: proc_macro2::TokenStream,
    version: proc_macro2::TokenStream,
}

impl PluginMetadata {
    fn from_args(args: &MacroArgs) -> Self {
        Self {
            name: args
                .plugin_name
                .as_deref()
                .map_or_else(|| quote! { env!("CARGO_PKG_NAME") }, |name| quote! { #name }),
            author: args
                .plugin_author
                .as_deref()
                .map_or_else(|| quote! { env!("CARGO_PKG_AUTHORS") }, |author| quote! { #author }),
            version: args.plugin_version.as_deref().map_or_else(
                || quote! { env!("CARGO_PKG_VERSION") },
                |version| quote! { #version },
            ),
        }
    }
}

fn generate_main_code(enable_logger: bool, item_fn: syn::ItemFn) -> proc_macro2::TokenStream {
    let fn_stmts = &item_fn.block.stmts;
    let ret_ty = &item_fn.sig.output;
    if enable_logger {
        quote! {
            if let Err(err) = std::panic::catch_unwind(|| #ret_ty {
                commonlibsse_ng::skse::init(skse);
                #(#fn_stmts)*
            }) {
                commonlibsse_ng::__private::tracing::error!("{err:?}");
            }
        }
    } else {
        quote! {
            let _ = std::panic::catch_unwind(|| #ret_ty {
                commonlibsse_ng::skse::init(skse);
                #(#fn_stmts)*
            });
        }
    }
}

pub(crate) fn generate_plugin_code(args: MacroArgs, item_fn: syn::ItemFn) -> proc_macro2::TokenStream {
    #[cfg(feature = "tracing")]
    let main_code = generate_main_code(true, item_fn);
    #[cfg(not(feature = "tracing"))]
    let main_code = generate_main_code(false, item_fn);

    let PluginMetadata { name, author, version } = PluginMetadata::from_args(&args);
    let LoggerTokenStream { init_logger, is_editor_log } = {
        #[cfg(feature = "tracing")]
        let tokens = super::logger::gen_logger_code(
            args.logger,
            args.plugin_name.as_deref(),
            args.log_level,
        );
        #[cfg(not(feature = "tracing"))]
        let tokens = LoggerTokenStream::default();
        tokens
    };

    let expanded = quote! {
        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        pub extern "C" fn SKSEPlugin_Query(
            skse: &commonlibsse_ng::skse::impls::stab::SKSEInterface,
            info: &mut commonlibsse_ng::skse::impls::stab::PluginInfo,
        ) -> bool {
            use commonlibsse_ng::skse::impls::stab::PluginInfo;

            const PKG_VERSION: commonlibsse_ng::rel::version::Version = commonlibsse_ng::rel::version::Version::from_str_const(#version);
            *info = PluginInfo {
                infoVersion: PluginInfo::VERSION,
                name: commonlibsse_ng::skse::interfaces::new_cstr(concat!(#name, "\0")).as_ptr(),
                version: PKG_VERSION.pack(),
            };

            if commonlibsse_ng::skse::interfaces::query::QueryInterface::is_editor(skse) {
                #is_editor_log
                return false;
            }

            true
        }

        #[unsafe(no_mangle)]
        #[allow(non_upper_case_globals)]
        pub static SKSEPlugin_Version: commonlibsse_ng::skse::interfaces::PluginVersionData = {
            use commonlibsse_ng::skse::interfaces::PluginVersionData;
            use commonlibsse_ng::skse::interfaces::to_fixed_str;

            const PKG_VERSION: commonlibsse_ng::rel::version::Version = commonlibsse_ng::rel::version::Version::from_str_const(#version);
            let mut compatible_versions = [0; 16];
            compatible_versions[0] = commonlibsse_ng::skse::version::RUNTIME_SSE_LATEST.pack();

            PluginVersionData {
                data_version: PluginVersionData::VERSION,
                plugin_version: PKG_VERSION.pack(),
                plugin_name: to_fixed_str(#name),
                author: to_fixed_str(#author),
                support_email: [0; 252],
                version_independence_ex: PluginVersionData::VERSION_INDEPENDENT_ADDRESS_LIBRARY_POST_AE,
                version_independence: PluginVersionData::VERSION_INDEPENDENT_EX_NO_STRUCT_USE,
                compatible_versions,
                xse_minimum: 0,
            }
        };

        #[unsafe(no_mangle)]
        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        pub extern "C" fn SKSEPlugin_Load(skse: &commonlibsse_ng::skse::interfaces::load::LoadInterface) -> bool {
            #init_logger
            #main_code

            true
        }
    };

    expanded
}
