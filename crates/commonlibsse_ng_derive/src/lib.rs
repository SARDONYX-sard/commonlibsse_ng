mod gen_logger;

use crate::gen_logger::{GeneratedCode, LogLevel, gen_logger_code};
use darling::{Error, FromMeta, ast::NestedMeta};
use proc_macro::TokenStream;
use quote::quote;
use std::{env, ffi::CString};
use syn::{ItemFn, parse_macro_input};

#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(default = "ret_true")]
    logger: bool,
    #[darling(default)]
    log_level: LogLevel,
    plugin_name: Option<String>,
    plugin_author: Option<String>,
    plugin_version: Option<String>,
}

fn ret_true() -> bool {
    true
}

/// This procedural macro is used to define the main entry point for an SKSE plugin.
/// It generates the required `SKSEPlugin_Query`, `SKSEPlugin_Version`, and `SKSEPlugin_Load`
/// functions, ensuring proper initialization and integration with the SKSE framework.
///
/// # Attributes
///
/// This macro takes optional attributes to configure the plugin's metadata:
///
/// | Attribute      | Description | Default |
/// |---------------|------------|---------|
/// | `plugin_name` | Sets the plugin's name.(ASCII string of 255 or less) | `CARGO_PKG_NAME` |
/// | `plugin_author` | Sets the plugin author's name.(ASCII string of 255 or less) | `CARGO_PKG_AUTHORS` |
/// | `plugin_version` | Specifies the plugin version. | `CARGO_PKG_VERSION` |
/// | `logger` | Enables or disables the logging system (`true` to enable, `false` to disable`). | `true` |
/// | `log_level`   | Specifies the log level for the plugin (e.g. `"trace"`, `"debug"`, `"info"`, `"warn"`, `"error"`). | `trace` |
///
/// # Generated no mangle Global Symbols
///
/// - `SKSEPlugin_Query`: Provides basic plugin information and ensures compatibility.
/// - `SKSEPlugin_Version`: Defines the plugin's version data and supported runtime versions.
/// - `SKSEPlugin_Load`: Initializes the plugin and executes the provided function body.
///
/// # Logging
///
/// If `logger = true`, a logger is initialized to output logs to a file named `{plugin_name}.log`.
/// If the logger fails to initialize, an error message is displayed via a Windows message box,
/// and the process terminates.
///
/// And need `tracing` feature. e.g. `commonlibsse_ng = { version = "*", features = ["tracing"] }`
///
/// # Example
///
/// ```rust:no_compile
/// #[cfg_attr(feature = "tracing", commonlibsse_ng::skse_plugin_main)]
/// #[cfg_attr(not(feature = "tracing"), commonlibsse_ng::skse_plugin_main(logger = false))]
/// fn plugin_entry() {
///     if let Some(messaging) = commonlibsse_ng::skse::api::get_messaging_interface() {
///         messaging.register_listener(|message| {
///             #[cfg(feature = "tracing")]
///             tracing::info!("SKSE event: {message:?}");
///         });
///     }
///     #[cfg(feature = "tracing")]
///     tracing::info!("MyPlugin has been loaded!");
/// }
/// ```
///
/// This will generate the necessary SKSE functions and execute `plugin_entry`.
#[proc_macro_attribute]
pub fn skse_plugin_main(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(attrs.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let input = parse_macro_input!(item as ItemFn);

    let plugin_name = args.plugin_name.as_deref().unwrap_or(env!("CARGO_PKG_NAME"));
    let plugin_author = args.plugin_author.unwrap_or_else(|| env!("CARGO_PKG_AUTHORS").to_string());
    let plugin_version = args.plugin_version.as_deref().unwrap_or(env!("CARGO_PKG_VERSION"));

    let mut plugin_log_name = plugin_name.to_string();
    plugin_log_name.push_str(".log");

    let plugin_name_c_str =
        CString::new(args.plugin_name.as_deref().unwrap_or(env!("CARGO_PKG_NAME")))
            .expect("Expected plugin name is valid ascii");

    let fn_stmts = &input.block.stmts;
    let ret_ty = &input.sig.output;

    let GeneratedCode { init_logger, is_editor_log } =
        gen_logger_code(args.logger, &plugin_log_name, args.log_level);

    let main_code = if args.logger {
        quote! {
                if let Err(err) = std::panic::catch_unwind(|| #ret_ty {
                    commonlibsse_ng::skse::init(skse);
                    #(#fn_stmts)*
                }) {
                    tracing::error!("{err:?}");
                }
        }
    } else {
        quote! {
            let _ = std::panic::catch_unwind(|| #ret_ty {
                commonlibsse_ng::skse::init(skse);
                #(#fn_stmts)*
            });
        }
    };

    let expanded = quote! {
        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        pub extern "C" fn SKSEPlugin_Query(
            skse: &commonlibsse_ng::skse::impls::stab::SKSEInterface,
            info: &mut commonlibsse_ng::skse::impls::stab::PluginInfo,
        ) -> bool {
            use commonlibsse_ng::skse::impls::stab::PluginInfo;

            const PKG_VERSION: commonlibsse_ng::rel::version::Version = commonlibsse_ng::rel::version::Version::from_str_const(#plugin_version);
            *info = PluginInfo {
                infoVersion: PluginInfo::VERSION,
                name: #plugin_name_c_str.as_ptr(),
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

            const PKG_VERSION: commonlibsse_ng::rel::version::Version = commonlibsse_ng::rel::version::Version::from_str_const(#plugin_version);
            let mut compatible_versions = [0; 16];
            compatible_versions[0] = commonlibsse_ng::skse::version::RUNTIME_SSE_LATEST.pack();

            PluginVersionData {
                data_version: PKG_VERSION.major() as u32,
                plugin_version: PKG_VERSION.pack(),
                plugin_name: to_fixed_str(#plugin_name),
                author: to_fixed_str(#plugin_author),
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

    TokenStream::from(expanded)
}
