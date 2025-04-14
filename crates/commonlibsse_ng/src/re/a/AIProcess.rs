mod cached_values;

use core::ffi::c_void;

pub use self::cached_values::*;

use crate::re::ActorPackage::ActorPackage;
use crate::re::BGSEquipSlot::BGSEquipSlot;
use crate::re::BSCoreTypes::RefHandle;
use crate::re::BSTArray::BSTArray;
use crate::re::BSTList::BSSimpleList;
use crate::re::HighProcessData;
use crate::re::MiddleHighProcessData::MiddleHighProcessData;
use crate::re::TESForm::TESForm;
use crate::re::TESObjectREFR::TESObjectREFR;

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PROCESS_TYPE {
    High = 0,
    MiddleHigh = 1,
    MiddleLow = 2,
    Low = 3,
}

impl PROCESS_TYPE {
    pub const NONE: u32 = u32::MAX;
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MiddleLowProcessData {
    hourPackageEvaluated: i32,
}
const _: () = assert!(core::mem::size_of::<MiddleLowProcessData>() == 0x4);

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectsToAcquire {
    unk00: u64, // 0x00
    unk08: u64, // 0x08
    unk10: u64, // 0x10
    unk18: u64, // 0x18
    unk20: u64, // 0x20
}
const _: () = assert!(core::mem::size_of::<ObjectsToAcquire>() == 0x28);

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum LowProcessFlags {
    None = 0,
    TargetActivated = 1 << 0,
    CurrentActionComplete = 1 << 1,
    Alert = 1 << 3,
    Follower = 1 << 4,
    PackageDoneOnce = 1 << 5,
    PackageIdleDone = 1 << 6,
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Hand {
    Left,
    Right,
}
const _: () = assert!(Hand_CEnum::count() == 2);

#[repr(C)]
#[derive(Debug)]
pub struct EquippedObject {
    object: *mut TESForm,    // 00
    slot: *mut BGSEquipSlot, // 08
}
const _: () = assert!(core::mem::size_of::<EquippedObject>() == 0x10);

#[repr(C)]
#[derive(Debug)]
pub struct Data0B8 {
    unk00: *mut c_void,  // 0x00
    unk08: *mut Data0B8, // 0x08
    unk10: *mut c_void,  // 0x10
    unk18: *mut c_void,  // 0x18
    unk20: u64,          // 0x20
    unk28: *mut c_void,  // 0x28
    unk30: u32,          // 0x30
    pad34: u32,          // 0x34
}
const _: () = assert!(core::mem::size_of::<Data0B8>() == 0x38);

#[repr(C)]
#[derive(Debug)]
pub struct AIProcess {
    pub middleLow: *mut MiddleLowProcessData,          // 0x000
    pub middleHigh: *mut MiddleHighProcessData,        // 0x008
    pub high: *mut HighProcessData,                    // 0x010
    pub currentPackage: ActorPackage,                  // 0x018
    pub hourLastProcessed: f32,                        // 0x048
    pub dateLastProcessed: f32,                        // 0x04C
    pub cachedValues: *mut CachedValues,               // 0x050
    pub numberItemsActivate: i32,                      // 0x058
    pub pad05C: u32,                                   // 0x05C
    pub objects: BSSimpleList<ObjectsToAcquire>,       // 0x060
    pub genericLocations: BSSimpleList<TESObjectREFR>, // 0x070
    pub acquireObject: *mut ObjectsToAcquire,          // 0x080
    pub savedAcquireObject: *mut ObjectsToAcquire,     // 0x088
    pub essentialDownTimer: f32,                       // 0x090
    pub deathTime: f32,                                // 0x094
    pub trackedDamage: f32,                            // 0x098
    pub pad09C: u32,                                   // 0x09C
    pub equippedForms: BSTArray<EquippedObject>,       // 0x0A0
    pub unk0B8: Data0B8,                               // 0x0B8
    pub equippedObjects: [*mut TESForm; 2],            // 0x0F0 2: Hand_CEnum::count()
    pub unk100: u64,                                   // 0x100
    pub unk108: u64,                                   // 0x108
    pub followTarget: RefHandle,                       // 0x110
    pub target: RefHandle,                             // 0x114
    pub arrestTarget: RefHandle,                       // 0x118
    pub unk120: u64,                                   // 0x120
    pub unk128: u64,                                   // 0x128
    pub unk130: u32,                                   // 0x130
    pub unk134: u16,                                   // 0x134
    pub lowProcessFlags: LowProcessFlags,              // 0x136
    pub processLevel: PROCESS_TYPE,                    // 0x137
    pub skippedTimeStampForPathing: bool,              // 0x138
    pub ignoringCombat: bool,                          // 0x139
    pub endAlarmOnActor: bool,                         // 0x13A
    pub escortingPlayer: bool,                         // 0x13B
    pub pad13C: u32,                                   // 0x13C
}
const _: () = assert!(core::mem::size_of::<AIProcess>() == 0x140);
