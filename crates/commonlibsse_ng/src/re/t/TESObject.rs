use core::ffi::{c_uint, c_void};

use crate::{
    re::{TESForm::TESForm, offsets_rtti::RTTI_TESObject, offsets_vtable::VTABLE_TESObject},
    rel::id::VariantID,
};

#[repr(C)]
pub struct TESObject {
    _base: TESForm,
}
const _: () = assert!(core::mem::size_of::<TESObject>() == 0x20);

#[repr(C)]
pub struct TESObjectVtbl {
    // TESForm vtable entries
    pub destructor: unsafe extern "C" fn(this: *mut TESObject),
    pub IsObject: unsafe extern "C" fn(this: *const TESObject) -> bool,
    pub GetRefCount: unsafe extern "C" fn(this: *const TESObject) -> c_uint,

    // TESObject-specific methods
    pub Unk_3B: unsafe extern "C" fn(this: *mut TESObject),
    pub IsBoundAnimObject: unsafe extern "C" fn(this: *mut TESObject) -> bool,
    pub GetWaterType: unsafe extern "C" fn(this: *const TESObject) -> *mut c_void,
    pub IsAutoCalc: unsafe extern "C" fn(this: *const TESObject) -> bool,
    pub SetAutoCalc: unsafe extern "C" fn(this: *mut TESObject, auto_calc: bool),
    pub Clone3D:
        unsafe extern "C" fn(this: *mut TESObject, ref_: *mut c_void, arg3: bool) -> *mut c_void,
    pub UnClone3D: unsafe extern "C" fn(this: *mut TESObject, ref_: *mut c_void),
    pub IsMarker: unsafe extern "C" fn(this: *mut TESObject) -> bool,
    pub IsOcclusionMarker: unsafe extern "C" fn(this: *mut TESObject) -> bool,
    pub ReplaceModel: unsafe extern "C" fn(this: *mut TESObject) -> bool,
    pub IncRef: unsafe extern "C" fn(this: *mut TESObject) -> c_uint,
    pub DecRef: unsafe extern "C" fn(this: *mut TESObject) -> c_uint,
    pub LoadGraphics: unsafe extern "C" fn(this: *mut TESObject, ref_: *mut c_void) -> *mut c_void,
}

impl TESObject {
    pub const RTTI: VariantID = RTTI_TESObject;
    pub const VTABLE: [VariantID; 1] = VTABLE_TESObject;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ChangeFlags {
    ObjectValue = 1 << 1,
    ObjectFullName = 1 << 2,
}
