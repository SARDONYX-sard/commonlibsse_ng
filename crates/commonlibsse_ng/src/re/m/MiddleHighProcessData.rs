use core::ffi::c_void;

use crate::re::ActorPackage::ActorPackage;
use crate::re::BGSEquipSlot::BGSEquipSlot;
use crate::re::BGSPerkEntry::BGSPerkEntry;
use crate::re::BSAnimationGraphManager::{BSAnimationGraphManager, BSAnimationGraphVariableCache};
use crate::re::BSAtomic::BSSpinLock;
use crate::re::BSPointerHandle::{ActorHandle, ObjectRefHandle};
use crate::re::BSTArray::BSTArray;
use crate::re::BSTEvent::BSTEventSource;
use crate::re::BSTList::BSSimpleList;
use crate::re::BSTSmartPointer::BSTSmartPointer;
use crate::re::ExtraDataList::ExtraDataList;
use crate::re::InventoryEntryData::InventoryEntryData;
use crate::re::MagicItem::MagicItem;
use crate::re::MagicSystem;
use crate::re::NiAVObject::NiAVObject;
use crate::re::NiPoint3::NiPoint3;
use crate::re::NiSmartPointer::NiPointer;
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::TESObjectREFR::TESObjectREFR;
use crate::re::{
    ActiveEffect, AnimResponse, BGSEntryPoint, BSCloneReserver, BSFaceGenAnimationData,
    BSFaceGenNiNode, BSLightingShaderProperty, HitData, NiNode, QueuedFile, TESIdleForm,
    bhkCharacterController, bhkRagdollPenetrationUtil,
};

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RESET_3D_FLAGS {
    None = 0,
    Model = 1 << 0,
    Skin = 1 << 1,
    Head = 1 << 2,
    Face = 1 << 3,
    Scale = 1 << 4,
    Skeleton = 1 << 5,
    InitDefault = 1 << 6,
    SkyCellSkin = 1 << 7,
}

#[repr(C)]
#[derive(Debug)]
pub struct CommandedActorData {
    commandedActor: ActorHandle,     // 0x00
    pad04: u32,                      // 0x04
    activeEffect: *mut ActiveEffect, // 0x10
}

#[repr(C)]
#[derive(Debug)]
pub struct ObjectEquipParams {
    extraDataList: *mut ExtraDataList, // 0x00
    count: i32,                        // 0x08
    pad0C: u32,                        // 0x0C
    equipSlot: *const BGSEquipSlot,    // 0x10
    unk18: *mut c_void,                // 0x18
    playEquipSounds: bool,             // 0x20
    forceEquip: bool,                  // 0x21
    showMessage: bool,                 // 0x22
    unk23: bool,                       // 0x23
    unk24: bool,                       // 0x24
    pad25: u8,                         // 0x25
    pad26: u16,                        // 0x26
}

#[repr(C)]
#[derive(Debug)]
pub struct QueuedItem {
    next: *mut QueuedItem,              // 0x00
    object: *mut TESBoundObject,        // 0x08
    equipParams: ObjectEquipParams,     // 0x10
    queuedFiles: NiPointer<QueuedFile>, // 0x38
    equip: bool,                        // 0x40
    pad41: u8,                          // 0x41
    pad42: u16,                         // 0x42
    pad44: u32,                         // 0x44
}

#[repr(C)]
#[derive(Debug)]
pub struct DeferredHideLimb {
    dismemberTimer: f32,                    // 0x00
    limbIndex: u32,                         // 0x04
    dismemberedLimbRoot: NiPointer<NiNode>, // 0x08
    replacementLimb: NiPointer<NiNode>,     // 0x10
    next: *mut DeferredHideLimb,            // 0x18
    explosion: bool,                        // 0x20
    pad21: u8,                              // 0x21
    pad22: u16,                             // 0x22
    pad24: u32,                             // 0x24
}
const _: () = assert!(core::mem::size_of::<DeferredHideLimb>() == 0x28);

