#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod b;
mod e;
mod f;
mod i;
mod n;
mod t;

pub mod rtti;

#[doc(hidden)]
#[rustfmt::skip]
pub mod offsets_rtti;
#[doc(hidden)]
#[rustfmt::skip]
pub mod offsets_ni_rtti;
#[doc(hidden)]
#[rustfmt::skip]
pub mod offsets_vtable;

pub use b::*;
pub use e::*;
pub use f::*;
pub use i::*;
pub use n::*;
pub use t::*;

// dummy
pub struct GFxMovieView;
pub struct GFxValue;

pub struct VMHandle(pub u64);
pub struct FormID(pub u32);
