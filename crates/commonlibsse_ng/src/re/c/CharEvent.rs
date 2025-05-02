use crate::re::InputEvent::{InputEvent, InputEventVtbl};
use crate::re::offsets_rtti::RTTI_CharEvent;
use crate::re::offsets_vtable::VTABLE_CharEvent;
use crate::rel::ResolvableAddress as _;
use crate::rel::id::{DataBaseError, VariantID};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct CharEvent {
    pub __base: InputEvent, // 0x00
    pub keyCode: u32,       // 0x18
    pub pad1C: u32,         // 0x1C
}
const _: () = assert!(core::mem::size_of::<CharEvent>() == 0x20);

impl CharEvent {
    /// Address & offset of RTTI for `CharEvent`.
    pub const RTTI: VariantID = RTTI_CharEvent;

    /// Address & offset of Virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_CharEvent;

    /// # Errors
    pub fn vtable() -> Result<&'static CharEventVtbl, DataBaseError> {
        Self::VTABLE[0].address().map(|vtable| unsafe { vtable.cast().as_ref() })
    }

    #[inline]
    pub const fn init(&mut self, key_code: u32) {
        self.keyCode = key_code;
    }
}

pub struct CharEventVtbl {
    pub __base: InputEventVtbl, // 0x00
}
