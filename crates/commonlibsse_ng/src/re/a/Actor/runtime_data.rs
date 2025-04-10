use windows::Win32::System::Threading::CRITICAL_SECTION;

use crate::re::BSPointerHandle::{ActorHandle, ObjectRefHandle};
use crate::re::BSTArray::BSTSmallArray;
use crate::re::BSTSmartPointer::BSTSmartPointer;
use crate::re::MagicItem::MagicItem;
use crate::re::NiPoint3::NiPoint3;
use crate::re::TESForm::TESForm;
use crate::re::{BGSDialogueBranch, BGSLocation, BipedAnim, TESFaction, TESRace};

use super::{
    ACTOR_CRITICAL_STAGE_CEnum, ActorValueStorage, BOOL_BITS, BOOL_FLAGS, Modifiers,
    SlotTypes_CEnum,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AIProcess;

#[derive(Debug, Clone, PartialEq)]
pub struct ActorMover;

#[derive(Debug, Clone, PartialEq)]
pub struct TESPackage;

#[derive(Debug, Clone, PartialEq)]
pub struct CombatController;

#[derive(Debug, Clone, PartialEq)]
pub struct AITimeStamp {
    dummy: [u8; 4],
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum EmotionType {
    Dummy,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpellItem;

#[derive(Debug, Clone, PartialEq)]
pub struct MovementControllerNPC;
impl crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCountedTrait for MovementControllerNPC {
    fn inc_ref(&self) -> u32 {
        todo!()
    }

    fn dec_ref(&self) -> u32 {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActorMagicCaster;

const _: () = assert!(SlotTypes_CEnum::count() == 4);

#[derive(Debug, Clone, PartialEq)]
pub struct ACTOR_RUNTIME_DATA {
    boolBits: BOOL_BITS,                                        // 0x0E0
    updateTargetTimer: f32,                                     // 0x0E4
    criticalStage: ACTOR_CRITICAL_STAGE_CEnum,                  // 0x0E8
    pad0EC: u32,                                                // 0x0EC
    currentProcess: *mut AIProcess,                             // 0x0F0
    dialogueItemTarget: ObjectRefHandle,                        // 0x0F8
    currentCombatTarget: ActorHandle,                           // 0x0FC
    myKiller: ActorHandle,                                      // 0x100
    checkMyDeadBodyTimer: f32,                                  // 0x104
    voiceTimer: f32,                                            // 0x108
    underWaterTimer: f32,                                       // 0x10C
    thiefCrimeStamp: i32,                                       // 0x110
    actionValue: i32,                                           // 0x114
    timerOnAction: f32,                                         // 0x118
    unk11C: u32,                                                // 0x11C
    editorLocCoord: NiPoint3,                                   // 0x120
    editorLocRot: f32,                                          // 0x12C
    editorLocForm: *mut TESForm,                                // 0x130
    editorLocation: *mut BGSLocation,                           // 0x138
    actorMover: *mut ActorMover,                                // 0x140
    movementController: BSTSmartPointer<MovementControllerNPC>, // 0x148
    unk150: *mut TESPackage,                                    // 0x150
    combatController: *mut CombatController,                    // 0x158
    vendorFaction: *mut TESFaction,                             // 0x160
    calculateVendorFactionTimer: AITimeStamp,                   // 0x168
    emotionType: EmotionType,                                   // 0x16C
    emotionValue: u32,                                          // 0x170
    unk174: u32,                                                // 0x174
    unk178: u32,                                                // 0x178
    intimidateBribeDayStamp: u32,                               // 0x17C
    unk180: u64,                                                // 0x180
    addedSpells: BSTSmallArray<*mut SpellItem>,                 // 0x188
    magicCasters: [*mut ActorMagicCaster; 4],                   // 0x1A0
    selectedSpells: [*mut MagicItem; 4],                        // 0x1C0
    selectedPower: *mut TESForm,                                // 0x1E0
    unk1E8: u32,                                                // 0x1E8
    pad1EC: u32,                                                // 0x1EC
    race: *mut TESRace,                                         // 0x1F0
    equippedWeight: f32,                                        // 0x1F8
    boolFlags: BOOL_FLAGS,                                      // 0x1FC
    avStorage: ActorValueStorage,                               // 0x200
    exclusiveBranch: *mut BGSDialogueBranch,                    // 0x220
    healthModifiers: Modifiers,                                 // 0x228
    magickaModifiers: Modifiers,                                // 0x234
    staminaModifiers: Modifiers,                                // 0x240
    voicePointsModifiers: Modifiers,                            // 0x24C
    lastUpdate: f32,                                            // 0x258
    lastSeenTime: u32,                                          // 0x25C
    biped: BSTSmartPointer<BipedAnim>,                          // 0x260
    armorRating: f32,                                           // 0x268
    armorBaseFactorSum: f32,                                    // 0x26C
    soundCallBackSet: i8,                                       // 0x271
    unk271: u8,                                                 // 0x270
    unk272: u8,                                                 // 0x272
    unk273: u8,                                                 // 0x273
    unk274: u32,                                                // 0x274
    unk278: u64,                                                // 0x278
    unk280: u64,                                                // 0x280
    unk288: CRITICAL_SECTION,                                   // 0x288 havok related
}
const _: () = {
    const ACTUAL_SIZE: usize = core::mem::size_of::<ACTOR_RUNTIME_DATA>();
    const EXPECTED_SIZE: usize = (0x288 + 40) - 0x0E0;
    // assert!(ACTUAL_SIZE == EXPECTED_SIZE);
};
