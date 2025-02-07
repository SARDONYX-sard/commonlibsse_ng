#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod rel;
pub mod skse;

/// FFI
#[doc(hidden)]
// #[cfg(not(feature = "no_sys"))]
pub mod sys;
