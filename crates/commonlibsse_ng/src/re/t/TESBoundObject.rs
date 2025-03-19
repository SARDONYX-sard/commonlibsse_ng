use core::ffi::{c_char, c_int, c_void};

use crate::re::TESObject::TESObject;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NiNPShortPoint3 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}
const _: () = assert!(std::mem::size_of::<NiNPShortPoint3>() == 0x6);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BOUND_DATA {
    pub bound_min: NiNPShortPoint3,
    pub bound_max: NiNPShortPoint3,
}
const _: () = assert!(std::mem::size_of::<BOUND_DATA>() == 0xC);

#[repr(C)]
pub struct TESBoundObject {
    _base: TESObject,
    pub bound_data: BOUND_DATA,
    pub pad2C: u32,
}
const _: () = assert!(std::mem::size_of::<TESBoundObject>() == 0x30);

#[repr(C)]
pub struct TESBoundObjectVtbl {
    // TESObject vtable entries
    pub destructor: unsafe extern "C" fn(this: *mut TESBoundObject),
    pub LoadObjectBound: unsafe extern "C" fn(this: *mut TESBoundObject, a_mod: *mut c_void),
    pub IsBoundObject: unsafe extern "C" fn(this: *const TESBoundObject) -> bool,
    pub Activate: unsafe extern "C" fn(
        this: *mut TESBoundObject,
        target_ref: *mut c_void,
        activator_ref: *mut c_void,
        arg3: u8,
        object: *mut TESBoundObject,
        target_count: c_int,
    ) -> bool,
    pub Clone3D: unsafe extern "C" fn(
        this: *mut TESBoundObject,
        ref_: *mut c_void,
        arg3: bool,
    ) -> *mut c_void,
    pub ReplaceModel: unsafe extern "C" fn(this: *mut TESBoundObject) -> bool,

    // TESBoundObject-specific methods
    pub SetObjectVoiceType:
        unsafe extern "C" fn(this: *mut TESBoundObject, voice_type: *mut c_void),
    pub GetObjectVoiceType: unsafe extern "C" fn(this: *const TESBoundObject) -> *mut c_void,
    pub Clone3D_2:
        unsafe extern "C" fn(this: *mut TESBoundObject, ref_: *mut c_void) -> *mut c_void,
    pub ReplaceModel_2: unsafe extern "C" fn(this: *mut TESBoundObject, str: *const c_char) -> bool,
    pub GetActivateText: unsafe extern "C" fn(
        this: *mut TESBoundObject,
        activator: *mut c_void,
        dst: *mut c_void,
    ) -> bool,
    pub CalculateDoFavor: unsafe extern "C" fn(
        this: *mut TESBoundObject,
        activator: *mut c_void,
        arg2: bool,
        to_activate: *mut c_void,
        arg3: f32,
    ) -> bool,
    pub HandleRemoveItemFromContainer:
        unsafe extern "C" fn(this: *mut TESBoundObject, container: *mut c_void),
    pub OnRemove3D: unsafe extern "C" fn(this: *mut TESBoundObject, obj3D: *mut c_void),
    pub OnCheckModels: unsafe extern "C" fn(this: *mut TESBoundObject),
    pub OnCopyReference: unsafe extern "C" fn(this: *mut TESBoundObject),
    pub OnFinishScale: unsafe extern "C" fn(this: *mut TESBoundObject),
}
