use crate::re::NiBound::NiBound;
use crate::re::NiCollisionObject::NiCollisionObject;
use crate::re::NiObjectNET::NiObjectNET;
use crate::re::NiSmartPointer::NiPointer;
use crate::re::NiTransform::NiTransform;
use crate::re::offsets_ni_rtti::NiRTTI_NiObject;
use crate::re::offsets_rtti::RTTI_NiObject;
use crate::re::offsets_vtable::VTABLE_NiObject;
use crate::re::{NiNode, bhkCollisionObject};
use crate::rel::id::VariantID;
use crate::rel::relocation::{RelocationError, relocate_member, relocate_member_mut};
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
    pub CxxDrop: unsafe extern "C" fn(this: *mut c_void), // 00
    /// C++ operator()
    pub CxxOperatorCall: unsafe extern "C" fn(*mut c_void, *mut NiAVObject) -> bool, // 01
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

impl NiAVObject {
    pub const RTTI: VariantID = RTTI_NiObject;
    pub const NI_RTTI: VariantID = NiRTTI_NiObject;
    pub const VTABLE: [VariantID; 1] = VTABLE_NiObject;

    #[must_use]
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 68835, ae_id = 70187)]
    pub fn clone(&self) -> *mut NiAVObject {}

    // pub fn cull_geometry(&mut self, cull: bool) {
    //     todo!()
    // }

    // pub fn cull_node(&mut self, cull: bool) {
    //     todo!()
    // }

    /// Returns whether the object is culled (hidden) in the application.
    ///
    /// This function checks the visibility status of the object based on its flags.
    /// If the object has the [`Flag::Hidden`] set, it is considered culled (i.e., hidden) and will
    /// not be rendered or processed for certain operations.
    ///
    /// The `get_app_culled` function essentially checks if the object is flagged as "hidden" by
    /// returning a boolean indicating whether the object is culled or not. This is commonly
    /// used for optimizing rendering performance by excluding objects that should not be visible.
    ///
    /// # Returns
    /// * `true` if the object is culled (i.e., hidden).
    /// * `false` if the object is not culled (i.e., visible).
    #[inline]
    pub fn get_app_culled(&self) -> bool {
        self.get_flags().is_ok_and(|flag_member| flag_member.contains(Flag::Hidden))
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 25482, ae_id = 26022)]
    pub fn get_collision_object(&self) -> *mut bhkCollisionObject {}

    // pub fn get_collision_layer(&self) -> COL_LAYER {
    //     self.collision_object.is_some()
    // }

    // #[must_use]
    // pub fn get_first_geometry_of_shader_type(
    //     &self,
    //     shader_type: BSShaderMaterialFeature,
    // ) -> Option<NonNull<BSGeometry>> {
    //     todo!()
    // }

    // #[must_use]
    // pub fn get_mass(&self) -> f32 {
    //     todo!()
    // }

    // #[must_use]
    // pub fn get_user_data(&self) -> Option<NonNull<TESObjectREFR>> {
    //     todo!()
    // }

    // pub fn set_user_data(&mut self, ref_obj: Option<NonNull<TESObjectREFR>>) {
    //     todo!()
    // }

    // #[must_use]
    // pub fn has_animation(&self) -> bool {
    //     todo!()
    // }

    // #[must_use]
    // pub fn has_shader_type(&self, shader_type: BSShaderMaterialFeature) -> bool {
    //     todo!()
    // }

    // pub fn remove_decals(&mut self) {
    //     todo!()
    // }

    // pub fn set_app_culled(&mut self, cull: bool) {
    //     todo!()
    // }

    // pub fn set_collision_layer(&mut self, layer: COL_LAYER) {
    //     todo!()
    // }

    // pub fn set_collision_layer_and_group(&mut self, layer: COL_LAYER, group: u32) {
    //     todo!()
    // }

    // #[must_use]
    // pub fn set_motion_type(
    //     &mut self,
    //     motion_type: hkpMotionType,
    //     recurse: bool,
    //     force: bool,
    //     allow_activate: bool,
    // ) -> bool {
    //     todo!()
    // }

    // #[must_use]
    // pub fn set_projected_uv_data(
    //     &mut self,
    //     uv_params: &NiColorA,
    //     uv_color: &NiColor,
    //     is_snow: bool,
    // ) -> bool {
    //     todo!()
    // }

    // pub fn tint_scenegraph(&mut self, color: &NiColorA) {
    //     todo!()
    // }

    // pub fn update(&mut self, data: &mut NiUpdateData) {
    //     todo!()
    // }

    // pub fn update_body_tint(&mut self, color: &NiColor) {
    //     todo!()
    // }

    // pub fn update_hair_color(&mut self, color: &NiColor) {
    //     todo!()
    // }

    // pub fn update_material_alpha(&mut self, alpha: f32, do_only_skin: bool) {
    //     todo!()
    // }

    // pub fn update_rigid_constraints(&mut self, enable: bool, arg2: u8, arg3: u32) {
    //     todo!()
    // }

    // #[must_use]
    // pub fn get_flags_mut(&mut self) -> &mut stl::Enumeration<Flag, u32> {
    //     unsafe {
    //         std::mem::transmute(
    //             ((self as *mut Self).cast::<u8>().add(0x0F4)) as *mut stl::Enumeration<Flag, u32>,
    //         )
    //     }
    // }

    /// # Errors
    #[inline]
    pub fn get_flags(&self) -> Result<&Flag, RelocationError> {
        relocate_member(self, 0x0F4, 0x10C)
    }

    /// # Errors
    #[inline]
    pub fn get_flags_mut(&mut self) -> Result<&mut Flag, RelocationError> {
        relocate_member_mut(self, 0x0F4, 0x10C)
    }

    // pub fn temp_nicast(
    //     &mut self,
    //     geometry: &mut BSGeometry,
    // ) -> Option<NonNull<BSLightingShaderProperty>> {
    //     todo!()
    // }
}

