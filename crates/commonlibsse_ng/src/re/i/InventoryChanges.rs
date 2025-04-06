use core::ffi::c_void;

use crate::re::BSTList::BSSimpleList;
use crate::re::ExtraDataList::ExtraDataList;
use crate::re::InventoryEntryData::InventoryEntryData;
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::TESForm::TESForm;
use crate::re::TESObjectREFR::TESObjectREFR;
use crate::re::offsets_rtti::RTTI_InventoryChanges__IItemChangeVisitor;
use crate::re::offsets_vtable::VTABLE_InventoryChanges__IItemChangeVisitor;
use crate::rel::id::VariantID;

struct TESObjectARMO;
struct BGSOutfit;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[repr(u32)]
#[derive(Debug)]
pub enum VisitResult {
    Stop = 0,
    Continue = 1,
}

pub struct IItemChangeVisitor {
    pub vptr: *const IItemChangeVisitorVtbl,
}

impl IItemChangeVisitor {
    pub const RTTI: VariantID = RTTI_InventoryChanges__IItemChangeVisitor;
    pub const VTABLE: [VariantID; 1] = VTABLE_InventoryChanges__IItemChangeVisitor;
}

pub struct IItemChangeVisitorVtbl {
    /// C++ virtual destructor
    pub _drop: fn(this: *mut c_void),

    pub visit: fn(this: *mut c_void, entry_data: &mut InventoryEntryData) -> VisitResult,
    pub should_visit:
        fn(this: *const c_void, entry_data: &InventoryEntryData, object: &TESBoundObject) -> bool,
    pub unk_03: fn(
        this: *mut c_void,
        entry_data: &mut InventoryEntryData,
        arg2: *mut c_void,
        arg3: &mut bool,
    ) -> VisitResult,
}

pub struct InventoryChanges {
    pub entry_list: *mut BSSimpleList<*mut InventoryEntryData>,
    pub owner: *mut TESObjectREFR,
    pub total_weight: f32,
    pub armor_weight: f32,
    pub changed: bool,
    pub unk19: u8,
    pub unk1a: u8,
    pub unk1b: u8,
    pub unk1c: u32,
}

impl InventoryChanges {
    /// C++ Constructor
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15812, ae_id = 16050)]
    #[allow(clippy::use_self)]
    fn new(ref_: *mut TESObjectREFR) -> InventoryChanges {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15813, ae_id = 16051)]
    /// C++ Destructor
    fn manually_drop(&mut self) {}

    #[allow(clippy::missing_const_for_fn)]
    pub fn add_entry_data(&mut self, entry: *mut InventoryEntryData) {
        // self.entry_list.push_front(entry); // move take?
        let _ = entry;
        self.changed = true;
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15829, ae_id = 16068)]
    pub fn generate_leveled_list_changes(&mut self) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15873, ae_id = 16113)]
    pub fn get_armor_in_slot(&self, slot: i32) -> *mut TESObjectARMO {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15883, ae_id = 16123)]
    pub fn get_inventory_weight(&self) -> f32 {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15868, ae_id = 16047)]
    pub fn get_item_count(&self, obj: *mut TESBoundObject) -> i16 {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15908, ae_id = 16148)]
    pub fn get_next_unique_id(&self) -> u16 {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15859, ae_id = 16099)]
    pub fn get_worn_mask(&self) -> u32 {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15890, ae_id = 16130)]
    pub fn init_from_container_extra(&self) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15889, ae_id = 16129)]
    pub fn init_leveled_items(&self) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15833, ae_id = 16072)]
    pub fn init_outfit_items(&self, outfit: *mut BGSOutfit, npc_level: u16) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15829, ae_id = 16068)]
    pub fn init_scripts(&self) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15859, ae_id = 16099)]
    pub fn remove_favorite(
        &mut self,
        entry: *mut InventoryEntryData,
        item_list: *mut ExtraDataList,
    ) {
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15878, ae_id = 441567)]
    pub fn remove_all_items(
        &mut self,
        ref_: *mut TESObjectREFR,
        move_to_ref: *mut TESObjectREFR,
        arg4: bool,
        keep_ownership: bool,
        arg6: bool,
    ) {
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15909, ae_id = 16149)]
    pub fn send_container_changed_event(
        &mut self,
        item_extrlist: *mut ExtraDataList,
        from_refr: *mut TESObjectREFR,
        item: *mut TESForm,
        count: i32,
    ) {
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15858, ae_id = 16098)]
    pub fn set_favorite(&mut self, entry: *mut InventoryEntryData, item_list: *mut ExtraDataList) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15907, ae_id = 16149)]
    pub fn set_unique_id(
        &mut self,
        item_list: *mut ExtraDataList,
        old_form: *mut TESForm,
        new_form: *mut TESForm,
    ) {
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15855, ae_id = 16095)]
    pub fn visit_inventory(&mut self, visitor: &mut IItemChangeVisitor) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15856, ae_id = 16096)]
    pub fn visit_worn_items(&mut self, visitor: &mut IItemChangeVisitor) {}
}
