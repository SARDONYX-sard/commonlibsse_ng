#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod a;
mod b;
mod c;
mod d;
mod e;
mod f;
mod g;
mod h;
mod i;
mod m;
mod n;
mod o;
mod p;
mod q;
mod s;
mod t;
mod u;

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

pub use self::a::*;
pub use self::b::*;
pub use self::c::*;
pub use self::d::*;
pub use self::e::*;
pub use self::f::*;
pub use self::g::*;
pub use self::h::*;
pub use self::i::*;
pub use self::m::*;
pub use self::n::*;
pub use self::o::*;
pub use self::p::*;
pub use self::q::*;
pub use self::s::*;
pub use self::t::*;
pub use self::u::*;

mod dummy_types;
pub use dummy_types::*;

/// C++ Virtual Class RTTI & Vtable accessor
pub trait CxxVirtClass {
    /// Gets the runtime information address ID reference.
    fn rtti() -> &'static crate::rel::id::VariantID;
    /// Gets the virtual function table address reference.
    fn vtable() -> &'static [crate::rel::id::VariantID];
}
