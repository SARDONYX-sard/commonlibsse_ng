use crate::re::NiCollisionObject;
use crate::re::NiObjectNET::NiObjectNET;
use crate::re::NiSmartPointer::NiPointer;
use crate::re::NiTransform::NiTransform;
use crate::re::{NiBound::NiBound, NiNode};
use core::ffi::{c_float, c_void};

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum NiUpdateData_Flag {
    None = 0,
    Dirty = 1 << 0,
    DisableCollision = 1 << 13,
}

#[repr(C)]
#[derive(Debug)]
pub struct NiUpdateData {
    pub time: c_float,            // 0x00
    pub flags: NiUpdateData_Flag, // 0x04
}

#[repr(C)]
pub struct PerformOpFuncVtbl {
    /// C++ class Destructor equivalent
    pub cxx_drop: unsafe extern "C" fn(this: *mut c_void), // 00

    /// C++ operator()
    pub op_call: unsafe extern "C" fn(*mut c_void, *mut NiAVObject) -> bool, // 01 (operator())
}

#[repr(C)]
pub struct PerformOpFunc {
    pub vtable: *const PerformOpFuncVtbl, // VTable pointer
}

#[repr(C)]
pub struct NiAVObject {
    pub __base: NiObjectNET,
    pub parent: *mut NiNode,                            // 0x030
    pub parent_index: u32,                              // 0x038
    pub unk03C: u32,                                    // 0x03C
    pub collision_object: NiPointer<NiCollisionObject>, // 0x040
    pub local_transform: NiTransform,                   // 0x048
    pub world_transform: NiTransform,                   // 0x07C
    pub previous_world_transform: NiTransform,          // 0x0B0
    pub world_bound: NiBound,                           // 0x0E4
    pub unk_f4: u32,                                    // 0x0F4
    pub unk_f8: u64,                                    // 0x0F8
    pub fade_amount: c_float,                           // 0x100
    pub last_updated_frame_counter: u32,                // 0x104
    pub unk104: u64,                                    // 0x108
}

const _: () = {
    assert!(core::mem::offset_of!(NiAVObject, __base) == 0x0);
    assert!(core::mem::offset_of!(NiAVObject, parent) == 0x30);
    assert!(core::mem::offset_of!(NiAVObject, parent_index) == 0x38);
    assert!(core::mem::offset_of!(NiAVObject, unk03C) == 0x3C);
    assert!(core::mem::offset_of!(NiAVObject, collision_object) == 0x40);
    assert!(core::mem::offset_of!(NiAVObject, local_transform) == 0x48);
    assert!(core::mem::offset_of!(NiAVObject, world_transform) == 0x7C);
    assert!(core::mem::offset_of!(NiAVObject, previous_world_transform) == 0xB0);
    assert!(core::mem::offset_of!(NiAVObject, world_bound) == 0xE4);
    assert!(core::mem::offset_of!(NiAVObject, unk_f4) == 0xF4);
    assert!(core::mem::offset_of!(NiAVObject, unk_f8) == 0xF8);
    assert!(core::mem::offset_of!(NiAVObject, fade_amount) == 0x100);
    assert!(core::mem::offset_of!(NiAVObject, last_updated_frame_counter) == 0x104);
    assert!(core::mem::offset_of!(NiAVObject, unk104) == 0x108);
    assert!(core::mem::size_of::<NiAVObject>() == 0x110);
};

#[repr(C)]
pub struct NiAVObjectVtbl {
    /// C++ class Destructor equivalent
    pub cxx_drop: unsafe extern "C" fn(this: *mut c_void), // 00

    // override (NiObjectNET)
    pub GetRTTI: unsafe extern "C" fn(this: *const c_void) -> *const c_void, // 02
    pub LoadBinary: unsafe extern "C" fn(this: *mut c_void, a_stream: *mut c_void), // 18
    pub LinkObject: unsafe extern "C" fn(this: *mut c_void, a_stream: *mut c_void), // 19
    pub RegisterStreamables: unsafe extern "C" fn(this: *mut c_void, a_stream: *mut c_void) -> bool, // 1A
    pub SaveBinary: unsafe extern "C" fn(this: *mut c_void, a_stream: *mut c_void), // 1B
    pub IsEqual: unsafe extern "C" fn(this: *mut c_void, a_object: *mut c_void) -> bool, // 1C
    pub ProcessClone: unsafe extern "C" fn(this: *mut c_void, a_cloning: *mut c_void), // 1D

