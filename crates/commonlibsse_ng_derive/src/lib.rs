//! `commonlibsse_ng` macro to automatically generate commonly used code patterns for crate use.
//!

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
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    commonlibsse_ng_proc_macro_common::skse_plugin_main::gen_skse_plugin_main(attrs.into(), item_fn)
        .into()
}

/// Relocates a address using Skyrim runtime-specific relocation IDs.
///
/// The `#[relocate]` attribute macro enables dynamic resolution of a pointer using relocation
/// IDs for Skyrim Special Edition (SE), Anniversary Edition (AE), and optionally VR. It injects code
/// to resolve the address at runtime and execute user-provided logic through a closure, passing the
/// resolved pointer as an argument.
///
/// # Attributes
///
/// | Attribute    | Type      | Required | Description                                                               |
/// |--------------|-----------|----------|---------------------------------------------------------------------------|
/// | `cast_as`    | `&str`    | Yes      | The type to cast the resolved pointer to (e.g., `"bool"`).                |
/// | `default`    | `&str`    | Yes      | The fallback value returned if resolution fails (e.g., `"false"`).        |
/// | `id.se`      | `u64`     | Yes      | Relocation ID for Skyrim Special Edition.                                 |
/// | `id.ae`      | `u64`     | Yes      | Relocation ID for Skyrim Anniversary Edition.                             |
/// | `id.vr`      | `u64`     | No       | Relocation ID for Skyrim VR. Defaults to `se` if omitted.                 |
///
/// # Function Body
///
/// The body must be a single closure of the form:
///
/// ```rust
/// |ptr: AsType| { ... }
/// ```
///
/// where `AsType` is the dereferenced value of the casted pointer.
/// This closure will only be called if the relocation address is resolved successfully.
///
/// If resolution fails, the `default` value will be returned instead (after parsing the literal).
///
/// # Example
///
/// ```rust
/// #[commonlibsse_ng::relocate(
///     cast_as = "bool",
///     default = "false",
///     id(se = 517711, ae = 404238)
/// )]
/// pub fn is_god_mode() -> bool {
///     |ptr: bool| ptr
/// }
/// ```
///
/// In this case, the macro will:
/// - Resolve the relocation address by using the given `se`/`ae` ID.
/// - Cast it to `*mut bool`, dereference it, and pass the value into the closure.
/// - If resolution fails, return `false`.
///
/// # Notes
///
/// - The macro requires `once_cell`, `Unique`, and `rel::ResolvableAddress` system to work.
/// - You must ensure the type provided in `cast_as` is safe to dereference.
/// - This pattern encourages a declarative and readable way to define relocation logic without
///   repetitive boilerplate.
/// - `SelfSignature`: Function signature for self. like C++ `decltype(T)`
///
/// # See Also
///
/// - `#[relocate_fn]` if you want to relocate and *call* a function with arguments instead of
///   resolving and evaluating a pointer.
#[proc_macro_attribute]
pub fn relocate(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    let crate_root_name = quote::quote! { commonlibsse_ng };

    commonlibsse_ng_proc_macro_common::relocate::gen_relocate(
        attrs.into(),
        item_fn,
        crate_root_name,
    )
    .into()
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
/// - `SelfSignature`: Function signature for self. like C++ `decltype(T)`
///
/// # Panics
/// - The macro generates code that checks the validity of the resolved address, logging an error and panicking if invalid.
#[proc_macro_attribute]
pub fn relocate_fn(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    let crate_root_name = quote::quote! { commonlibsse_ng };

    commonlibsse_ng_proc_macro_common::relocate_fn::gen_relocate_fn(
        attrs.into(),
        item_fn,
        crate_root_name,
    )
    .into()
}

/// An attribute macro to generate FFI-compatible bitflags from an `enum`.
///
/// # Attributes
/// The macro accepts the following arguments:
///
/// | Attribute   | Description                                                                                |
/// |-------------|--------------------------------------------------------------------------------------------|
/// | `flag_name` | The Identifier of Flag struct(optional, defaults to `_CEnum` suffix struct if not provided) |
///
/// # Why this is necessary
/// In the context of FFI (Foreign Function Interface), the `enum` type is often represented as `i32` or `u32`.
/// Using Rust's `enum` in a struct for FFI may cause **undefined behavior** if an invalid or unknown value is received.
///
/// The timing of use is C, C++ return values and class members. Arguments are input, so just make sure to use enum.
///
/// This macro prevents it:
/// - Represent the numeric value of the `enum` in a **Flag** structure that safely represents the value of FFI.
/// - Provide a `to_enum()` method to return the flags to the `enum`,
///   returning `None` for invalid values instead of causing undefined behavior.
/// - Ensure safe and predictable FFI interactions.
///
/// # Example
///
/// - [Expanded sample](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=037f7efdd562a28e7af4cbb59406602b)
///
/// ```rust
/// #[commonlibsse_ng::ffi_enum] // auto generate `struct MyEnum_CEnum(i32)`
/// #[repr(i32)]
/// enum MyEnum {
///     A = 1,
///     B = 2,
///     C = 4,
/// }
///
/// // FFI -> Enum
/// let valid = MyEnum_CEnum::A;
/// assert_eq!(core::mem::size_of::<MyEnum_CEnum>(), core::mem::size_of::<i32>());
/// assert_eq!(valid.to_enum(), Some(MyEnum::A));
///
/// // Invalid flag: using a bit value that is not defined in the enum
/// let invalid = MyEnum_CEnum(999);
/// assert_eq!(invalid.to_enum(), None);
///
/// // Enum -> FFI
/// let flag = MyEnum_CEnum::from_enum(MyEnum::B);
/// assert_eq!(flag, MyEnum_CEnum::B);
/// ```
#[proc_macro_attribute]
pub fn ffi_enum(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let item_enum = syn::parse_macro_input!(item as syn::ItemEnum);
    commonlibsse_ng_proc_macro_common::ffi_enum::ffi_enum(attrs.into(), item_enum).into()
}

/// Converts a regular `enum` into a `bitflags`-compatible type using its variant values.
///
/// The `#[to_bitflags]` attribute macro transforms a plain Rust `enum` definition into a [`bitflags`] struct,
/// while keeping the original enum syntax intact. This is particularly useful when you want to use bitflags
/// functionality without switching to a `bitflags!` macro or changing enum semantics.
///
/// # Syntax
///
/// ```rust
/// #[commonlibsse_ng::to_bitflags]
/// #[derive(Default)]
/// pub enum MyFlags {
///     A = 0b0001,
///     B = 0b0010,
///     C = 0b0100,
///
///     #[default] // Optional, sets the default bitflags (requires `#[derive(Default)]`)
///     None = 0,
/// }
/// ```
///
/// Optionally, you can specify the output type name using the `fn_name` parameter:
///
/// ```rust
/// #[commonlibsse_ng::to_bitflags(fn_name = "MyFlagsBits")]
/// pub enum MyFlags {
///     A = 1,
///     B = 2,
/// }
/// ```
///
/// # Attributes
///
/// | Attribute    | Type      | Required | Description                                                                    |
/// |--------------|-----------|----------|--------------------------------------------------------------------------------|
/// | `fn_name`    | `&str`    | No       | The name of the generated `bitflags` struct. Defaults to the enum name. |
///
/// # Features
///
/// - Automatically implements `bitflags!` for the enum using its variant values.
/// - Honors `#[default]` on a variant if `#[derive(Default)]` is also used.
/// - Avoids the need to rewrite your enum as a `bitflags!` block.
/// - Works with enums using `explicit discriminant values`.
///
/// # Example
///
/// ```rust
/// #[commonlibsse_ng::to_bitflags]
/// #[derive(Default)]
/// pub enum RenderFlags {
///     Alpha = 0b0001,
///     Depth = 0b0010,
///     Stencil = 0b0100,
///
///     #[default]
///     None = 0,
/// }
///
/// let flags = RenderFlags::ALPHA | RenderFlags::DEPTH;
/// assert!(flags.contains(RenderFlags::ALPHA));
/// ```
///
/// # Notes
///
/// - The macro expects all enum variants to have constant discriminant values (e.g., integers or const exprs).
/// - The original `enum` is preserved in the output (i.e., not removed or renamed).
///
/// # Dependencies
///
/// - `#[derive(Default)]` and `#[default]` can be used to control the default value of the generated flag struct.
///
/// [`bitflags`]: https://docs.rs/bitflags
#[proc_macro_attribute]
pub fn to_bitflags(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let item_enum = syn::parse_macro_input!(item as syn::ItemEnum);
    let crate_root_name = quote::quote! { commonlibsse_ng };
    commonlibsse_ng_proc_macro_common::to_bitflags::to_bitflags(
        attrs.into(),
        item_enum,
        crate_root_name,
    )
    .into()
}
