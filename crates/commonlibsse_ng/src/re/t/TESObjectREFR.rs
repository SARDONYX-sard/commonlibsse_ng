use core::ffi::c_void;
use core::ptr::NonNull;
use std::collections::HashMap;

use crate::re::ExtraDataList::ExtraDataList;
use crate::re::InventoryEntryData::InventoryEntryData;
use crate::re::NiPoint3::NiPoint3;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// dummy
struct TESBoundObject;
struct InventoryChanges;
impl InventoryChanges {
    pub fn init_leveled_items(&self) {
        todo!()
    }
    pub fn init_from_container_extra(&self) {
        todo!()
    }
    pub fn init_scripts(&self) {
        todo!()
    }
}

struct ExtraContainerChanges {
    changes: *mut InventoryChanges,
}

struct BSTSmallArray<T> {
    pub data: *mut T,
    pub size: usize,
    pub capacity: usize,
}
struct TesWaterForm;
struct NiPointer<T> {
    pub ptr: NonNull<T>,
}
struct NiAVObject;

struct TESObjectCELL;
struct TESForm;
struct BSHandleRefObject;
struct BSTEventSink<T> {
    maker: core::marker::PhantomData<T>,
}
struct BSAnimationGraphEvent;
struct IAnimationGraphManagerHolder;

enum ItemRemoveReason {
    Remove,
    Steal,
    Selling,
    Dropping,
    StoreInContainer,
    StoreInTeammate,
}
struct ObjectHandle;
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct OBJ_REFR {
    objectReference: *mut TESBoundObject, // 00
    angle: NiPoint3,                      // 08
    location: NiPoint3,                   // 14
}

const _: () = {
    assert!(core::mem::size_of::<OBJ_REFR>() == 0x20);
};

#[repr(C)]
pub struct LOADED_REF_DATA {
    pub unk00: BSTSmallArray<*mut c_void>, // handleList?
    pub current_water_type: Option<NonNull<TesWaterForm>>,
    pub relevant_water_height: f32,
    pub cached_radius: f32,
    pub flags: u16,
    pub underwater_count: i16,
    pub unk30: u64,
    pub unk38: u64,
    pub unk40: u64,
    pub unk48: u64,
    pub unk50: u64,
    pub unk58: u64,
    pub unk60: u64,
    pub data_3d: NiPointer<NiAVObject>,
    pub unk70: Option<NonNull<c_void>>, // smart ptr
}

struct TESObjectREFR {
    _base: TESForm,                              // 00
    _base1: BSHandleRefObject,                   // 20
    _base2: BSTEventSink<BSAnimationGraphEvent>, // 30
    _base3: IAnimationGraphManagerHolder,        // 38
    data: OBJ_REFR,                              // 40
    parentCell: *mut TESObjectCELL,              // 60
    loadedData: *mut LOADED_REF_DATA,            // 68
    extraList: ExtraDataList,                    // 70
}

type Count = i32;
pub type InventoryItemMap = HashMap<*mut TESBoundObject, Count>;
pub type InventoryCountMap = HashMap<*mut TESBoundObject, (Count, Box<InventoryEntryData>)>;
pub type InventoryDropMap = HashMap<*mut TESBoundObject, (Count, Vec<ObjectHandle>)>;

impl TESObjectREFR {
    // fn create_reference(
    //     _handle_out: &mut ObjectRefHandle,
    //     _form_type: FormType,
    //     _add_actor_to_process_list: bool,
    // ) {
    //     unimplemented!()
    // }

    // fn lookup_by_handle(_ref_handle: RefHandle) -> Option<Box<TESObjectREFR>> {
    //     unimplemented!()
    // }

    // fn lookup_by_handle_out(
    //     _ref_handle: RefHandle,
    //     _refr_out: &mut Option<Box<TESObjectREFR>>,
    // ) -> bool {
    //     unimplemented!()
    // }

    // fn find_reference_for_3d(_object_3d: &NiAVObject) -> Option<&TESObjectREFR> {
    //     unimplemented!()
    // }

    // fn activate_ref(
    //     &self,
    //     _activator: &TESObjectREFR,
    //     _arg2: u8,
    //     _object: &TESBoundObject,
    //     _count: i32,
    //     _default_processing_only: bool,
    // ) -> bool {
    //     unimplemented!()
    // }

    // fn apply_art_object(
    //     &self,
    //     _art_object: &BGSArtObject,
    //     _duration: f32,
    //     _facing_ref: Option<&TESObjectREFR>,
    //     _face_target: bool,
    //     _attach_to_camera: bool,
    //     _attach_node: Option<&NiAVObject>,
    //     _interface_effect: bool,
    // ) -> ModelReferenceEffect {
    //     unimplemented!()
    // }

    // fn apply_effect_shader(
    //     &self,
    //     _effect_shader: &TESEffectShader,
    //     _duration: f32,
    //     _facing_ref: Option<&TESObjectREFR>,
    //     _face_target: bool,
    //     _attach_to_camera: bool,
    //     _attach_node: Option<&NiAVObject>,
    //     _interface_effect: bool,
    // ) -> ShaderReferenceEffect {
    //     unimplemented!()
    // }

    // fn can_be_moved(&self) -> bool {
    //     unimplemented!()
    // }

    // fn create_ref_handle(&self) -> ObjectRefHandle {
    //     unimplemented!()
    // }