impl crate::re::NiSmartPointer::RefCountable for NiAVObject {
    #[inline]
    fn inc_ref_count(&self) {
        self.__base.__base.__base.inc_ref_count();
    }

    #[inline]
    fn dec_ref_count(&mut self) {
        self.__base.__base.__base.dec_ref_count();
    }
}

#[repr(C)]
pub struct NiAVObjectVtbl {
    /// C++ class Destructor equivalent
    pub cxx_drop: unsafe extern "C" fn(this: *mut c_void), // 00

    // override (NiObjectNET)
    pub GetRTTI: unsafe extern "C" fn(this: *const c_void) -> *const c_void, // 02
    pub LoadBinary: unsafe extern "C" fn(this: *mut c_void, stream: *mut c_void), // 18
    pub LinkObject: unsafe extern "C" fn(this: *mut c_void, stream: *mut c_void), // 19
    pub RegisterStreamables: unsafe extern "C" fn(this: *mut c_void, stream: *mut c_void) -> bool, // 1A
    pub SaveBinary: unsafe extern "C" fn(this: *mut c_void, stream: *mut c_void), // 1B
    pub IsEqual: unsafe extern "C" fn(this: *mut c_void, object: *mut c_void) -> bool, // 1C
    pub ProcessClone: unsafe extern "C" fn(this: *mut c_void, cloning: *mut c_void), // 1D

    // Custom add-ons
    pub UpdateControllers: unsafe extern "C" fn(this: *mut c_void, data: *mut c_void), // 25

    // VR
    pub ApplyLocalTransformToWorld: Option<unsafe extern "C" fn(this: *mut c_void)>, // Optional, for VR-specific functionality
    pub PerformOp: Option<unsafe extern "C" fn(this: *mut c_void, func: *mut c_void)>, // 26
    pub AttachProperty: Option<unsafe extern "C" fn(this: *mut c_void, property: *mut c_void)>, // 27
    pub SetMaterialNeedsUpdate: Option<unsafe extern "C" fn(this: *mut c_void, needs_update: bool)>, // 28
    pub SetDefaultMaterialNeedsUpdateFlag:
        Option<unsafe extern "C" fn(this: *mut c_void, flag: bool)>, // 29
    pub GetObjectByName:
        Option<unsafe extern "C" fn(this: *mut c_void, name: *mut c_void) -> *mut c_void>, // 2A
    pub SetSelectiveUpdateFlags: Option<
        unsafe extern "C" fn(
            this: *mut c_void,
            a_selective_update: bool,
            a_selective_update_transforms: bool,
            a_rigid: bool,
        ),
    >, // 2B
    pub UpdateDownwardPass:
        Option<unsafe extern "C" fn(this: *mut c_void, data: *mut c_void, arg2: u32)>, // 2C
    pub UpdateSelectedDownwardPass:
        Option<unsafe extern "C" fn(this: *mut c_void, data: *mut c_void, arg2: u32)>, // 2D
    pub UpdateRigidDownwardPass:
        Option<unsafe extern "C" fn(this: *mut c_void, data: *mut c_void, arg2: u32)>, // 2E
    pub UpdateWorldBound: Option<unsafe extern "C" fn(this: *mut c_void)>, // 2F
    pub UpdateWorldData: Option<unsafe extern "C" fn(this: *mut c_void, data: *mut c_void)>, // 30
    pub UpdateTransformAndBounds:
        Option<unsafe extern "C" fn(this: *mut c_void, data: *mut c_void)>, // 31
    pub PreAttachUpdate:
        Option<unsafe extern "C" fn(this: *mut c_void, parent: *mut c_void, data: *mut c_void)>, // 32
    pub PostAttachUpdate: Option<unsafe extern "C" fn(this: *mut c_void)>, // 33
    pub OnVisible: Option<unsafe extern "C" fn(this: *mut c_void, process: *mut c_void)>, // 34
}

bitflags::bitflags! {
    /// NiAVObject Flags
    #[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Flag: u32 {
        const None = 0;
        const Hidden = 1 << 0;
        const SelectiveUpdate = 1 << 1;
        const SelectiveUpdateTransforms = 1 << 2;
        const SelectiveUpdateController = 1 << 3;
        const SelectiveUpdateRigid = 1 << 4;
        const DisplayObject = 1 << 5;
        const DisableSorting = 1 << 6;
        const SelectiveUpdateTransformsOverride = 1 << 7;
        const SaveExternalGeometryData = 1 << 9;
        const NoDecals = 1 << 10;
        const AlwaysDraw = 1 << 11;
        const MeshLOD = 1 << 12;
        const FixedBound = 1 << 13;
        const TopFadeNode = 1 << 14;
        const IgnoreFade = 1 << 15;
        const NoAnimSyncX = 1 << 16;
        const NoAnimSyncY = 1 << 17;
        const NoAnimSyncZ = 1 << 18;
        const NoAnimSyncS = 1 << 19;
        const NoDismember = 1 << 20;
        const NoDismemberValidity = 1 << 21;
        const RenderUse = 1 << 22;
        const MaterialsApplied = 1 << 23;
        const HighDetail = 1 << 24;
        const ForceUpdate = 1 << 25;
        const PreProcessedNode = 1 << 26;
    }
}
