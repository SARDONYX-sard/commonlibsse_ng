use crate::re::BSAtomic::BSReadWriteLock;
use crate::re::BSPointerHandle::ActorHandle;
use crate::re::BSTArray::BSTArray;
use crate::re::TESFaction;

// Enum to represent different crime types
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum CRIME_TYPE {
    #[default]
    None = u32::MAX,
    Steal = 0,
    Pickpocket = 1,
    Trespass = 2,
    Attack = 3,
    Murder = 4,
    Escape = 5,
    Unused = 6,
}
impl CRIME_TYPE {
    pub const TOTAL: usize = 7;
}

#[repr(C)]
#[derive(Debug)]
pub struct Crime {
    unk00: u64,                                  // 0x00
    unk08: u64,                                  // 0x08
    unk10: u64,                                  // 0x10
    unk18: u64,                                  // 0x18
    unk20: u64,                                  // 0x20
    actors_know_of_crime: BSTArray<ActorHandle>, // 0x28
    unk40: u64,                                  // 0x40
    unk48: u64,                                  // 0x48
    unk50: u64,                                  // 0x50
    unk58: u64,                                  // 0x58
    crime_faction: *mut TESFaction,              // 0x60
    unk68: u32,                                  // 0x68
    lock: BSReadWriteLock,                       // 0x68 - Mutable lock used for thread safety
    unk74: u32,                                  // 0x74
}
