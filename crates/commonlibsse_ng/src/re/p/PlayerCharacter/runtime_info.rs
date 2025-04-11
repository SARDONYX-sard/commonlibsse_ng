use crate::re::BSCoreTypes::RefHandle;
use crate::re::BSPointerHandle::{ActorHandle, ObjectRefHandle};
use crate::re::BSTArray::BSTArray;
use crate::re::BSTSmartPointer::BSTSmartPointer;
use crate::re::Crime::CRIME_TYPE;
use crate::re::NiAVObject::NiAVObject;
use crate::re::NiSmartPointer::NiPointer;
use crate::re::PlayerCharacter::crime::TeleportPath;
use crate::re::PlayerCharacter::skill::PlayerSkills;
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::TESForm::TESForm;
use crate::re::{AlchemyItem, BSLight, BipedAnim, CombatGroup, NiNode, TESClass};

const _: () = assert!(7 == CRIME_TYPE::TOTAL);

#[repr(C, packed(4))]
pub struct INFO_RUNTIME_DATA {
    // dummy: [u8; 20],
    pub sleepSeconds: u32,                                   // 014
    pub largeBiped: BSTSmartPointer<BipedAnim>,              // 018
    pub firstPerson3D: NiPointer<NiNode>,                    // 020
    pub eyeHeight: f32,                                      // 028
    pub greetTimer: f32,                                     // 02C
    pub encumberedTimer: f32,                                // 030
    pub powerAttackTimer: f32,                               // 034
    pub hoursToSleep: i32,                                   // 038
    pub amountStolenSold: i32,                               // 03C
    pub valueStolen: u32,                                    // 040
    pub lastRiddenMount: ActorHandle,                        // 044
    pub lightTarget: ActorHandle,                            // 048
    pub sortActorDistanceTimer: f32,                         // 04C
    pub sitHeadingDelta: f32,                                // 050 only in SSE, VR needs adjustment
    pub playerMapMarker: ObjectRefHandle,                    // 054
    pub playerMarkerPath: *mut TeleportPath,                 // 058
    pub skillTrainingsThisLevel: u32,                        // 060
    pub unk064: u32,                                         // 064
    pub defaultClass: *mut TESClass,                         // 068
    pub unk070: u64,                                         // 070
    pub crimeCounts: [u32; 7],                               // 078  7: CRIME_TYPE::TOTAL
    pub unk094: u32,                                         // 094
    pub pendingPoison: *mut AlchemyItem,                     // 098
    pub lastPlayingTimeUpdate: i64,                          // 0A0
    pub totalPlayingTime: i64,                               // 0A8
    pub characterSeed: i32,                                  // 0B0
    pub unk0B4: u32,                                         // 0B4
    pub lastKnownGoodLocation: *mut TESForm,                 // 0B8
    pub unk0C0: u32,                                         // 0C0
    pub unk0C4: u32,                                         // 0C4
    pub firstPersonLight: NiPointer<BSLight>,                // 0C8
    pub thirdPersonLight: NiPointer<BSLight>,                // 0D0
    pub dropAngleMod: f32,                                   // 0D8
    pub lastDropAngleMod: f32,                               // 0DC
    pub skills: *mut PlayerSkills,                           // 0E0
    pub autoAimActor: ActorHandle,                           // 0E8
    pub unk0EC: RefHandle,                                   // 0EC
    pub unk118: u64,                                         // 0F0
    pub targeted3D: NiPointer<NiAVObject>,                   // 0F8
    pub combatGroup: *mut CombatGroup,                       // 100
    pub actorsToDisplayOnTheHUDArray: BSTArray<ActorHandle>, // 108
    pub advanceObject: *mut TESForm,                         // 120
    pub lastOneHandItems: [*mut TESBoundObject; 2],          // 128
    pub teammateCount: u32,                                  // 138
    pub combatTimer: f32,                                    // 13C
    pub yieldTimer: f32,                                     // 140
    pub chaseTimer: f32,                                     // 144
    pub drawSheatheSafetyTimer: f32,                         // 148
    pub unk14C: u32,                                         // 14C
}
const _: () = assert!(core::mem::size_of::<INFO_RUNTIME_DATA>() == 0x13C);

#[repr(C, packed(4))]
pub struct VR_INFO_RUNTIME_DATA {
    pub sleepSeconds: u32,                                   // FE0
    pub unkFE4: u32,                                         // FE4
    pub largeBiped: BSTSmartPointer<BipedAnim>,              // FE8
    pub firstPerson3D: NiPointer<NiNode>,                    // FF0
    pub eyeHeight: f32,                                      // FF8
    pub greetTimer: f32,                                     // FFC
    pub encumberedTimer: f32,                                // 1000
    pub powerAttackTimer: f32,                               // 1004
    pub hoursToSleep: i32,                                   // 1008
    pub amountStolenSold: i32,                               // 100C
    pub valueStolen: u32,                                    // 1010
    pub lastRiddenMount: ActorHandle,                        // 1014
    pub lightTarget: ActorHandle,                            // 1018
    pub sortActorDistanceTimer: f32,                         // 101C
    pub playerMapMarker: ObjectRefHandle,                    // 1020
    pub pad1024: u32,                                        // 1024
    pub playerMarkerPath: *mut TeleportPath,                 // 1028
    pub skillTrainingsThisLevel: u32,                        // 1030
    pub unk1034: u32,                                        // 1034
    pub defaultClass: *mut TESClass,                         // 1038
    pub unk1040: u64,                                        // 1040
    pub crimeCounts: [u32; CRIME_TYPE::TOTAL],               // 1048
    pub unk964: u32,                                         // 1064
    pub pendingPoison: *mut AlchemyItem,                     // 1068
    pub lastPlayingTimeUpdate: i64,                          // 1070
    pub totalPlayingTime: i64,                               // 1078
    pub characterSeed: i32,                                  // 1080
    pub unk984: u32,                                         // 1084
    pub lastKnownGoodLocation: *mut TESForm,                 // 1088
    pub unk990: u32,                                         // 1090
    pub unk994: u32,                                         // 1094
    pub firstPersonLight: NiPointer<BSLight>,                // 1098
    pub thirdPersonLight: NiPointer<BSLight>,                // 10A0
    pub dropAngleMod: f32,                                   // 10A8
    pub lastDropAngleMod: f32,                               // 10AC
    pub skills: *mut PlayerSkills,                           // 10B0
    pub autoAimActor: ActorHandle,                           // 10B8
    pub unk9BC: RefHandle,                                   // 10BC
    pub unk9C0: u64,                                         // 10C0
    pub targeted3D: NiPointer<NiAVObject>,                   // 10C8
    pub combatGroup: *mut CombatGroup,                       // 10D0
    pub actorsToDisplayOnTheHUDArray: BSTArray<ActorHandle>, // 10D8
    pub advanceObject: *mut TESForm,                         // 10F0
    pub lastOneHandItems: [*mut TESBoundObject; 2],          // 10F8
    pub teammateCount: u32,                                  // 1108
    pub combatTimer: f32,                                    // 110C
    pub yieldTimer: f32,                                     // 1110
    pub chaseTimer: f32,                                     // 1114
    pub drawSheatheSafetyTimer: f32,                         // 1118
    pub unk111C: u32,                                        // 111C
}
const _: () = assert!(core::mem::size_of::<VR_INFO_RUNTIME_DATA>() == 0x140);
