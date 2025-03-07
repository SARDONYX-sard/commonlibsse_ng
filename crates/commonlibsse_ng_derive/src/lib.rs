pub(crate) mod args;
mod logger;
pub(crate) mod plugin_entry;

use proc_macro::TokenStream;

/// This procedural macro is used to define the main entry point for an SKSE plugin.
/// It generates the required `SKSEPlugin_Query`, `SKSEPlugin_Version`, and `SKSEPlugin_Load`
/// functions, ensuring proper initialization(`skse::init`) and integration with the SKSE framework.
///
/// # Attributes
///
/// This macro takes optional attributes to configure the plugin's metadata:
///
/// | Attribute        | Description    | Default             | Possible Values                                            | Requires Feature |
/// |------------------|----------------|---------------------|------------------------------------------------------------|------------------|
/// | `plugin_name`    | Plugin name    | `CARGO_PKG_NAME`    | ASCII string (≤ 255 chars)                                 | -                |
/// | `plugin_author`  | Plugin author  | `CARGO_PKG_AUTHORS` | ASCII string (≤ 255 chars)                                 | -                |
/// | `plugin_version` | Plugin version | `CARGO_PKG_VERSION` | [SemVer](https://semver.org/) format (`major.minor.patch`) | -                |
/// | `logger`         | Enable logging | `true`              | `true` / `false`                                           | `tracing`        |
/// | `log_level`      | Log level      | `"trace"`           | `"trace"`, `"debug"`, `"info"`, `"warn"`, `"error"`        | `tracing`        |
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
/// #[commonlibsse_ng::skse_plugin_main]
/// fn plugin_main() {
///     match commonlibsse_ng::skse::api::get_messaging_interface() {
///         Ok(messaging) => {
///             if let Err(err) = messaging.register_skse_listener(|message| {
///                 #[cfg(feature = "tracing")]
///                 tracing::info!("SKSE event: {message:#?}");
///             }) {
///                 #[cfg(feature = "tracing")]
///                 tracing::error!("{err}");
///             };
///         }
///         Err(err) => {
///             #[cfg(feature = "tracing")]
///             tracing::error!("Failed to skse::init: {err}");
///         }
///     }
/// }
/// ```
///
/// This will generate the necessary SKSE functions and execute `plugin_main` in `SKSE_PluginLoad` function.
#[proc_macro_attribute]
pub fn skse_plugin_main(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let args = {
        let attr_args = match darling::ast::NestedMeta::parse_meta_list(attrs.into()) {
            Ok(v) => v,
            Err(e) => {
                return TokenStream::from(darling::Error::from(e).write_errors());
            }
        };

        match <args::MacroArgs as darling::FromMeta>::from_list(&attr_args) {
            Ok(v) => v,
            Err(e) => {
                return TokenStream::from(e.write_errors());
            }
        }
    };
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);

    plugin_entry::generate_plugin_code(args, item_fn)
}
