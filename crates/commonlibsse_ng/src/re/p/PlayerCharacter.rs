pub mod crime;
pub mod grab_data;
pub mod player_target_loc;
pub mod runtime_info;
pub mod skill;
pub mod vr_node_data;

use crate::re::ActorValues::ActorValue;
use crate::re::BGSActorCellEvent::BGSActorCellEvent;
use crate::re::BGSAddToPlayerInventoryEvent::ACQUIRE_TYPE;
use crate::re::BGSTextureSet::BGSTextureSet;
use crate::re::BSCoreTypes::RefHandle;
use crate::re::BSPointerHandle::ActorHandle;
use crate::re::BSTEvent::{BSTEventSink, BSTEventSource};
use crate::re::Character::{Character, CharacterVtbl};
use crate::re::CrosshairPickData::VR_DEVICE;
use crate::re::FormTypes::FormType;
use crate::re::InventoryEntryData::InventoryEntryData;
use crate::re::MagicItem::MagicItem;
use crate::re::MenuModeChangeEvent::MenuModeChangeEvent;
use crate::re::MenuOpenCloseEvent::MenuOpenCloseEvent;
use crate::re::NiSmartPointer::NiPointer;
use crate::re::PositionPlayerEvent::PositionPlayerEvent;
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::TESForm::TESForm;
use crate::re::TESObject::TESObject;
use crate::re::TESObjectCELL::TESObjectCELL;
use crate::re::TESObjectREFR::TESObjectREFR;
use crate::re::TESTrackedStatsEvent::TESTrackedStatsEvent;
use crate::re::offsets_rtti::RTTI_PlayerCharacter;
use crate::re::offsets_vtable::VTABLE_PlayerCharacter;
use crate::re::{BGSActorDeathEvent, TESFaction, TESObjectWEAP, TESRace};
use crate::re::{Effect, MagicSystem, UserEventEnabledEvent};
use crate::rel::id::VariantID;

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum PLAYER_ACTION {
    #[default]
    None = 0,
    SwingMeleeWeapon,
    CastProjectileSpell,
    ShootBow,
    ZKeyObject,
    Jumping,
    KnockingOverObjects,
    StandOnTableChair,
    IronSights,
    DestroyObject,
    LockedObject,
    Pickpocket,
    CastSelfSpell,
    Shout,
    ActorCollision,

    Total,
    InvalidMarker,
}

#[repr(C)]
pub struct FriendshipFactionsStruct {
    friend_counts: [u16; 4], // 0
}
const _: () = assert!(core::mem::size_of::<FriendshipFactionsStruct>() == 0x8);

#[repr(C)]
pub struct PlayerActionObject {
    timer: f32,          // 0
    refObj: RefHandle,   // 4
    next: PLAYER_ACTION, // 8
}
const _: () = assert!(core::mem::size_of::<PlayerActionObject>() == 0x0C);

#[repr(C)]
#[derive(Debug)]
pub struct PlayerCharacter {
    pub __base: Character,                            // 000
    pub __base1: BSTEventSource<BGSActorCellEvent>,   // SE: 0x2D0, AE: 0x2D8, VR: 0x2E8
    pub __base2: BSTEventSource<BGSActorDeathEvent>,  // SE: 0x328, AE: 0x330, VR: 0x340
    pub __base3: BSTEventSource<PositionPlayerEvent>, // SE: 0x380, AE: 0x388, VR: 0x398
    pub __base4: BSTEventSink<MenuOpenCloseEvent>,    // SE,VR: 0x2B0, AE: 0x2B8
    pub __base5: BSTEventSink<MenuModeChangeEvent>,   // SE,VR: 0x2B8, AE: 0x2C0
    pub __base6: BSTEventSink<UserEventEnabledEvent>, // SE,VR: 0x2C0, AE: 0x2C8
    pub __base7: BSTEventSink<TESTrackedStatsEvent>,  // SE,VR: 0x2C8, AE: 0x2D0
}
const _: () = assert!(core::mem::size_of::<PlayerCharacter>() == 0x1A0);

impl PlayerCharacter {
    pub const RTTI: VariantID = RTTI_PlayerCharacter;
    pub const VTABLE: [VariantID; 17] = VTABLE_PlayerCharacter;
    pub const FORM_TYPE: FormType = FormType::ActorCharacter;

