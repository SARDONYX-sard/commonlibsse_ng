//! Module related to SKSE.exe (version information of SkyrimSE.exe, etc.)

pub mod api;
pub mod impls;
pub mod input_map;
pub mod interfaces;
pub mod logger;
pub mod trampoline;
pub mod translation;
pub mod version;

pub use api::init;
