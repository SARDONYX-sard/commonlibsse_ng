#![allow(non_snake_case)]

mod b;
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

// dummy
pub struct GFxMovieView;
pub struct GFxValue;
pub struct InventoryEntryData;

pub struct VMHandle(pub u64);
pub struct FormID(pub u32);

#[derive(Debug)]
pub struct BSTEventSource<T: core::fmt::Debug>(std::marker::PhantomData<T>);
