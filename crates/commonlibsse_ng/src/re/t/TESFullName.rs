use crate::re::BSFixedString::BSFixedString;
use crate::re::BaseFormComponent::{BaseFormComponent, BaseFormComponentVtbl};
use crate::re::offsets_rtti::RTTI_TESFullName;
use crate::re::offsets_vtable::VTABLE_TESFullName;
use crate::rel::id::VariantID;
use core::ffi::c_char;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct TESFullName {
    pub __base: BaseFormComponent, // 0x00
    pub fullName: BSFixedString,   // 0x08 - FULL
}
const _: () = assert!(std::mem::size_of::<TESFullName>() == 0x10);

impl TESFullName {
    pub const RTTI: VariantID = RTTI_TESFullName;
    pub const VTABLE: [VariantID; 1] = VTABLE_TESFullName;

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 22318, ae_id = 22791)]
    pub fn set_full_name(&mut self, name: *const c_char) {}

    /// Returns a reference to the virtual function table for `TESFullName`.
    ///
    /// # Panics
    ///
    /// Panics if the vtable pointer is null.
    #[inline]
    fn get_vtable(&self) -> &TESFullNameVtbl {
        let vtbl = self.__base.vtable.cast::<TESFullNameVtbl>();
        if vtbl.is_null() {
            #[cfg(feature = "tracing")]
            tracing::error!("TESFullName::vtbl is null — object: {:p}", self);
            panic!("TESFullName::vtbl is null");
        }

        unsafe { &*vtbl }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TESFullNameVtbl {
    /// override
    /// - `CxxDrop`(0x00):
    /// - `InitializeDataComponent`(0x01): `{ name = ""; }`
    /// - `ClearDataComponent`(0x02): `{ return; }`
    /// - `CopyComponent`(0x03):
    pub __base: BaseFormComponentVtbl,
    pub GetFullNameLength: fn(this: *const TESFullName) -> u32, // 0x04
    pub GetFullName: fn(this: *const TESFullName) -> *const c_char, // 0x05
}
const _: () = {
    const VFUNC_COUNT: usize = 0x5;

    const EXPECTED_SIZE: usize = (VFUNC_COUNT + 1) * core::mem::size_of::<usize>();
    assert!(core::mem::size_of::<TESFullNameVtbl>() == EXPECTED_SIZE);
};

pub trait TESFullNameVtblTrait {
    fn GetFullNameLength(this: &TESFullName) -> u32; // 0x04
    fn GetFullName(this: &TESFullName) -> *const c_char; // 0x05
}

impl TESFullNameVtblTrait for TESFullName {
    #[inline]
    fn GetFullNameLength(this: &TESFullName) -> u32 {
        let vtable = this.get_vtable();
        let func = vtable.GetFullNameLength;
        func(this)
    }

    #[inline]
    fn GetFullName(this: &TESFullName) -> *const c_char {
        let vtable = this.get_vtable();
        let func = vtable.GetFullName;
        func(this)
    }
}
