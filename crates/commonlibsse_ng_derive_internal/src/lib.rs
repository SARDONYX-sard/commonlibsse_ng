//! `commonlibsse_ng` macro to automatically generate commonly used code patterns for crate use.
//!

use proc_macro::TokenStream;

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
/// #[commonlibsse_ng_internal::relocate_fn(se_id = 1, ae_id = 2, vr_id = 3)]
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
/// | `flag_name` | The Identifier of Flag struct(optional, defaults to `Flags` suffix struct if not provided) |
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
/// #[commonlibsse_ng_derive_internal::ffi_enum] // auto generate `struct MyEnumFlags(i32)`
/// #[repr(i32)]
/// enum MyEnum {
///     A = 1,
///     B = 2,
///     C = 4,
/// }
///
/// // FFI -> Enum
/// let valid = MyEnumFlags::A;
/// assert_eq!(core::mem::size_of::<MyEnumFlags>(), core::mem::size_of::<i32>());
/// assert_eq!(valid.to_enum(), Some(MyEnum::A));
///
/// // Invalid flag: using a bit value that is not defined in the enum
/// let invalid = MyEnumFlags(999);
/// assert_eq!(invalid.to_enum(), None);
///
/// // Enum -> FFI
/// let flag = MyEnumFlags::from_enum(MyEnum::B);
/// assert_eq!(flag, MyEnumFlags::B);
/// ```
#[proc_macro_attribute]
pub fn ffi_enum(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let item_enum = syn::parse_macro_input!(item as syn::ItemEnum);
    commonlibsse_ng_proc_macro_common::ffi_enum::ffi_enum(attrs.into(), item_enum).into()
}

/// This is a convenience macro for those who wish to use bitflags without changing the existing enum notation.
#[proc_macro_attribute]
pub fn to_bitflags(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let item_enum = syn::parse_macro_input!(item as syn::ItemEnum);
    commonlibsse_ng_proc_macro_common::to_bitflags::to_bitflags(attrs.into(), item_enum).into()
}
