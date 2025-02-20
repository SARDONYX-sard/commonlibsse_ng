//! Module related to SKSE.exe (version information of SkyrimSE.exe, etc.)

pub mod api;
pub mod impls;
pub mod input_map;
pub mod interfaces;
pub mod trampoline;
#[cfg(not(feature = "no_sys"))]
mod translation;
pub mod version;

pub use api::init;