    /// Returns the singleton instance of `Self`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "NiPointer<PlayerCharacter>",
        default = "None",
        id(se = 517014, ae = 403521)
    )]
    pub fn get_singleton() -> Option<&'static PlayerCharacter> {
        |as_type: AsType| unsafe { as_type.as_ptr().map(|p| p.as_ref()) }
    }

    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut bool",
        default = "false",
        deref_once,
        id(se = 517711, ae = 404238)
    )]
    #[inline]
    pub fn is_god_mode() -> bool {
        |deref_type: DerefType| deref_type
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 39471, ae_id = 40548)]
    pub fn activate_pick_ref(&mut self) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 39471, ae_id = 40548)]
    pub fn activate_pick_ref_vr(&mut self, device: VR_DEVICE) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 39384, ae_id = 40456)]
    pub fn add_player_add_item_event(
        &mut self,
        object: TESObject,
        owner: *mut TESForm,
        container: *mut TESObjectREFR,
        type_: ACQUIRE_TYPE,
    ) {
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 39568, ae_id = 40654)]
    pub fn attempt_pickpocket(
        &mut self,
        container_ref: *mut TESObjectREFR,
        entry: *mut InventoryEntryData,
        number: i32,
        from_container: bool,
    ) -> bool {
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 39409, ae_id = 40484)]
    pub fn check_cast(
        &mut self,
        spell: MagicItem,
        effect: Effect,
        reason: MagicSystem::CannotCastReason,
    ) -> bool {
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 39480, ae_id = 40557)]
    pub fn destroy_mouse_springs(&mut self) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 39365, ae_id = 40437)]
    pub fn center_on_cell_impl(
        &mut self,
        cell_name: Option<&str>,
        cell: Option<TESObjectCELL>,
    ) -> bool {
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 39413, ae_id = 40488)]
    pub fn add_skill_experience(&mut self, skill: ActorValue, experience: f32) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 39507, ae_id = 40586)]
    pub fn set_ai_driven(&mut self, enable: bool) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 39574, ae_id = 40660)]
    pub fn set_escaping(&mut self, flag: bool, escaped: bool) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 39475, ae_id = 40552)]
    pub fn start_grab_object(&mut self, device: VR_DEVICE) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 39535, ae_id = 40621)]
    pub fn update_crosshairs(&mut self) {}

    // VR-specific functions (if VR is enabled)
    // pub fn is_grabbing_with_device(&mut self, device: VR_DEVICE) -> bool {}
}

pub struct PlayerCharacterVtbl {
    pub __base: CharacterVtbl,

    // 0x12A * 8 = 4520
    pub Unk_12A: fn(this: *mut PlayerCharacter), // 0x12A
    pub GetViolentCrimeGoldValue: fn(this: *const PlayerCharacter, faction: *mut TESFaction) -> u32, // 0x12B
    pub GetNonViolentCrimeGoldValue:
        fn(this: *const PlayerCharacter, faction: *mut TESFaction) -> u32, // 0x12C
    pub ClearAllCrimeGold: fn(this: *mut PlayerCharacter, faction: *mut TESFaction), // 0x12D
    pub Unk_12E: fn(this: *mut PlayerCharacter),                                     // 0x12E
}

impl crate::re::NiSmartPointer::RefCountable for PlayerCharacter {
    #[inline]
    fn inc_ref_count(&self) {
        self.__base.__base.__base.inc_ref_count();
    }

