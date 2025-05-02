use crate::re::BSFixedString::BSFixedString;
use crate::re::BSIInputDevice::BSIInputDevice;
use crate::re::BSTEvent::BSTEventSource;
use crate::re::InputDevices::INPUT_DEVICE;
use crate::re::InputEvent::InputEvent;

#[repr(C)]
#[derive(Debug)]
pub struct BSInputDeviceManager {
    pub __base: BSTEventSource<*mut InputEvent>, // 0x000
    // pub __base1: BSTSingletonSDM<BSInputDeviceManager>, // Empty base optimization -> 0 size
    //
    #[allow(unused)]
    pad59: u8, // 0x059
    #[allow(unused)]
    pad5A: u16, // 0x05A
    #[allow(unused)]
    pad5C: u32, // 0x05C

    pub devices: [*mut BSIInputDevice; 4], // 0x060
}
const _: () = assert!(core::mem::size_of::<BSInputDeviceManager>() == 0x80);

impl BSInputDeviceManager {
    /// Gets the singleton instance of `ControlMap`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut BSInputDeviceManager",
        default = "None",
        deref_once,
        id(se = 516574, ae = 402776)
    )]
    pub fn get_singleton() -> Option<&'static BSInputDeviceManager> {
        |deref_type: DerefType| unsafe { deref_type.as_ref() }
    }

    /// Gets the mutable singleton instance of `BSInputDeviceManager`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut BSInputDeviceManager",
        default = "None",
        deref_once,
        id(se = 516574, ae = 402776)
    )]
    pub fn get_singleton_mut() -> Option<&'static mut BSInputDeviceManager> {
        |deref_type: DerefType| unsafe { deref_type.as_mut() }
    }

    // TODO: `&mut self` permission need really?(currently propagate vtable definition)
    #[inline]
    pub fn get_button_name_from_id(
        &mut self,
        device: INPUT_DEVICE,
        id: u32,
    ) -> Option<BSFixedString> {
        let device = unsafe { self.devices.get_mut(device.0 as usize)?.as_mut() }?;

        let mut output = BSFixedString::new(c"");
        unsafe { (device.vtable.as_ref()?.GetKeyMapping)(device, id, &mut output) };
        Some(output)
    }
}
