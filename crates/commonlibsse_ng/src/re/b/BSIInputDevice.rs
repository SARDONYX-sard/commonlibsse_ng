use crate::re::BSFixedString::BSFixedString;
use crate::re::offsets_rtti::RTTI_BSIInputDevice;
use crate::re::offsets_vtable::VTABLE_BSIInputDevice;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct BSIInputDevice {
    pub vtable: *const BSIInputDeviceVtbl, // 0x00
}
const _: () = assert!(core::mem::size_of::<BSIInputDevice>() == 0x8);

impl BSIInputDevice {
    /// Address & offset of RTTI for `BSIInputDevice`.
    pub const RTTI: VariantID = RTTI_BSIInputDevice;

    /// Address & offset of Virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_BSIInputDevice;
}

pub struct BSIInputDeviceVtbl {
    pub CxxDrop: unsafe extern "C" fn(this: *mut BSIInputDevice), // 0x0
    pub Initialize: unsafe extern "C" fn(this: *mut BSIInputDevice), // 0x1
    pub Process: unsafe extern "C" fn(this: *mut BSIInputDevice, arg1: f32), // 0x2
    pub Release: unsafe extern "C" fn(this: *mut BSIInputDevice), // 0x3
    pub GetKeyMapping:
        unsafe extern "C" fn(this: *mut BSIInputDevice, key: u32, mapping: &mut BSFixedString), // 0x4
    pub GetMappingKey: unsafe extern "C" fn(this: *mut BSIInputDevice, mapping: &mut BSFixedString), // 0x5
    pub GetMappingKeyCode:
        unsafe extern "C" fn(this: *mut BSIInputDevice, key: u32, out_key_code: &mut u32), // 0x6
    pub IsEnabled: unsafe extern "C" fn(this: *const BSIInputDevice), // 0x7
    pub Reset: unsafe extern "C" fn(this: *mut BSIInputDevice),       // 0x8
}
