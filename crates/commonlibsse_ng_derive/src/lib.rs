//! `commonlibsse_ng` macro to automatically generate commonly used code patterns for crate use.
//!

mod relocate_fn;
mod skse_plugin_main;

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
/// And need `tracing` feature.
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
    skse_plugin_main::gen_skse_plugin_main(attrs, item)
}

/// `relocate_fn` is a procedural macro used to generate a relocated function in Rust.
/// It allows you to specify function relocation IDs (`se_id`, `ae_id`, and `vr_id`) to relocate a function at runtime.
/// This macro generates code that attempts to resolve a function's address based on the provided IDs,
/// and calls the relocated function if the address is valid.
///
/// # Attributes
/// The macro accepts the following arguments:
///
/// | Attribute | Description                                        |
/// |-----------|----------------------------------------------------|
/// | `se_id`   | The ID of the Skyrim Special Edition (mandatory)       |
/// | `ae_id`   | The ID of the Skyrim Anniversary Edition (mandatory) |
/// | `vr_id`   | The ID of the Skyrim VR (optional, defaults to `se_id` if not provided) |
///
/// The macro generates code that will dynamically resolve the address of the function using these IDs.
/// If the resolution is successful and the address is valid (non-null and aligned),
/// the relocated function will be called with the same arguments as the original function.
/// Otherwise, an error will be logged(`tracing` feature is required), and the program will panic.
///
/// # Example
/// ```rust:no_compile
/// #[commonlibsse_ng::relocate_fn(se_id = 1, ae_id = 2, vr_id = 3)]
/// fn my_function(arg1: usize, arg2: usize) -> bool {
///     tracing::info("arg1 = {arg1}, arg2 = {arg2}"); // We can sandwich the process before that.
///
///     // macro is expanded from here.
///     // 1. id to address
///     // 2. Execute function with the argument as it is.
/// }
/// ```
///
/// This will generate a function that attempts to resolve its address using the provided IDs and
/// call it safely with `arg1` and `arg2` passed as arguments.
///
/// # Notes
/// - The relocation IDs are used for dynamically finding the target function address.
/// - If no `vr_id` is provided, the `se_id` is used as the default value.
///
/// # Panics
/// - The macro generates code that checks the validity of the resolved address, logging an error and panicking if invalid.
#[proc_macro_attribute]
pub fn relocate_fn(attrs: TokenStream, item: TokenStream) -> TokenStream {
    relocate_fn::gen_relocate_fn(attrs, item)
}
