use core::ffi::CStr;

use crate::re::Setting::Setting;
use crate::re::SettingCollectionList::{SettingCollectionList, SettingCollectionListVtbl};
use crate::re::offsets_rtti::RTTI_INISettingCollection;
use crate::re::offsets_vtable::VTABLE_INISettingCollection;
use crate::rel::id::VariantID;

#[derive(Debug)]
#[repr(C)]
pub struct INISettingCollection {
    pub __base: SettingCollectionList<Setting>, // 0x000
}
const _: () = {
    assert!(core::mem::offset_of!(INISettingCollection, __base) == 0x00);
    assert!(core::mem::size_of::<INISettingCollection>() == 0x128);
};

impl INISettingCollection {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_INISettingCollection;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_INISettingCollection;

    /// Gets the singleton instance of `INISettingCollection`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut INISettingCollection",
        default = "None",
        deref_once,
        id(se = 524557, ae = 411155)
    )]
    #[inline]
    pub fn get_singleton() -> Option<&'static INISettingCollection> {
        |deref_type: DerefType| unsafe { deref_type.as_ref() }
    }

    pub fn get_setting(&self, name: &CStr) -> Option<&Setting> {
        for setting in &self.__base.settings {
            let setting = match unsafe { setting.as_ref() } {
                Some(setting) => setting,
                None => continue,
            };
            let setting_name = match setting.get_name() {
                Some(name) => name,
                None => continue,
            };

            if name == setting_name {
                return Some(setting);
            }
        }

        None
    }
}

/// The virtual function table for `INIPrefSettingCollection`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct INISettingCollectionVtbl {
    pub __base: SettingCollectionListVtbl<Setting>,
}
