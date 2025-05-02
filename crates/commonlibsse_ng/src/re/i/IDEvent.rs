use crate::re::BSFixedString::BSFixedString;
use crate::re::InputEvent::{InputEvent, InputEventVtbl};
use crate::re::offsets_rtti::RTTI_IDEvent;
use crate::re::offsets_vtable::VTABLE_IDEvent;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct IDEvent {
    pub __base: InputEvent,       // 0x00
    pub userEvent: BSFixedString, // 0x18
    pub idCode: u32,              // 0x20
    pub pad24: u32,               // 0x24
}
const _: () = assert!(core::mem::size_of::<IDEvent>() == 0x28);

impl IDEvent {
    /// Address & offset of RTTI for `IDEvent`.
    pub const RTTI: VariantID = RTTI_IDEvent;

    /// Address & offset of Virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_IDEvent;
}

pub struct IDEventVtbl {
    pub __base: InputEventVtbl, // 0x00
}
