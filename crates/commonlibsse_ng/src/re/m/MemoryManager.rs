use core::{ffi::c_void, ptr::NonNull};

use crate::re::BSSmallBlockAllocator::BSSmallBlockAllocator;
use crate::re::CompactingStore;
use crate::re::IMemoryHeap::IMemoryHeap;

/// Represents the `ScrapHeap` struct, which needs to be defined externally.
#[repr(C)]
pub struct ScrapHeap {
    pub data: [u8; 0x90], // Placeholder size, actual struct size is 0x90
}

/// Represents a thread-local heap in the memory manager.
#[repr(C)]
pub struct ThreadScrapHeap {
    pub heap: ScrapHeap,            // 0x00
    pub next: *mut ThreadScrapHeap, // 0x90
    pub owningThread: u32,          // 0x98
    pub pad: u32,                   // 0x9C
}

const _: () = {
    assert!(core::mem::size_of::<ThreadScrapHeap>() == 0xA0);
};

/// Memory manager interface
#[repr(C)]
pub struct MemoryManager {
    pub initialized: bool,                               // 0x000
    pub numHeaps: u16,                                   // 0x002
    pub numPhysicalHeaps: u16,                           // 0x004
    pub heaps: *mut *mut IMemoryHeap,                    // 0x008
    pub allowOtherContextAllocs: *mut bool,              // 0x010
    pub heapsByContext: [*mut IMemoryHeap; 127],         // 0x018
    pub threadScrapHeap: *mut ThreadScrapHeap,           // 0x410
    pub physicalHeaps: *mut *mut IMemoryHeap,            // 0x418
    pub bigAllocHeap: *mut IMemoryHeap,                  // 0x420
    pub emergencyHeap: *mut IMemoryHeap,                 // 0x428
    pub smallBlockAllocator: *mut BSSmallBlockAllocator, // 0x430
    pub compactingStore: *mut CompactingStore::Store,    // 0x438
    pub externalHavokAllocator: *mut IMemoryHeap,        // 0x440
    pub specialHeaps: bool,                              // 0x448
    pub allowPoolUse: bool,                              // 0x449
    pub pad44A: [u8; 2],                                 // 0x44A
    pub sysAllocBytes: u32,                              // 0x44C
    pub mallocBytes: u32,                                // 0x450
    pub alignmentForPools: u32,                          // 0x454
    pub mainThreadMemoryProblemPassSignal: u32,          // 0x458
    pub failedAllocationSize: usize,                     // 0x460
    pub numMemoryProblemPassesRun: u32,                  // 0x468
    pub timeOfLastMemoryProblemPass: usize,              // 0x470
    pub defaultHeap: *mut IMemoryHeap,                   // 0x478
}

const _: () = {
    assert!(core::mem::size_of::<MemoryManager>() == 0x480);
};

/// Virtual table for `MemoryManager` with function pointers.
#[repr(C)]
pub struct MemoryManagerVtbl {
    pub GetSingleton: fn() -> *mut MemoryManager,
    pub Allocate: fn(
        this: *mut MemoryManager,
        size: usize,
        alignment: i32,
        alignment_required: bool,
    ) -> *mut c_void,
    pub Deallocate: fn(this: *mut MemoryManager, mem: *mut c_void, alignment_required: bool),
    pub GetThreadScrapHeap: fn(this: *mut MemoryManager) -> *mut ScrapHeap,
    pub Reallocate: fn(
        this: *mut MemoryManager,
        old_mem: *mut c_void,
        new_size: usize,
        alignment: i32,
        aligned: bool,
    ) -> *mut c_void,
    pub RegisterMemoryManager: fn(this: *mut MemoryManager),
}

impl MemoryManager {
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 11045, ae_id = 11141)]
    pub unsafe fn get_singleton() -> Option<NonNull<MemoryManager>> {}

    /// Allocate memory.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 66859, ae_id = 68115)]
    pub unsafe fn allocate(
        &mut self,
        size: usize,
        alignment: i32,
        alignment_required: bool,
    ) -> *mut c_void {
    }

    /// Deallocate memory.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 66861, ae_id = 68117)]
    pub unsafe fn deallocate(&mut self, mem: *mut c_void, alignment_required: bool) {}

    /// Get thread-local scrap heap.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 66841, ae_id = 68088)]
    pub unsafe fn get_thread_scrap_heap(&mut self) -> *mut ScrapHeap {}

    /// Reallocate memory.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 66860, ae_id = 68116)]
    pub unsafe fn reallocate(
        &mut self,
        old_mem: *mut c_void,
        new_size: usize,
        alignment: i32,
        aligned: bool,
    ) -> *mut c_void {
    }

    /// Register the memory manager.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 35199, ae_id = 36091)]
    pub unsafe fn register_memory_manager(&mut self) {}
}
