use core::marker::PhantomData;

use stdx::ptr::ConstNonNull;

use crate::re::BSFixedString::BSFixedString;
use crate::re::InputDevices::INPUT_DEVICE;
use crate::re::offsets_rtti::RTTI_InputEvent;
use crate::re::offsets_vtable::VTABLE_InputEvent;
use crate::rel::id::VariantID;

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum INPUT_EVENT_TYPE {
    Button = 0,
    MouseMove,
    Char,
    Thumbstick,
    DeviceConnect,
    Kinect,
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct InputEvent {
    pub vtable: *const InputEventVtbl, // 0x00
    pub device: INPUT_DEVICE,          // 0x08
    pub eventType: INPUT_EVENT_TYPE,   // 0x0C
    pub next: *mut InputEvent,         // 0x10
}
const _: () = assert!(core::mem::size_of::<InputEvent>() == 0x18);

impl InputEvent {
    /// Address & offset of RTTI for `InputEvent`.
    pub const RTTI: VariantID = RTTI_InputEvent;

    /// Address & offset of Virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_InputEvent;

    #[inline]
    pub const fn iter(&self) -> InputEventIterator {
        InputEventIterator::new(self)
    }
}

pub struct InputEventVtbl {
    pub CxxDrop: unsafe extern "C" fn(this: *mut InputEvent), // 0x0
    pub HasIDCode: unsafe extern "C" fn(this: *const InputEvent) -> bool, // 0x1
    pub QUserEvent: unsafe extern "C" fn(this: *const InputEvent) -> &'static BSFixedString, // 0x2
}

// Iterator for InputEvent
pub struct InputEventIterator<'a> {
    current: Option<ConstNonNull<InputEvent>>,
    marker: PhantomData<&'a InputEvent>,
}
impl<'a> InputEventIterator<'a> {
    pub const fn new(current: &'a InputEvent) -> Self {
        Self { current: Some(ConstNonNull::from_ref(current)), marker: PhantomData }
    }
}
impl<'a> Iterator for InputEventIterator<'a> {
    type Item = &'a InputEvent;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        let current_ref = unsafe { current.as_ref() };
        self.current = ConstNonNull::new(current_ref.next);
        Some(current_ref)
    }
}
