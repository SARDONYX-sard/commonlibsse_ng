use core::ffi::CStr;
use core::str;

use crate::re::Actor::Actor;
use crate::re::BSTList::BSSimpleList;
use crate::re::EnchantmentItem::EnchantmentItem;
use crate::re::ExtraCharge::ExtraCharge;
use crate::re::ExtraDataList::ExtraDataList;
use crate::re::ExtraEnchantment::ExtraEnchantment;
use crate::re::GameSettingCollection::GameSettingCollection;
use crate::re::Setting::SettingValue;
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::TESBox::TESBox;
use crate::re::TESEnchantableForm::TESEnchantableForm;
use crate::re::TESForm::TESForm;
use crate::re::{AlchemyItem, SoulLevel};

#[derive(Debug, Clone)]
pub struct InventoryEntryData {
    pub object: *mut TESBoundObject,
    pub extraLists: Option<TESBox<BSSimpleList<ExtraDataList>>>,
    pub countDelta: i32,
    pub pad14: u32,
}
const_assert_eq!(core::mem::size_of::<InventoryEntryData>(), 0x18);

impl InventoryEntryData {
    #[inline]
    pub const fn new(object: *mut TESBoundObject, count_delta: i32) -> Self {
        Self { object, extraLists: None, countDelta: count_delta, pad14: 0 }
    }

    #[inline]
    pub fn add_extra_list(&mut self, extra: ExtraDataList) {
        self.extraLists.get_or_insert_default().push_front(extra);
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

    pub fn get_display_name(&mut self) -> Option<&CStr> {
        let object = unsafe { self.object.as_ref() }?;
        if let Some(extra_lists) = &mut self.extraLists {
            for list in extra_lists.iter_mut() {
                if let Some(name) = list.get_display_name(object) {
                    return Some(name);
                };
            }
        }
        None
    }

    pub fn get_missing_display_name(&self) -> Option<&str> {
        let game_settings = GameSettingCollection::get_singleton()?;
        let ni_map = &game_settings.__base.settings.__base.__base.__base;
        let missing_name = unsafe { ni_map.get(&c"sMissingName".as_ptr())?.as_ref() }?;
        if let SettingValue::String(string) = missing_name.get_value() {
            return str::from_utf8(string.to_bytes()).ok();
        }

        None
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15788, ae_id = 16026)]
    pub fn get_enchantment(&self) -> *mut EnchantmentItem {}

    pub fn get_enchantment_charge(&self) -> Option<f64> {
        let obj = self.get_object()?;
        let enchantable_form =
            unsafe { (obj as *const TESBoundObject).cast::<TESEnchantableForm>().as_ref()? };

        let form_enchanting = !enchantable_form.formEnchanting.is_null();
        let amount_of_enchantment = enchantable_form.amountOfEnchantment;

        // First, if base form has enchantment and a non-zero amount, return 100.0
        if form_enchanting && amount_of_enchantment != 0 {
            return Some(100.0);
        }

        if let Some(extra_lists) = &self.extraLists {
            for ex_list in extra_lists.iter() {
                let x_charge = ex_list.get_by_type_as::<ExtraCharge>();
                let x_enchant = ex_list.get_by_type_as::<ExtraEnchantment>();

                match unsafe { (x_enchant.map(|e| e.as_ref()), x_charge.map(|c| c.as_ref())) } {
                    // If ExtraEnchantment exists and has a valid enchantment and non-zero charge
                    // AND ExtraCharge exists, compute the charge ratio
                    (Some(x_enchant), Some(x_charge))
                        if x_enchant.enchantment.is_some() && x_enchant.charge != 0 =>
                    {
                        return Some((x_charge.charge as f64 / x_enchant.charge as f64) * 100.0);
                    }
                    // If ExtraEnchantment exists (valid), but ExtraCharge is missing, fallback to 100%
                    (Some(x_enchant), None)
                        if x_enchant.enchantment.is_some() && x_enchant.charge != 0 =>
                    {
                        return Some(100.0);
                    }
                    // If only ExtraCharge exists, and base form is enchantable, compute using base enchantment amount
                    (None, Some(x_charge)) if form_enchanting && amount_of_enchantment != 0 => {
                        return Some(
                            (x_charge.charge as f64 / amount_of_enchantment as f64) * 100.0,
                        );
                    }
                    _ => {}
                }
            }
        }

        None
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

    #[inline]
    pub fn get_weight(&self) -> Option<f32> {
        Some(unsafe { self.object.as_ref() }?.__base.__base.get_weight())
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

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15782, ae_id = 16020)]
    pub fn is_owned_by(&self, test_owner: *mut Actor, default_to: bool) -> bool {}

    pub fn is_quest_object(&self) -> bool {
        todo!()
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15786, ae_id = 16024)]
    pub fn poison_object(&mut self, alchemy_item: *mut AlchemyItem, count: u32) {}
}
