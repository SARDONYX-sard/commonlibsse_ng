//! Module related to SKSE.exe (version information of SkyrimSE.exe, etc.)

pub mod impls;
// #[cfg(not(feature = "no_sys"))]
pub mod interface;
pub mod trampoline;
#[cfg(not(feature = "no_sys"))]
mod translation;

pub mod version;
