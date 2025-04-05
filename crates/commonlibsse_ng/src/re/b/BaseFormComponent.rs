use crate::re::offsets_rtti::RTTI_BaseFormComponent;
use crate::re::offsets_vtable::VTABLE_BaseFormComponent;
use crate::rel::id::VariantID;
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BaseFormComponent {
    pub vtbl: *const BaseFormComponentVtbl,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BaseFormComponentVtbl {
    /// C++ destructor: `~BaseFormComponent`
    pub CxxDrop: extern "C" fn(this: *mut c_void),

    /// - `BaseFormComponent`: pure virtual
    pub InitializeDataComponent: extern "C" fn(this: *mut c_void),
    /// - `BaseFormComponent`: pure virtual
    pub ClearDataComponent: extern "C" fn(this: *mut c_void),
    /// - `BaseFormComponent`: always return `c_void`
    pub CopyComponent: extern "C" fn(this: *mut c_void, _rhs: *mut c_void),
}

const _: () = {
    assert!(core::mem::size_of::<BaseFormComponent>() == 0x8);
    assert!(core::mem::align_of::<BaseFormComponent>() == 8);

    assert!(core::mem::size_of::<BaseFormComponentVtbl>() == 32); // 4 * 8
};

impl BaseFormComponent {
    pub const RTTI: VariantID = RTTI_BaseFormComponent;
    pub const VTABLE: [VariantID; 1] = VTABLE_BaseFormComponent;
}