    #[inline]
    fn dec_ref_count(&mut self) {
        self.__base.__base.__base.dec_ref_count();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventType {
    Thief = 3,
    Container = 5,
    DeadBody = 6,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ByCharGenFlag {
    #[default]
    None = 0,
    DisableSaving = 1 << 0,
    HandsBound = 1 << 2,
}

bitflags::bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct PlayerFlags: u64 {
        const TRAVEL_USE_DOOR                = 1 << 0;      // 0:0
        const FAST_TRAVELING                 = 1 << 1;      // 0:1
        const OVER_AUTO_AIM_TARGET           = 1 << 2;      // 0:2
        const SHOW_QUEST_ITEMS               = 1 << 3;      // 0:3
        const UNK0_4                         = 1 << 4;      // 0:4
        const HAS_QUEUED_EQUIP_ANIM          = 1 << 5;      // 0:5
        const ESCAPING                       = 1 << 6;      // 0:6
        const FORCE_QUEST_TARGET_REPATH      = 1 << 7;      // 0:7
        const UNK1_0                         = 1 << 8;      // 1:0
        const UNK1_1                         = 1 << 9;      // 1:1
        const SLEEPING                       = 1 << 10;     // 1:2
        const UNK1_3                         = 1 << 11;     // 1:3
        const UNK1_4                         = 1 << 12;     // 1:4
        const UNK1_5                         = 1 << 13;     // 1:5
        const GREETING_PLAYER                = 1 << 14;     // 1:6
        const UNK1_7                         = 1 << 15;     // 1:7
        const UNK2_0                         = 1 << 16;     // 2:0
        const AI_CONTROLLED_TO_POS           = 1 << 17;     // 2:1
        const AI_CONTROLLED_FROM_POS         = 1 << 18;     // 2:2
        const AI_CONTROLLED_PACKAGE          = 1 << 19;     // 2:3
        const RETURN_TO_LAST_KNOWN_GOOD_POSITION = 1 << 20; // 2:4
        const IS_BEING_CHASED                = 1 << 21;     // 2:5
        const UNK2_6                         = 1 << 22;     // 2:6
        const UNK2_7                         = 1 << 23;     // 2:7
        const IS_IN_THIRD_PERSON_MODE        = 1 << 24;     // 3:0
        const UNK3_1                         = 1 << 25;     // 3:1
        const UNK3_2                         = 1 << 26;     // 3:2
        const UNK3_3                         = 1 << 27;     // 3:3
        const TARGET_3D_DISTANT              = 1 << 28;     // 3:4
        const IS_IN_COMBAT                   = 1 << 29;     // 3:5
        const ATTEMPTED_YIELD_IN_CURRENT_COMBAT = 1 << 30;  // 3:6
        const UNK3_7                         = 1 << 31;     // 3:7
        const IS_LOADING                     = 1 << 32;     // 4:0
        const SHOULD_UPDATE_CROSSHAIR        = 1 << 33;     // 4:1
        const UNK4_2                         = 1 << 34;     // 4:2
        const HEALTH_TUTORIAL_SHOWN          = 1 << 35;     // 4:3
        const MAGICKA_TUTORIAL_SHOWN         = 1 << 36;     // 4:4
        const STAMINA_TUTORIAL_SHOWN         = 1 << 37;     // 4:5
        const GO_TO_JAIL_QUEUED              = 1 << 38;     // 4:6
        const UNK4_7                         = 1 << 39;     // 4:7
        const IS_SPRINTING                   = 1 << 40;     // 5:0
        const IS_SUNGAZING                   = 1 << 41;     // 5:1
        const DRAGON_RIDE_TARGET_LOCKED      = 1 << 42;     // 5:2
        const EVER_MODDED                    = 1 << 43;     // 5:3
        const SERVING_JAIL_TIME              = 1 << 44;     // 5:4
        const UNK5_5                         = 1 << 45;     // 5:5
        const UNK5_6                         = 1 << 46;     // 5:6
        const UNK5_7                         = 1 << 47;     // 5:7
        const UNK6_0                         = 1 << 47;     // 6:0
        const UNK6_1                         = 1 << 47;     // 6:1
        const UNK6_2                         = 1 << 47;     // 6:2
        const UNK6_3                         = 1 << 47;     // 6:3
        const UNK6_4                         = 1 << 47;     // 6:4
        const UNK6_5                         = 1 << 47;     // 6:5
        const UNK6_6                         = 1 << 47;     // 6:6
        const UNK6_7                         = 1 << 47;     // 6:7
        const UNK7_0                         = 1 << 47;     // 7:0
        const UNK7_1                         = 1 << 47;     // 7:1
        const UNK7_2                         = 1 << 47;     // 7:2
        const UNK7_3                         = 1 << 47;     // 7:3
        const UNK7_4                         = 1 << 47;     // 7:4
        const UNK7_5                         = 1 << 47;     // 7:5
        const UNK7_6                         = 1 << 47;     // 7:6
        const UNK7_7                         = 1 << 47;     // 7:7
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct QueuedWeapon {
    rightHandWeapon: *mut TESObjectWEAP, // 00 - These may be main/off hand weapon for VR?
    leftHandWeapon: *mut TESObjectWEAP,  // 08
}
const _: () = {
    assert!(core::mem::offset_of!(QueuedWeapon, rightHandWeapon) == 0x0);
    assert!(core::mem::offset_of!(QueuedWeapon, leftHandWeapon) == 0x8);
    assert!(core::mem::size_of::<QueuedWeapon>() == 0x10);
};

#[repr(C)]
#[derive(Debug)]
pub struct PreTransformationData {
    pub storedSelectedSpells: [*mut MagicItem; 4],
    pub storedRace: *mut TESRace,
    pub storedSelectedPower: *mut TESForm,
    pub storedLastOneHandItems: [*mut TESBoundObject; 2],
}

#[repr(C)]
#[derive(Debug)]
pub struct RaceData {
    pub complexion: *mut BGSTextureSet,
    pub charGenRace: *mut TESRace,
    pub race2: *mut TESRace,
}

#[repr(C)]
#[derive(Debug)]
pub struct GameStateData {
    pub difficulty: i32,
    pub assumedIdentity: ActorHandle,
    pub murder: i8,
    pub perkCount: i8,
    pub byCharGenFlag: ByCharGenFlag,
    pub padB: u8,
}
const _: () = assert!(core::mem::size_of::<GameStateData>() == 0xC);
