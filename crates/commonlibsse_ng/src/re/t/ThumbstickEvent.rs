use crate::re::BSFixedString::BSFixedString;
use crate::re::IDEvent::{IDEvent, IDEventVtbl};
use crate::re::InputDevices::{INPUT_DEVICE, INPUT_DEVICE_SE};
use crate::re::offsets_rtti::RTTI_ThumbstickEvent;
use crate::re::offsets_vtable::VTABLE_ThumbstickEvent;
use crate::rel::ResolvableAddress as _;
use crate::rel::id::{DataBaseError, VariantID};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct ThumbstickEvent {
    pub __base: IDEvent, // 0x00
    pub xValue: i32,     // 0x28
    pub yValue: i32,     // 0x2C
}
const _: () = assert!(core::mem::size_of::<ThumbstickEvent>() == 0x30);

impl ThumbstickEvent {
    /// Address & offset of RTTI for `ThumbstickEvent`.
    pub const RTTI: VariantID = RTTI_ThumbstickEvent;

    /// Address & offset of Virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_ThumbstickEvent;

    /// # Errors
    pub fn vtable() -> Result<&'static ThumbstickEventVtbl, DataBaseError> {
        Self::VTABLE[0].address().map(|vtable| unsafe { vtable.cast().as_ref() })
    }

    #[inline]
    pub fn init_with_xy(&mut self, id: InputType, x: i32, y: i32) {
        self.init_with_xy_device(id, INPUT_DEVICE_SE::Gamepad.into(), x, y);
    }

    #[inline]
    pub fn init_with_xy_device(&mut self, id: InputType, device: INPUT_DEVICE, x: i32, y: i32) {
        self.init_with_xy_event(id, device, x, y, BSFixedString::new(c""));
    }

    #[inline]
    pub fn init_with_xy_event(
        &mut self,
        id: InputType,
        device: INPUT_DEVICE,
        x: i32,
        y: i32,
        user_event: BSFixedString,
    ) {
        self.xValue = x;
        self.yValue = y;
        self.__base.__base.device = device;
        self.__base.idCode = id as u32;
        self.__base.userEvent = user_event;
    }

    /// Is left ThumbStick event?
    #[inline]
    pub const fn is_left(&self) -> bool {
        self.__base.idCode == (InputType::LeftThumbstick as u32)
    }

    /// Is right ThumbStick event?
    #[inline]
    pub const fn is_right(&self) -> bool {
        self.__base.idCode == (InputType::RightThumbstick as u32)
    }
}

pub struct ThumbstickEventVtbl {
    pub __base: IDEventVtbl, // 0x00
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputType {
    LeftThumbstick = 0x0B,
    RightThumbstick = 0x0C,
}
