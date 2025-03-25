use crate::re::offsets_rtti::RTTI_BaseFormComponent;
use crate::re::offsets_vtable::VTABLE_BaseFormComponent;
use crate::rel::id::VariantID;
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug)]
pub struct BaseFormComponent {
    pub vtbl: *const BaseFormComponentVtbl,
}

#[repr(C)]
#[derive(Debug)]
pub struct BaseFormComponentVtbl {
    /// C++ destructor
    pub delete: extern "C" fn(this: *mut c_void),

    /// - `BaseFormComponent`: pure virtual
    pub initialize_data_component: extern "C" fn(this: *mut c_void),
    /// - `BaseFormComponent`: pure virtual
    pub clear_data_component: extern "C" fn(this: *mut c_void),
    /// - `BaseFormComponent`: always return `c_void`
    pub copy_component: extern "C" fn(this: *mut c_void, _rhs: *mut c_void),
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
