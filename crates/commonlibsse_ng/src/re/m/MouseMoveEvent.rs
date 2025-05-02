use crate::re::BSFixedString::BSFixedString;
use crate::re::IDEvent::{IDEvent, IDEventVtbl};
use crate::re::offsets_rtti::RTTI_MouseMoveEvent;
use crate::re::offsets_vtable::VTABLE_MouseMoveEvent;
use crate::rel::ResolvableAddress;
use crate::rel::id::{DataBaseError, VariantID};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct MouseMoveEvent {
    pub __base: IDEvent,  // 0x00
    pub mouseInputX: i32, // 0x28
    pub mouseInputY: i32, // 0x2C
}
const _: () = assert!(core::mem::size_of::<MouseMoveEvent>() == 0x30);

impl MouseMoveEvent {
    /// Address & offset of RTTI for `MouseMoveEvent`.
    pub const RTTI: VariantID = RTTI_MouseMoveEvent;

    /// Address & offset of Virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_MouseMoveEvent;

    /// # Errors
    pub fn vtable() -> Result<&'static MouseMoveEventVtbl, DataBaseError> {
        Self::VTABLE[0].address().map(|vtable| unsafe { vtable.cast().as_ref() })
    }

    #[inline]
    pub const fn set_xy(&mut self, x: i32, y: i32) {
        self.mouseInputX = x;
        self.mouseInputY = y;
    }

    #[inline]
    pub fn set_xy_with_event(&mut self, x: i32, y: i32, user_event: BSFixedString) {
        self.mouseInputX = x;
        self.mouseInputY = y;
        self.__base.userEvent = user_event;
    }
}

pub struct MouseMoveEventVtbl {
    pub __base: IDEventVtbl, // 0x00
}
