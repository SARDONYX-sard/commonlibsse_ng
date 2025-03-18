use std::option::Option;
use std::sync::Arc;

use crate::re::BSTList::BSSimpleList;
use crate::re::ExtraDataList::ExtraDataList;
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::TESForm::TESForm;

#[derive(Debug, Clone)]
pub struct EnchantmentItem; // Placeholder for EnchantmentItem
#[derive(Debug, Clone)]
pub struct Actor; // Placeholder for Actor
#[derive(Debug, Clone)]
pub struct AlchemyItem; // Placeholder for AlchemyItem

#[derive(Debug, Clone)]
pub struct InventoryEntryData {
    pub object: *mut TESBoundObject,
    pub extra_lists: *mut BSSimpleList<ExtraDataList>,
    pub count_delta: i32,
    #[allow(unused)]
    pad14: u32,
}

const_assert_eq!(core::mem::size_of::<InventoryEntryData>(), 0x18);

impl InventoryEntryData {
    pub const fn new(object: *mut TESBoundObject, count_delta: i32) -> Self {
        Self { object, extra_lists: core::ptr::null_mut(), count_delta, pad14: 0 }
    }

    pub fn add_extra_list(&mut self, extra: ExtraDataList) {
        if self.extra_lists.is_null() {
            self.extra_lists = Box::into_raw(Box::new(BSSimpleList::new()));
        }
        unsafe { self.extra_lists.as_mut().map(|list| list.push_front(extra)) };
    }

    pub fn can_item_be_taken(
        &self,
        no_equipped: bool,
        no_favorited: bool,
        no_quest_item: bool,
    ) -> bool {
        todo!()
    }

    pub fn get_display_name(&self) -> &str {
        todo!()
    }

    pub fn get_enchantment(&self) -> Option<&EnchantmentItem> {
        todo!()
    }

    pub fn get_enchantment_charge(&self) -> Option<f64> {
        todo!()
    }

    pub fn get_object(&self) -> Option<&TESBoundObject> {
        unsafe { self.object.as_ref().map(|obj| obj) }
    }

    pub fn get_owner(&self) -> Option<&TESForm> {
        todo!()
    }

    pub fn get_soul_level(&self) -> Option<SoulLevel> {
        todo!()
    }

    pub fn get_value(&self) -> i32 {
        todo!()
    }

    pub fn get_weight(&self) -> f32 {
        todo!()
    }

    pub fn is_enchanted(&self) -> bool {
        todo!()
    }

    pub fn is_favorited(&self) -> bool {
        todo!()
    }

    pub fn is_leveled(&self) -> bool {
        todo!()
    }

    pub fn is_poisoned(&self) -> bool {
        todo!()
    }

    pub fn is_worn(&self) -> bool {
        todo!()
    }

    pub fn is_owned_by(&self, test_owner: Option<Arc<Actor>>, default_to: bool) -> bool {
        // Placeholder for ownership check
        default_to
    }

    pub fn is_quest_object(&self) -> bool {
        todo!()
    }

    pub fn poison_object(&mut self, alch_item: Option<Arc<AlchemyItem>>, count: u32) {
        // Placeholder for poisoning object
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SoulLevel {
    // Placeholder for SoulLevel enum
}
