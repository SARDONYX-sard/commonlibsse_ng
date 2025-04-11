//! `commonlibsse_ng` macro to automatically generate commonly used code patterns for crate use.
//!

use proc_macro::TokenStream;

/// Relocates an address using Skyrim runtime-specific relocation IDs.
///
/// The `#[relocate]` attribute macro enables dynamic resolution of a pointer using relocation
/// IDs for Skyrim Special Edition (SE), Anniversary Edition (AE), and optionally VR. It injects code
/// to resolve the address at runtime and execute user-provided logic through a closure, passing the
/// resolved pointer as an argument.
///
/// # Attributes
///
/// | Attribute      | Type      | Required | Description                                                                      |
/// |----------------|-----------|----------|----------------------------------------------------------------------------------|
/// | `cast_as`      | `&str`    | Yes      | The type to cast the resolved pointer to (e.g., `"*mut bool"`, `"*mut T"`).      |
/// | `default`      | `&str`    | Yes      | The fallback value returned if resolution fails (e.g., `"false"`, `None`).       |
/// | `deref_once`   | `bool`    | No       | If specified, the casted pointer will be dereferenced once(by `read_unaligned`). |
/// | `id.se`        | `u64`     | Yes      | Relocation ID for Skyrim Special Edition.                                        |
/// | `id.ae`        | `u64`     | Yes      | Relocation ID for Skyrim Anniversary Edition.                                    |
/// | `id.vr`        | `u64`     | No       | Relocation ID for Skyrim VR. Defaults to `se` if omitted.                        |
///
/// If `deref_once` is specified and the `cast_as` type is a multi-level pointer (e.g., `*mut *mut T`),
/// the macro will automatically strip one level and define a helper type alias `DerefType`:
///
/// ```rust
/// type DerefType = *mut T;
/// ```
///
/// This type alias can then be used as the parameter type in the closure.
///
/// # Function Body
///
/// The function body must be a single closure of the form:
///
/// ```rust
/// |as_type: AsType| { ... }
/// ```
///
/// where `AsType` is either the raw casted pointer or the dereferenced value, depending on `deref_once`.
///
/// If resolution fails, the `default` value will be returned instead (parsed as a Rust expression).
///
/// # Examples
///
/// ## Without deref_once
/// ```rust
/// #[commonlibsse_ng_derive_internal::relocate(
///     cast_as = "*mut EntryPoint",
///     default = "None",
///     id(se = 675707, ae = 368994)
/// )]
/// #[inline]
/// fn entry_points(entry_point: ENTRY_POINT) -> Option<NonNull<EntryPoint>> {
///     |as_type| unsafe { NonNull::new(as_type.add(entry_point as usize)) }
/// }
/// ```
///
/// ## With deref_once and pointed pointer(e.g. `GetSingleton`)
/// ```rust
/// #[commonlibsse_ng_derive_internal::relocate(
///     cast_as = "*mut *mut INIPrefSettingCollection",
///     default = "None",
///     deref_once,
///     id(se = 524557, ae = 411155)
/// )]
/// pub fn get_singleton() -> Option<&'static INIPrefSettingCollection> {
///     |deref_type: DerefType| unsafe { deref_type.as_ref() }
/// }
/// ```
///
/// # Notes
///
/// - `cast_as` must be a valid Rust type (pointer types encouraged for safety).
/// - `deref_once` is especially useful for singletons and global pointers.
/// - This pattern avoids boilerplate and enables declarative relocation definitions.
///
/// # See Also
///
/// - [`#[relocate_fn]`](relocate_fn) — relocate and invoke a function with arguments instead of reading a pointer.
#[proc_macro_attribute]
pub fn relocate(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    let crate_root_name = quote::quote! { crate };

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
/// #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 1, ae_id = 2, vr_id = 3)]
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
    let crate_root_name = quote::quote! { crate };

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
/// #[commonlibsse_ng_derive_internal::ffi_enum] // auto generate `struct MyEnum_CEnum(i32)`
/// #[repr(i32)]
/// enum MyEnum {
///     A = 1,
///     B = 2,
///     C = 4,
/// }
///
/// assert_eq!(MyEnum_CEnum::count(), 3);
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
/// #[commonlibsse_ng_derive_internal::to_bitflags]
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
/// #[commonlibsse_ng_derive_internal::to_bitflags(fn_name = "MyFlagsBits")]
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
/// #[commonlibsse_ng_derive_internal::to_bitflags]
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
    let crate_root_name = quote::quote! { crate };
    commonlibsse_ng_proc_macro_common::to_bitflags::to_bitflags(
        attrs.into(),
        item_enum,
        crate_root_name,
    )
    .into()
}
