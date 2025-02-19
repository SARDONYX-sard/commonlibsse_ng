#![allow(non_snake_case)]

mod b;

pub use b::*;

// dummy
pub struct GFxMovieView;
pub struct GFxValue;
pub struct InventoryEntryData;

pub struct VMHandle(pub u64);
pub struct FormID(pub u32);

pub struct BSTEventSource<T>(std::marker::PhantomData<T>);