    // fn do_trap_data(&self, _data: &TrapData) {
    //     unimplemented!()
    // }

    // fn do_trap_entry(&self, _trap: &TrapEntry, _target: &TargetEntry) {
    //     unimplemented!()
    // }

    // fn enable(&self, _reset_inventory: bool) {
    //     unimplemented!()
    // }

    // fn get_3d(&self, _first_person: bool) -> Option<&NiAVObject> {
    //     unimplemented!()
    // }

    // fn get_actor_owner(&self) -> Option<&TESNPC> {
    //     unimplemented!()
    // }

    // fn get_angle(&self) -> NiPoint3 {
    //     unimplemented!()
    // }

    // fn get_angle_x(&self) -> f32 {
    //     unimplemented!()
    // }

    // fn get_angle_y(&self) -> f32 {
    //     unimplemented!()
    // }

    // fn get_angle_z(&self) -> f32 {
    //     unimplemented!()
    // }

    // fn get_base_height(&self) -> f32 {
    //     unimplemented!()
    // }

    // fn get_base_object(&self) -> Option<&TESBoundObject> {
    //     unimplemented!()
    // }

    // fn get_biped(&self, _first_person: bool) -> Option<&BSTSmartPointer<BipedAnim>> {
    //     unimplemented!()
    // }

    // fn get_calc_level(&self, _adjust_level: bool) -> u16 {
    //     unimplemented!()
    // }

    // fn get_container(&self) -> Option<&TESContainer> {
    //     unimplemented!()
    // }

    // fn get_current_location(&self) -> Option<&BGSLocation> {
    //     unimplemented!()
    // }

    // fn get_display_full_name(&self) -> Option<&str> {
    //     unimplemented!()
    // }

    // fn get_distance(
    //     &self,
    //     _other: &TESObjectREFR,
    //     _disabled_refs: bool,
    //     _ignore_worldspace: bool,
    // ) -> f32 {
    //     unimplemented!()
    // }

    // fn get_dropped_inventory(&self) -> InventoryDropMap {
    //     unimplemented!()
    // }

    // fn get_editor_location(&self) -> Option<BGSLocation> {
    //     unimplemented!()
    // }

    // fn get_enchantment_charge(&self) -> Option<f64> {
    //     unimplemented!()
    // }

    // fn get_faction_owner(&self) -> Option<&TESFaction> {
    //     unimplemented!()
    // }

    // fn get_handle(&self) -> ObjectRefHandle {
    //     unimplemented!()
    // }

    // fn get_heading_angle(&self, _pos: &NiPoint3, _abs: bool) -> f32 {
    //     unimplemented!()
    // }

    // fn get_height(&self) -> f32 {
    //     unimplemented!()
    // }

    // fn get_inventory(&self) -> InventoryItemMap {
    //     unimplemented!()
    // }

    fn get_inventory_filter<F>(&self, filter: F, no_init: bool) -> InventoryItemMap
    where
        F: Fn(&mut TESBoundObject) -> bool,
    {
        let inventory_changed = self.get_inventory_changes(no_init);
        unimplemented!()
    }

    // fn get_inventory_counts(&self) -> InventoryCountMap {
    //     unimplemented!()
    // }

    fn get_inventory_changes(&self, no_init: bool) -> Option<*mut InventoryChanges> {
        if !self.extraList.has_type::<ExtraContainerChanges>() {
            if no_init {
                return None;
            };

            if !self.init_inventory_if_required(false) {
                self.force_init_inventory_changes();
            }
        }
        let x_count_changes = self.extraList.get_by_type2::<ExtraContainerChanges>();
        if !x_count_changes.is_null() {
            return Some(unsafe { &*x_count_changes }.changes);
        };

        None
    }

    /// # Panics
    /// If failed to resolve this method's address.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15800, ae_id = 16038)]
    pub fn init_inventory_if_required(&self, ignore_container_extra_data: bool) -> bool {
        type S = SelfSignature;
    }

    pub fn force_init_inventory_changes(&self) -> *mut InventoryChanges {
        let changes = self.make_inventory_changes();
        if !changes.is_null() {
            let changes = unsafe { &*changes };
            changes.init_leveled_items();
            changes.init_from_container_extra();
            changes.init_scripts();
        }
        changes
    }

    /// # Panics
    /// If failed to resolve this method's address.
    pub fn make_inventory_changes(&self) -> *mut InventoryChanges {
        type SelfSignature = fn(this: *const ()) -> *mut InventoryChanges;

        {
            static FUNC: std::sync::LazyLock<SelfSignature> = std::sync::LazyLock::new(|| {
                use crate::rel::ResolvableAddress as _;
                use crate::rel::id::RelocationID;

                const SE_ID: u64 = 15802;
                const AE_ID: u64 = 16040;

                let fn_ptr =
                    RelocationID::new(SE_ID, AE_ID, SE_ID).address().unwrap_or_else(|err| {
                        #[cfg(feature = "tracing")]
                        tracing::error!("[Critical Error] Failed to resolve address: {err}");
                        panic!("Failed to resolve address: {err}")
                    });
                unsafe { core::mem::transmute::<NonNull<c_void>, SelfSignature>(fn_ptr) }
            });
            FUNC((self as *const Self).cast())
        }
    }
}
