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

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// dummies

pub struct GFxMovieView;
pub struct GFxValue;

pub struct VMHandle(pub u64);
pub struct FormID(pub u32);

pub struct ExtraContainerChanges {
    changes: *mut InventoryChanges::InventoryChanges,
}

pub struct TesWaterForm;

pub struct BSHandleRefObject {
    pub opaque: [u8; 10],
}
pub struct BSTEventSink<T> {
    maker: core::marker::PhantomData<T>,
}
pub struct BSAnimationGraphEvent;
pub struct IAnimationGraphManagerHolder;

pub enum ItemRemoveReason {
    Remove,
    Steal,
    Selling,
    Dropping,
    StoreInContainer,
    StoreInTeammate,
}
pub struct ObjectHandle;
pub struct NiExtraData;
pub struct NiTimeController;

impl NiSmartPointer::RefCountable for NiTimeController {
    fn inc_ref_count(&self) {
        todo!()
    }

    fn dec_ref_count(&self) {
        todo!()
    }
}
pub struct NiNode;
pub struct NiCollisionObject;

impl crate::re::NiSmartPointer::RefCountable for NiCollisionObject {
    fn inc_ref_count(&self) {
        unimplemented!()
    }

    fn dec_ref_count(&self) {
        unimplemented!()
    }
}
