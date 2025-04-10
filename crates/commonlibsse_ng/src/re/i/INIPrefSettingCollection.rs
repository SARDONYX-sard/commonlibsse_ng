use crate::re::INISettingCollection::{INISettingCollection, INISettingCollectionVtbl};
use crate::re::offsets_rtti::RTTI_INIPrefSettingCollection;
use crate::re::offsets_vtable::VTABLE_INIPrefSettingCollection;
use crate::rel::id::VariantID;

#[derive(Debug)]
#[repr(C)]
pub struct INIPrefSettingCollection {
    pub __base: INISettingCollection, // 0x000
}
const _: () = {
    assert!(core::mem::offset_of!(INIPrefSettingCollection, __base) == 0x00);
    assert!(core::mem::size_of::<INIPrefSettingCollection>() == 0x128);
};

impl INIPrefSettingCollection {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_INIPrefSettingCollection;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_INIPrefSettingCollection;

    /// Gets the singleton instance of `INIPrefSettingCollection`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut INIPrefSettingCollection",
        default = "None",
        id(se = 524557, ae = 411155)
    )]
    pub fn get_singleton() -> Option<&'static INIPrefSettingCollection> {
        |as_type: AsType| unsafe { as_type.as_ref() }
    }
}

/// The virtual function table for `INIPrefSettingCollection`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct INIPrefSettingCollectionVtbl {
    pub __base: INISettingCollectionVtbl,
}
