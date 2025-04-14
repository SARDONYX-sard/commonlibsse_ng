use windows::Win32::System::Threading::CRITICAL_SECTION;

use crate::re::AIProcess::AIProcess;
use crate::re::BSPointerHandle::{ActorHandle, ObjectRefHandle};
use crate::re::BSTArray::BSTSmallArray;
use crate::re::BSTSmartPointer::BSTSmartPointer;
use crate::re::MagicItem::MagicItem;
use crate::re::NiPoint3::NiPoint3;
use crate::re::SpellItem::SpellItem;
use crate::re::TESForm::TESForm;
use crate::re::{
    AITimeStamp, ActorMagicCaster, ActorMover, BGSDialogueBranch, BGSLocation, BipedAnim,
    CombatController, EmotionType, MovementControllerNPC, TESFaction, TESPackage, TESRace,
};

use super::{
    ACTOR_CRITICAL_STAGE_CEnum, ActorValueStorage, BOOL_BITS, BOOL_FLAGS, Modifiers,
    SlotTypes_CEnum,
};

const _: () = assert!(SlotTypes_CEnum::count() == 4);

#[derive(Debug, Clone, PartialEq)]
pub struct ACTOR_RUNTIME_DATA {
    pub boolBits: BOOL_BITS,                                        // 0x0E0
    pub updateTargetTimer: f32,                                     // 0x0E4
    pub criticalStage: ACTOR_CRITICAL_STAGE_CEnum,                  // 0x0E8
    pub pad0EC: u32,                                                // 0x0EC
    pub currentProcess: *mut AIProcess,                             // 0x0F0
    pub dialogueItemTarget: ObjectRefHandle,                        // 0x0F8
    pub currentCombatTarget: ActorHandle,                           // 0x0FC
    pub myKiller: ActorHandle,                                      // 0x100
    pub checkMyDeadBodyTimer: f32,                                  // 0x104
    pub voiceTimer: f32,                                            // 0x108
    pub underWaterTimer: f32,                                       // 0x10C
    pub thiefCrimeStamp: i32,                                       // 0x110
    pub actionValue: i32,                                           // 0x114
    pub timerOnAction: f32,                                         // 0x118
    pub unk11C: u32,                                                // 0x11C
    pub editorLocCoord: NiPoint3,                                   // 0x120
    pub editorLocRot: f32,                                          // 0x12C
    pub editorLocForm: *mut TESForm,                                // 0x130
    pub editorLocation: *mut BGSLocation,                           // 0x138
    pub actorMover: *mut ActorMover,                                // 0x140
    pub movementController: BSTSmartPointer<MovementControllerNPC>, // 0x148
    pub unk150: *mut TESPackage,                                    // 0x150
    pub combatController: *mut CombatController,                    // 0x158
    pub vendorFaction: *mut TESFaction,                             // 0x160
    pub calculateVendorFactionTimer: AITimeStamp,                   // 0x168
    pub emotionType: EmotionType,                                   // 0x16C
    pub emotionValue: u32,                                          // 0x170
    pub unk174: u32,                                                // 0x174
    pub unk178: u32,                                                // 0x178
    pub intimidateBribeDayStamp: u32,                               // 0x17C
    pub unk180: u64,                                                // 0x180
    pub addedSpells: BSTSmallArray<*mut SpellItem>,                 // 0x188
    pub magicCasters: [*mut ActorMagicCaster; 4],                   // 0x1A0
    pub selectedSpells: [*mut MagicItem; 4],                        // 0x1C0
    pub selectedPower: *mut TESForm,                                // 0x1E0
    pub unk1E8: u32,                                                // 0x1E8
    pub pad1EC: u32,                                                // 0x1EC
    pub race: *mut TESRace,                                         // 0x1F0
    pub equippedWeight: f32,                                        // 0x1F8
    pub boolFlags: BOOL_FLAGS,                                      // 0x1FC
    pub avStorage: ActorValueStorage,                               // 0x200
    pub exclusiveBranch: *mut BGSDialogueBranch,                    // 0x220
    pub healthModifiers: Modifiers,                                 // 0x228
    pub magickaModifiers: Modifiers,                                // 0x234
    pub staminaModifiers: Modifiers,                                // 0x240
    pub voicePointsModifiers: Modifiers,                            // 0x24C
    pub lastUpdate: f32,                                            // 0x258
    pub lastSeenTime: u32,                                          // 0x25C
    pub biped: BSTSmartPointer<BipedAnim>,                          // 0x260
    pub armorRating: f32,                                           // 0x268
    pub armorBaseFactorSum: f32,                                    // 0x26C
    pub soundCallBackSet: i8,                                       // 0x271
    pub unk271: u8,                                                 // 0x270
    pub unk272: u8,                                                 // 0x272
    pub unk273: u8,                                                 // 0x273
    pub unk274: u32,                                                // 0x274
    pub unk278: u64,                                                // 0x278
    pub unk280: u64,                                                // 0x280
    pub unk288: CRITICAL_SECTION,                                   // 0x288 havok related
}
const _: () = {
    const ACTUAL_SIZE: usize = core::mem::size_of::<ACTOR_RUNTIME_DATA>();
    const EXPECTED_SIZE: usize = (0x288 + 40) - 0x0E0;
    // assert!(ACTUAL_SIZE == EXPECTED_SIZE);
};
