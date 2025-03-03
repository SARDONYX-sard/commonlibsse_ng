//! This is a Rust reimplementation of CommonLibSSE-NG.
//!
//! It is intended to be memory-safe using the power of Rust.
//!
//! - rel: Module related to Relocation (calculate address from ID according to version, read module information, parse version information, etc.)
//! - rex: Module related to Win32 API
//! - skse: Module related to SKSE.exe (version information of SkyrimSE.exe, etc.)

pub mod re;
pub mod rel;
pub mod rex;
pub mod skse;

#[cfg(feature = "derive")]
pub use commonlibsse_ng_derive::skse_plugin_main;