#[repr(C)]
#[derive(Debug)]
pub struct AIPerkData {
    pub perkEntryArrays: [BSTArray<*mut BGSPerkEntry>; BGSEntryPoint::ENTRY_POINT_CEnum::count()],
}
const _: () = assert!(core::mem::size_of::<AIPerkData>() == 0x8A0);

#[repr(C)]
#[derive(Debug)]
pub struct MiddleHighProcessData {
    pub unk000: BSTEventSource<*mut c_void>,   // 0x000
    pub runOncePackage: ActorPackage,          // 0x058
    pub deadDetectList: BSTArray<ActorHandle>, // 0x088
    pub refListChairBed: BSSimpleList<*mut TESObjectREFR>, // 0x0A0
    pub rotation: NiPoint3,                    // 0x0B0
    pub rotationSpeed: NiPoint3,               // 0x0BC
    pub actorMountPosition: NiPoint3,          // 0x0C8
    pub furniturePathPoint: NiPoint3,          // 0x0D4
    pub lastSeenPosition: NiPoint3,            // 0x0E0
    pub bleedoutAttacker: u32,                 // 0x0EC
    pub wardState: MagicSystem::WardState,     // 0x0F0
    pub pad0F4: u32,                           // 0x0F4
    pub animResponse: BSTSmartPointer<AnimResponse>, // 0x0F8
    pub commandedActors: BSTArray<CommandedActorData>, // 0x100
    pub damageRootNode: [*mut NiNode; 6],      // 0x118
    pub unk148: *mut NiAVObject,               // 0x148
    pub weaponBone: *mut NiNode,               // 0x150
    pub headNode: *mut NiAVObject,             // 0x158
    pub torsoNode: *mut NiAVObject,            // 0x160
    pub faceTargetSourceNode: *mut NiAVObject, // 0x168
    pub faceNodeSkinned: *mut BSFaceGenNiNode, // 0x170
    pub lightingProperty: NiPointer<BSLightingShaderProperty>, // 0x178
    pub unk180: u64,                           // 0x180
    pub itemstoEquipUnequip: *mut QueuedItem,  // 0x188
    pub lastHitData: *mut HitData,             // 0x190
    pub headDeferredHideLimb: *mut DeferredHideLimb, // 0x198
    pub activeEffects: *mut BSSimpleList<*mut ActiveEffect>, // 0x1A0
    pub animationGraphManager: BSTSmartPointer<BSAnimationGraphManager>, // 0x1A8
    pub animationVariableCache: *mut BSAnimationGraphVariableCache, // 0x1B0
    pub unk1B8: BSTArray<*mut c_void>,         // 0x1B8
    pub unk1D0: BSTArray<*mut c_void>,         // 0x1D0
    pub unk1E8: BSSpinLock,                    // 0x1E8
    pub unk1F0: *mut c_void,                   // 0x1F0 - smart ptr
    pub unk1F8: u16,                           // 0x1F8
    pub unk1FA: u16,                           // 0x1FA
    pub unk1FC: u32,                           // 0x1FC
    pub unk200: u32,                           // 0x200
    pub headHeightOffset: f32,                 // 0x204
    pub occupiedFurniture: ObjectRefHandle,    // 0x208
    pub unk20C: u32,                           // 0x20C
    pub unk210: *mut TESIdleForm,              // 0x210
    pub commandingActor: ActorHandle,          // 0x218
    pub pad21C: u32,                           // 0x21C
    pub leftHand: *mut InventoryEntryData,     // 0x220
    pub furnitureIdle: *mut TESIdleForm,       // 0x228
    pub unk230: *mut c_void,                   // 0x230 - smart ptr
    pub faceAnimationData: *mut BSFaceGenAnimationData, // 0x238
    pub currentPackageSpell: *mut MagicItem,   // 0x240
    pub unk248: u64,                           // 0x248
    pub charController: NiPointer<bhkCharacterController>, // 0x250
    pub penetrationDetectUtil: BSTSmartPointer<bhkRagdollPenetrationUtil>, // 0x258
    pub rightHand: *mut InventoryEntryData,    // 0x260
    pub bothHands: *mut InventoryEntryData,    // 0x268
    pub bodyPartPreload: NiPointer<QueuedFile>, // 0x270
    pub unk278: NiPointer<BSCloneReserver>,    // 0x278
    pub lastIdlePlayed: *mut TESIdleForm,      // 0x280
    pub perkData: *mut AIPerkData,             // 0x288
    pub unk290: u32,                           // 0x290
    pub currentFurnitureSubgraphID: u32,       // 0x294
    pub unk298: f32,                           // 0x298
    pub unk29C: f32,                           // 0x29C
    pub unk2A0: f32,                           // 0x2A0
    pub unk2A4: f32,                           // 0x2A4
    pub currentMovementSpeed: f32,             // 0x2A8
    pub unk2AC: f32,                           // 0x2AC
    pub unk2B0: f32,                           // 0x2B0
    pub bleedoutRate: f32,                     // 0x2B4
    pub unk2B8: f32,                           // 0x2B8
    pub maximumWardPower: f32,                 // 0x2BC
    pub unk2C0: f32,                           // 0x2C0
    pub torchEvaluationTimer: f32,             // 0x2C4
    pub alphaMult: f32,                        // 0x2C8
    pub scriptRefractPower: f32,               // 0x2CC
    pub unk2D0: f32,                           // 0x2D0
    pub deferredKillTimer: f32,                // 0x2D4
    pub killMoveTimer: f32,                    // 0x2D8
    pub unk2DC: f32,                           // 0x2DC
    pub unk2E0: u32,                           // 0x2E0
    pub unk2E4: u32,                           // 0x2E4
    pub currentFurnitureMarkerID: u32,         // 0x2E8
    pub unk2EC: u32,                           // 0x2EC
    pub unk2F0: u64,                           // 0x2F0
    pub unk2F8: u32,                           // 0x2F8
    pub unk2FC: u32,                           // 0x2FC
    pub unk300: u32,                           // 0x300
    pub unk304: u16,                           // 0x304
    pub unk306: u16,                           // 0x306
    pub unk308: u64,                           // 0x308
    pub unk310: u8,                            // 0x310
    pub update3DModel: RESET_3D_FLAGS,         // 0x311
    pub unk312: u16,                           // 0x312
    pub unk314: u16,                           // 0x314
    pub unk316: bool,                          // 0x316
    pub unk317: bool,                          // 0x317
    pub unk318: bool,                          // 0x318
    pub unk319: bool,                          // 0x319
    pub unk31A: bool,                          // 0x31A
    pub pickPocketed: bool,                    // 0x31B
    pub summonedCreature: bool,                // 0x31C
    pub forceNextUpdate: bool,                 // 0x31D
    pub unk31E: bool,                          // 0x31E
    pub unk31F: bool,                          // 0x31F
    pub unk320: bool,                          // 0x320
    pub unk321: bool,                          // 0x321
    pub beenAttacked: bool,                    // 0x322
    pub alwaysHit: bool,                       // 0x323
    pub doNoDamage: bool,                      // 0x324
    pub soulTrapped: bool,                     // 0x325
    pub unk326: bool,                          // 0x326
    pub unk327: bool,                          // 0x327
    pub unk328: bool,                          // 0x328
    pub preventCombat: bool,                   // 0x329
    pub unk32A: bool,                          // 0x32A
    pub isFleeing: bool,                       // 0x32B
    pub unk32C: bool,                          // 0x32C
    pub hostileGuard: bool,                    // 0x32D
    pub unk32E: bool,                          // 0x32E
    pub unk32F: bool,                          // 0x32F
    pub unk330: bool,                          // 0x330
    pub killQueued: bool,                      // 0x331
    pub inDeferredKill: bool,                  // 0x332
    pub pad333: bool,                          // 0x333
    pub pad334: u32,                           // 0x334
}
const _: () = assert!(core::mem::size_of::<MiddleHighProcessData>() == 0x338);
