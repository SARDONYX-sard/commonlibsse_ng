use crate::re::Actor::Actor;
use crate::re::BSTList::BSSimpleList;
use crate::re::ExtraDataList::ExtraDataList;
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::TESForm::TESForm;
use crate::re::{AlchemyItem, EnchantmentItem, SoulLevel};

#[derive(Debug, Clone)]
pub struct InventoryEntryData {
    pub object: *mut TESBoundObject,
    pub extraLists: *mut BSSimpleList<ExtraDataList>,
    pub countDelta: i32,
    pub pad14: u32,
}
const_assert_eq!(core::mem::size_of::<InventoryEntryData>(), 0x18);

impl InventoryEntryData {
    pub const fn new(object: *mut TESBoundObject, count_delta: i32) -> Self {
        Self { object, extraLists: core::ptr::null_mut(), countDelta: count_delta, pad14: 0 }
    }

    pub fn add_extra_list(&mut self, extra: ExtraDataList) {
        if self.extraLists.is_null() {
            self.extraLists = Box::into_raw(Box::new(BSSimpleList::new()));
        }
        unsafe { self.extraLists.as_mut().map(|list| list.push_front(extra)) };
    }

    pub fn can_item_be_taken(
        &self,
        no_equipped: bool,
        no_favorited: bool,
        no_quest_item: bool,
    ) -> bool {
        let _ = no_equipped;
        let _ = no_favorited;
        let _ = no_quest_item;
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

    pub const fn get_object(&self) -> Option<&TESBoundObject> {
        unsafe { self.object.as_ref() }
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

    pub fn is_owned_by(&self, test_owner: *mut Actor, default_to: bool) -> bool {
        let _ = test_owner;
        let _ = default_to;
        todo!()
    }

    pub fn is_quest_object(&self) -> bool {
        todo!()
    }

    pub fn poison_object(&mut self, alchemy_item: *mut AlchemyItem, count: u32) {
        let _ = alchemy_item;
        let _ = count;
        todo!()
    }
}
