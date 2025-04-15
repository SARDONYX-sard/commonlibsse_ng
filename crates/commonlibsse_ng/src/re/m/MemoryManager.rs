pub mod SimpleArray;
mod alloc;

pub use self::alloc::{alloc, alloc_zeroed, dealloc, global, global::TESGlobalAlloc, realloc};

use core::ffi::c_void;
use core::ptr;

use crate::re::BSSmallBlockAllocator::BSSmallBlockAllocator;
use crate::re::CompactingStore;
use crate::re::IMemoryHeap::IMemoryHeap;
use crate::re::ScrapHeap::ScrapHeap;

/// Represents a thread-local heap in the memory manager.
#[repr(C)]
pub struct ThreadScrapHeap {
    pub heap: ScrapHeap,            // 0x00
    pub next: *mut ThreadScrapHeap, // 0x90
    pub owningThread: u32,          // 0x98
    pub pad: u32,                   // 0x9C
}
const _: () = assert!(core::mem::size_of::<ThreadScrapHeap>() == 0xA0);

/// Memory manager interface
#[repr(C)]
#[derive(Debug)]
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
const _: () = assert!(core::mem::size_of::<MemoryManager>() == 0x480);

impl Default for MemoryManager {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryManager {
    #[inline]
    pub const fn new() -> Self {
        Self {
            initialized: false,
            numHeaps: 0,
            numPhysicalHeaps: 0,
            heaps: ptr::null_mut(),
            allowOtherContextAllocs: ptr::null_mut(),
            heapsByContext: [ptr::null_mut(); 127],
            threadScrapHeap: ptr::null_mut(),
            physicalHeaps: ptr::null_mut(),
            bigAllocHeap: ptr::null_mut(),
            emergencyHeap: ptr::null_mut(),
            smallBlockAllocator: ptr::null_mut(),
            compactingStore: ptr::null_mut(),
            externalHavokAllocator: ptr::null_mut(),
            specialHeaps: false,
            allowPoolUse: true,
            pad44A: [0; 2],
            sysAllocBytes: 0,
            mallocBytes: 0,
            alignmentForPools: 0,
            mainThreadMemoryProblemPassSignal: 0,
            failedAllocationSize: 0,
            numMemoryProblemPassesRun: 0,
            timeOfLastMemoryProblemPass: 0,
            defaultHeap: ptr::null_mut(),
        }
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 11045, ae_id = 11141)]
    pub unsafe fn GetSingleton() -> *mut MemoryManager {}

    /// Allocate memory.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 66859, ae_id = 68115)]
    pub unsafe fn Allocate(
        &mut self,
        size: usize,
        alignment: i32,
        alignment_required: bool,
    ) -> *mut c_void {
    }

    /// Deallocate memory.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 66861, ae_id = 68117)]
    pub unsafe fn Deallocate(&mut self, mem: *mut c_void, alignment_required: bool) {}

    /// Get thread-local scrap heap.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 66841, ae_id = 68088)]
    pub unsafe fn GetThreadScrapHeap(&mut self) -> *mut ScrapHeap {}

    /// Reallocate memory.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 66860, ae_id = 68116)]
    pub unsafe fn Reallocate(
        &mut self,
        old_mem: *mut c_void,
        new_size: usize,
        alignment: i32,
        aligned: bool,
    ) -> *mut c_void {
    }

    /// Register the memory manager.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 35199, ae_id = 36091)]
    pub unsafe fn RegisterMemoryManager(&mut self) {}
}

/// # Safety
#[inline]
pub unsafe fn malloc(size: usize) -> *mut c_void {
    unsafe { MemoryManager::GetSingleton().as_mut() }
        .map_or(ptr::null_mut(), |heap| unsafe { heap.Allocate(size, 0, false) })
}

/// # Safety
#[inline]
pub unsafe fn free(ptr: *mut c_void) {
    unsafe {
        if let Some(heap) = MemoryManager::GetSingleton().as_mut() {
            heap.Deallocate(ptr, false);
        };
    }
}
