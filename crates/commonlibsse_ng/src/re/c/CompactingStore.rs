use core::ffi::c_void;

use crate::re::BSAtomic::BSNonReentrantSpinLock;
use crate::re::IMemoryStoreBase::{IMemoryStoreBase, MemoryStats};
use crate::re::offsets_rtti::RTTI_CompactingStore__Store;
use crate::re::offsets_vtable::VTABLE_CompactingStore__Store;
use crate::rel::ResolvableAddress as _;
use crate::rel::id::VariantID;

/// Dummy
#[repr(C)]
pub struct BlockHeader;
/// Dummy
#[repr(C)]
pub struct FreeBlock;
/// Dummy
#[repr(C)]
pub struct StoreBlock;

#[repr(C)]
pub struct Store {
    /// C++ Base class `IMemoryStoreBase`.
    pub __base: IMemoryStoreBase,

    // Members
    pub lock: BSNonReentrantSpinLock,
    pub allocBase: *mut c_void,
    pub allocEndMin: *mut c_void,
    pub allocEnd: *mut c_void,
    pub storeEnd: *mut c_void,
    pub lastBlock: *mut BlockHeader,
    pub smallFree: [*mut FreeBlock; 66],
    pub currentFree: *mut FreeBlock,
    pub nextMerge: *mut FreeBlock,
    pub storeBlockMin: *mut StoreBlock,
    pub nextStoreBlock: *mut StoreBlock,
    pub freeStoreBlockList: *mut StoreBlock,
    pub currentThread: u32,
    pub allocated: usize,
    pub numAllocatedBlocks: u32,
    pub free: usize,
    pub numFreeBlocks: u32,
    pub compacted: u32,
    pub batchDeallocateTlsSlot: u32,
}
const _: () = {
    use std::mem;
    assert!(mem::size_of::<Store>() == 0x2A0);
    assert!(mem::align_of::<Store>() == mem::align_of::<IMemoryStoreBase>());
};

impl Store {
    /// Runtime Type Information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_CompactingStore__Store;
    /// Virtual table identifier.
    pub const VTABLE: [VariantID; 1] = VTABLE_CompactingStore__Store;

    #[inline]
    pub fn vtable(&self) -> Option<&'static StoreVtbl> {
        Some(unsafe { Self::VTABLE[0].address().ok()?.cast().as_ref() })
    }
}

/// Virtual table for `Store`.
#[repr(C)]
pub struct StoreVtbl {
    /// C++ `virtual ~Store`
    pub CxxDrop: unsafe extern "C" fn(this: *mut Store),
    pub Size: unsafe extern "C" fn(this: *const Store, mem: *const u8) -> usize,
    pub GetMemoryStats: unsafe extern "C" fn(this: *mut Store, stats: *mut MemoryStats),
    pub ContainsBlockImpl: unsafe extern "C" fn(this: *const Store, block: *const u8) -> bool,
}
