#[cfg(not(feature = "no_sys"))]
mod interface;
#[cfg(not(feature = "no_sys"))]
mod trampoline;
#[cfg(not(feature = "no_sys"))]
mod translation;

pub mod version;
