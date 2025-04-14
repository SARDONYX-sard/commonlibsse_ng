use crate::re::BSAtomic::BSSpinLock;
use crate::re::BSPointerHandle::ObjectRefHandle;
use crate::re::{ActorPackageData, TESPackage};

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ACTOR_PACKAGE_FLAG {
    None = 0,
    SaveLoadSharedPackage = 1 << 0,
}

#[repr(C)]
#[derive(Debug)]
pub struct ActorPackage {
    pub packageLock: BSSpinLock,               // 00
    pub package: *mut TESPackage,              // 08
    pub data: *mut ActorPackageData,           // 10
    pub target: ObjectRefHandle,               // 18
    pub currentProcedureIndex: i32,            // 1C
    pub packageStartTime: f32,                 // 20
    pub modifiedPackageFlag: u32,              // 24
    pub modifiedInterruptFlag: u16,            // 28
    pub actorPackageFlags: ACTOR_PACKAGE_FLAG, // 2A
    pub preferredSpeed: i8,                    // 2B
    pub pad2C: u32,                            // 2C
}
const _: () = assert!(core::mem::size_of::<ActorPackage>() == 0x30);
