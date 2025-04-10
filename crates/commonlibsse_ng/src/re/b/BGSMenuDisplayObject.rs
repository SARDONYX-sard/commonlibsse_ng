use core::ptr::NonNull;

use crate::re::BaseFormComponent::{BaseFormComponent, BaseFormComponentVtbl};
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::offsets_rtti::RTTI_BGSMenuDisplayObject;
use crate::re::offsets_vtable::VTABLE_BGSMenuDisplayObject;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct BGSMenuDisplayObject {
    pub __base: BaseFormComponent,                  // 0x0
    pub equipSlot: Option<NonNull<TESBoundObject>>, // 0x8
}
const _: () = assert!(core::mem::size_of::<BGSMenuDisplayObject>() == 0x10);

impl BGSMenuDisplayObject {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_BGSMenuDisplayObject;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_BGSMenuDisplayObject;
}

pub struct BGSMenuDisplayObjectVtbl {
    pub __base: BaseFormComponentVtbl,
    pub GetMenuDisplayObject: fn(this: &BGSMenuDisplayObject),
}
