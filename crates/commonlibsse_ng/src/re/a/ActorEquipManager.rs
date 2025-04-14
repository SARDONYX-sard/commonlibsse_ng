use crate::re::Actor::Actor;
use crate::re::BGSEquipSlot::BGSEquipSlot;
use crate::re::ExtraDataList::ExtraDataList;
use crate::re::SpellItem::SpellItem;
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::TESShout::TESShout;

#[derive(Debug)]
pub struct ActorEquipManager {
    ///  - non EBO: BSTSingletonSDM<ActorEquipManager> address
    __base: [u8; 1], // 0x00
    pub unk01: bool, // 0x01
}
const _: () = assert!(core::mem::size_of::<ActorEquipManager>() == 0x2);

impl ActorEquipManager {
    /// Returns the singleton instance of `Self`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut ActorEquipManager",
        default = "None",
        deref_once,
        id(se = 514494, ae = 400636)
    )]
    pub fn get_singleton() -> Option<&'static ActorEquipManager> {
        |deref_type: DerefType| unsafe { deref_type.as_ref() }
    }

    /// Returns the mutable singleton instance of `Self`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut ActorEquipManager",
        default = "None",
        deref_once,
        id(se = 514494, ae = 400636)
    )]
    pub fn get_singleton_mut() -> Option<&'static mut ActorEquipManager> {
        |deref_type: DerefType| unsafe { deref_type.as_mut() }
    }

    #[allow(clippy::too_many_arguments)]
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 37938, ae_id = 38894)]
    pub fn equip_object(
        &mut self,
        actor: *mut Actor,
        object: *mut TESBoundObject,
        extra_data: *mut ExtraDataList,
        count: u32,
        slot: *const BGSEquipSlot,
        queue_equip: bool,
        force_equip: bool,
        play_sounds: bool,
        apply_now: bool,
    ) {
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 37941, ae_id = 38897)]
    pub fn equip_shout(&mut self, actor: *mut Actor, shout: *mut TESShout) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 37939, ae_id = 38895)]
    pub fn equip_spell(
        &mut self,
        actor: *mut Actor,
        spell: *mut SpellItem,
        slot: *mut BGSEquipSlot,
    ) {
    }

    #[allow(clippy::too_many_arguments)]
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 37945, ae_id = 38901)]
    pub fn un_equip_object(
        &mut self,
        actor: *mut Actor,
        object: *mut TESBoundObject,
        extra_data: *mut ExtraDataList,
        count: u32,
        slot: *const BGSEquipSlot,
        queue_equip: bool,
        force_equip: bool,
        play_sounds: bool,
        apply_now: bool,
        slot_replace: *const BGSEquipSlot,
    ) {
    }
}
