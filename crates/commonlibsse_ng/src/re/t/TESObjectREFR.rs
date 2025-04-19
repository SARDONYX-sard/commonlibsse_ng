mod vtable;

pub use self::vtable::TESObjectREFRVtbl;

use core::ffi::c_void;
use core::ptr::NonNull;
use std::collections::HashMap;

use crate::re::BSAnimationGraphEvent::BSAnimationGraphEvent;
use crate::re::BSHandleRefObject::BSHandleRefObject;
use crate::re::BSTArray::BSTSmallArray;
use crate::re::BSTEvent::BSTEventSink;
use crate::re::ExtraContainerChanges::ExtraContainerChanges;
use crate::re::ExtraDataList::ExtraDataList;
use crate::re::IAnimationGraphManagerHolder::IAnimationGraphManagerHolder;
use crate::re::InventoryChanges::InventoryChanges;
use crate::re::InventoryEntryData::InventoryEntryData;
use crate::re::NiAVObject::NiAVObject;
use crate::re::NiPoint3::NiPoint3;
use crate::re::NiSmartPointer::NiPointer;
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::TESForm::TESForm;
use crate::re::TESObjectCELL::TESObjectCELL;
use crate::re::{ObjectHandle, TesWaterForm};

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Debug)]
pub struct OBJ_REFR {
    pub objectReference: *mut TESBoundObject, // 00
    pub angle: NiPoint3,                      // 08
    pub location: NiPoint3,                   // 14
}

const _: () = assert!(core::mem::size_of::<OBJ_REFR>() == 0x20);

#[derive(Debug)]
#[repr(C)]
pub struct LOADED_REF_DATA {
    pub unk00: BSTSmallArray<*mut c_void>, // handleList?
    pub current_water_type: *mut TesWaterForm,
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
    pub unk70: *mut c_void, // smart ptr
}

#[repr(C)]
#[derive(Debug)]
pub struct TESObjectREFR {
    pub __base: TESForm,                              // 00
    pub __base1: BSHandleRefObject,                   // 20
    pub __base2: BSTEventSink<BSAnimationGraphEvent>, // 30
    pub __base3: IAnimationGraphManagerHolder,        // 38
    pub data: OBJ_REFR,                               // 40
    pub parentCell: *mut TESObjectCELL,               // 60
    pub loadedData: *mut LOADED_REF_DATA,             // 68
    pub extraList: ExtraDataList,                     // 70
}
const _: () = assert!(core::mem::size_of::<TESObjectREFR>() == 0x78);

impl crate::re::NiSmartPointer::RefCountable for TESObjectREFR {
    #[inline]
    fn inc_ref_count(&self) {
        self.__base1.inc_ref_count();
    }

    #[inline]
    fn dec_ref_count(&mut self) {
        self.__base1.dec_ref_count();
    }
}

type Count = i32;
pub type InventoryCountMap = HashMap<*mut TESBoundObject, Count>;
pub type InventoryItemMap = HashMap<*mut TESBoundObject, (Count, Box<InventoryEntryData>)>;
pub type InventoryDropMap = HashMap<*mut TESBoundObject, (Count, Vec<ObjectHandle>)>;

impl TESObjectREFR {
    pub fn get_inventory_filter<F>(&self, filter: F, no_init: bool) -> Option<InventoryItemMap>
    where
        F: Fn(&TESBoundObject) -> bool,
    {
        let inventory_changed = self.get_inventory_changes(no_init)?;
        let inventory_changed = unsafe { &*inventory_changed };

        let mut inventory = InventoryItemMap::new();
        for entry in unsafe { inventory_changed.entryList.as_ref()?.iter() } {
            if entry.is_null() {
                continue;
            }

            let entry_ref = unsafe { entry.as_ref()? };
            let object = entry_ref.object;
            if filter(unsafe { object.as_ref()? }) {
                inventory.insert(object, (entry_ref.countDelta, Box::new(entry_ref.clone())));
            }
        }

        Some(inventory)
    }

    // fn get_inventory_counts(&self) -> InventoryCountMap {
    //     unimplemented!()
    // }

    pub fn get_inventory_changes(&self, no_init: bool) -> Option<*mut InventoryChanges> {
        if !self.extraList.has_type(ExtraContainerChanges::EXTRA_DATA_TYPE) {
            if no_init {
                return None;
            };

            if !self.init_inventory_if_required(false) {
                self.force_init_inventory_changes();
            }
        }
        let x_container = self.extraList.get_by_type_as::<ExtraContainerChanges>()?;
        Some(unsafe { x_container.as_ref() }.changes)
    }

    #[inline]
    pub const fn get_object_reference(&self) -> *mut TESBoundObject {
        self.data.objectReference
    }

    /// # Panics
    /// Returns an error if address resolution fails.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15800, ae_id = 16038)]
    pub fn init_inventory_if_required(&self, ignore_container_extra_data: bool) -> bool {}

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
