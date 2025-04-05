use core::ptr::NonNull;

use crate::re::BSTArray::BSTArray;
use crate::re::BSTHashMap::BSTHashMap;
use crate::re::TESFaction;
use crate::re::TESObjectCELL::TESObjectCELL;
use crate::re::TESObjectREFR::TESObjectREFR;
use crate::re::TESWorldSpace::TESWorldSpace;

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CrimeGoldStruct {
    violentCur: f32,       // 00
    nonViolentCur: f32,    // 04
    nonViolentInfamy: f32, // 08
    violentInfamy: f32,    // 0C
}
const _: () = {
    assert!(core::mem::offset_of!(CrimeGoldStruct, violentCur) == 0x0);
    assert!(core::mem::offset_of!(CrimeGoldStruct, nonViolentCur) == 0x4);
    assert!(core::mem::offset_of!(CrimeGoldStruct, nonViolentInfamy) == 0x8);
    assert!(core::mem::offset_of!(CrimeGoldStruct, violentInfamy) == 0xc);
    assert!(core::mem::size_of::<CrimeGoldStruct>() == 0x10);
};

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StolenItemValueStruct {
    unwitnessed: i32, // 0
    witnessed: i32,   // 4
}
const _: () = {
    assert!(core::mem::offset_of!(StolenItemValueStruct, unwitnessed) == 0x0);
    assert!(core::mem::offset_of!(StolenItemValueStruct, witnessed) == 0x4);
    assert!(core::mem::size_of::<StolenItemValueStruct>() == 0x8);
};

#[repr(C)]
#[derive(Debug)]
pub struct TeleportPath {
    unk00: BSTArray<Unk00Data>, // 0x00
    unk18: BSTArray<Unk18Data>, // 0x18
    unk30: u64,                 // 0x30
    unk38: u64,                 // 0x38
    unk40: u64,                 // 0x40
}
const _: () = assert!(core::mem::size_of::<TeleportPath>() == 0x48);

#[repr(C)]
#[derive(Debug)]
pub struct Unk00Data {
    unk00: bool,    // 0x00 - Determines whether to use worldspace or cell?
    pad01: [u8; 7], // 0x01
    worldspace: Option<NonNull<TESWorldSpace>>, // 0x08
    interiorCell: Option<NonNull<TESObjectCELL>>, // 0x10
}
const _: () = assert!(core::mem::size_of::<Unk00Data>() == 0x18);

#[repr(C)]
#[derive(Debug)]
pub struct Unk18Data {
    unk00: Option<NonNull<TESObjectREFR>>, // 0x00
    unk08: u64,                            // 0x08
    unk10: u64,                            // 0x10
}
const _: () = assert!(core::mem::size_of::<Unk18Data>() == 0x18);

#[repr(C)]
#[derive(Debug)]
pub struct CrimeValue {
    pub crimeGoldMap: BSTHashMap<*const TESFaction, CrimeGoldStruct>,
    pub stolenItemValueMap: BSTHashMap<*const TESFaction, StolenItemValueStruct>,
}