    // Custom add-ons
    pub UpdateControllers: unsafe extern "C" fn(this: *mut c_void, a_data: *mut c_void), // 25

    // VR
    pub ApplyLocalTransformToWorld: Option<unsafe extern "C" fn(this: *mut c_void)>, // Optional, for VR-specific functionality
    pub PerformOp: Option<unsafe extern "C" fn(this: *mut c_void, a_func: *mut c_void)>, // 26
    pub AttachProperty: Option<unsafe extern "C" fn(this: *mut c_void, a_property: *mut c_void)>, // 27
    pub SetMaterialNeedsUpdate:
        Option<unsafe extern "C" fn(this: *mut c_void, a_needs_update: bool)>, // 28
    pub SetDefaultMaterialNeedsUpdateFlag:
        Option<unsafe extern "C" fn(this: *mut c_void, a_flag: bool)>, // 29
    pub GetObjectByName:
        Option<unsafe extern "C" fn(this: *mut c_void, a_name: *mut c_void) -> *mut c_void>, // 2A
    pub SetSelectiveUpdateFlags: Option<
        unsafe extern "C" fn(
            this: *mut c_void,
            a_selective_update: bool,
            a_selective_update_transforms: bool,
            a_rigid: bool,
        ),
    >, // 2B
    pub UpdateDownwardPass:
        Option<unsafe extern "C" fn(this: *mut c_void, a_data: *mut c_void, a_arg2: u32)>, // 2C
    pub UpdateSelectedDownwardPass:
        Option<unsafe extern "C" fn(this: *mut c_void, a_data: *mut c_void, a_arg2: u32)>, // 2D
    pub UpdateRigidDownwardPass:
        Option<unsafe extern "C" fn(this: *mut c_void, a_data: *mut c_void, a_arg2: u32)>, // 2E
    pub UpdateWorldBound: Option<unsafe extern "C" fn(this: *mut c_void)>, // 2F
    pub UpdateWorldData: Option<unsafe extern "C" fn(this: *mut c_void, a_data: *mut c_void)>, // 30
    pub UpdateTransformAndBounds:
        Option<unsafe extern "C" fn(this: *mut c_void, a_data: *mut c_void)>, // 31
    pub PreAttachUpdate:
        Option<unsafe extern "C" fn(this: *mut c_void, a_parent: *mut c_void, a_data: *mut c_void)>, // 32
    pub PostAttachUpdate: Option<unsafe extern "C" fn(this: *mut c_void)>, // 33
    pub OnVisible: Option<unsafe extern "C" fn(this: *mut c_void, a_process: *mut c_void)>, // 34
}

#[repr(C)]
#[derive(Debug)]
pub enum NiAVObject_Flag {
    None = 0,
    Hidden = 1 << 0,
    SelectiveUpdate = 1 << 1,
    SelectiveUpdateTransforms = 1 << 2,
    SelectiveUpdateController = 1 << 3,
    SelectiveUpdateRigid = 1 << 4,
    DisplayObject = 1 << 5,
    DisableSorting = 1 << 6,
    SelectiveUpdateTransformsOverride = 1 << 7,
    SaveExternalGeometryData = 1 << 9,
    NoDecals = 1 << 10,
    AlwaysDraw = 1 << 11,
    MeshLOD = 1 << 12,
    FixedBound = 1 << 13,
    TopFadeNode = 1 << 14,
    IgnoreFade = 1 << 15,
    NoAnimSyncX = 1 << 16,
    NoAnimSyncY = 1 << 17,
    NoAnimSyncZ = 1 << 18,
    NoAnimSyncS = 1 << 19,
    NoDismember = 1 << 20,
    NoDismemberValidity = 1 << 21,
    RenderUse = 1 << 22,
    MaterialsApplied = 1 << 23,
    HighDetail = 1 << 24,
    ForceUpdate = 1 << 25,
    PreProcessedNode = 1 << 26,
}

impl crate::re::NiSmartPointer::RefCountable for NiAVObject {
    #[inline]
    fn inc_ref_count(&self) {
        self.__base.__base.__base.inc_ref_count();
    }

    #[inline]
    fn dec_ref_count(&self) {
        self.__base.__base.__base.dec_ref_count();
    }
}
