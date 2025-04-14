//! This is a Rust reimplementation of CommonLibSSE-NG.
//!
//! It is intended to be memory-safe using the power of Rust.
//!
//! - rel: Module related to Relocation (calculate address from ID according to version, read module information, parse version information, etc.)
//! - rex: Module related to Win32 API
//! - skse: Module related to SKSE.exe (version information of SkyrimSE.exe, etc.)

#[macro_use]
mod macros;

pub mod re;
pub mod rel;
pub mod rex;
pub mod skse;

#[cfg(feature = "derive")]
pub use commonlibsse_ng_derive::{ffi_enum, relocate, relocate_fn, skse_plugin_main, to_bitflags};

// Used by generated code and doc tests. Not public API.(For derive)
#[doc(hidden)]
#[path = "private/mod.rs"]
pub mod __private;
