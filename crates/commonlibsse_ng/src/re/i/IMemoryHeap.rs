use core::ffi::{c_char, c_void};

use crate::re::offsets_rtti::RTTI_IMemoryHeap;
use crate::re::offsets_vtable::VTABLE_IMemoryHeap;
use crate::rel::id::VariantID;

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Default,Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MEM_CONTEXT: u32 {}
}

/// Represents the `HeapStats` struct.
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct HeapStats {
    pub heapName: *const c_char,     // 0x00
    pub memHeapSize: usize,          // 0x08
    pub memHeapCommitted: usize,     // 0x10
    pub memAllocatedToBlocks: usize, // 0x18
    pub numBlocks: i32,              // 0x20
    pub numFreeBlocks: i32,          // 0x24
    pub memFreeInBlocks: usize,      // 0x28
    pub memUsedInBlocks: usize,      // 0x30
    pub smallestFreeBlock: usize,    // 0x38
    pub largestFreeBlock: usize,     // 0x40
    pub heapOverhead: usize,         // 0x48
    pub freeListOverhead: usize,     // 0x50
    pub blockOverhead: usize,        // 0x58
    pub totalFree: usize,            // 0x60
}

const _: () = {
    assert!(std::mem::size_of::<HeapStats>() == 0x68);
};

/// Memory heap interface
#[repr(C)]
pub struct IMemoryHeap {
    pub vtable: *const IMemoryHeapVtbl,
}

#[repr(C)]
pub struct IMemoryHeapVtbl {
    pub CxxDrop: unsafe extern "C" fn(this: *mut IMemoryHeap), // 00

    pub Size: unsafe extern "C" fn(this: *const IMemoryHeap, mem: *const c_void) -> usize, // 01
    pub GetMemoryStats: unsafe extern "C" fn(this: *mut IMemoryHeap, stats: *mut HeapStats), // 02
    pub ContainsBlockImpl:
        unsafe extern "C" fn(this: *const IMemoryHeap, block: *const c_void) -> bool, // 03
    pub AllocateAlignImpl:
        unsafe extern "C" fn(this: *mut IMemoryHeap, size: usize, alignment: u32) -> *mut c_void, // 04
    pub DeallocateAlignImpl: unsafe extern "C" fn(this: *mut IMemoryHeap, block: *mut *mut c_void), // 05

    // IMemoryHeap-specific methods
    pub GetName: unsafe extern "C" fn(this: *const IMemoryHeap) -> *const c_char, // 07
    pub Allocate:
        unsafe extern "C" fn(this: *mut IMemoryHeap, size: usize, alignment: u32) -> *mut c_void, // 08
    pub Deallocate: unsafe extern "C" fn(this: *mut IMemoryHeap, mem: *mut c_void, alignment: u32), // 09
    pub PointerInHeap:
        unsafe extern "C" fn(this: *const IMemoryHeap, pointer: *const c_void) -> bool, // 0A
    pub TotalSize: unsafe extern "C" fn(this: *const IMemoryHeap, pointer: *const c_void) -> usize, // 0B
    pub GetHeapStats:
        unsafe extern "C" fn(this: *mut IMemoryHeap, stats: *mut HeapStats, full_block_info: bool), // 0C
    pub ShouldTrySmallBlockPools:
        unsafe extern "C" fn(this: *const IMemoryHeap, size: usize, context: MEM_CONTEXT) -> bool, // 0D
    pub GetPageSize: unsafe extern "C" fn(this: *const IMemoryHeap) -> u32, // 0E
}

impl IMemoryHeap {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_IMemoryHeap;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_IMemoryHeap;

    /// Get the name of the heap.
    ///
    /// # Safety
    pub unsafe fn get_name(&self) -> *const c_char {
        unsafe { ((*self.vtable).GetName)(self) }
    }

    /// Allocate memory with alignment.
    ///
    /// # Safety
    pub unsafe fn allocate(&mut self, size: usize, alignment: u32) -> *mut c_void {
        unsafe { ((*self.vtable).Allocate)(self, size, alignment) }
    }

    /// Deallocate memory.
    ///
    /// # Safety
    pub unsafe fn deallocate(&mut self, mem: *mut c_void, alignment: u32) {
        unsafe { ((*self.vtable).Deallocate)(self, mem, alignment) };
    }

    /// Check if a pointer is in the heap.
    ///
    /// # Safety
    pub unsafe fn pointer_in_heap(&self, pointer: *const c_void) -> bool {
        unsafe { ((*self.vtable).PointerInHeap)(self, pointer) }
    }

    /// Get the total size of a block in the heap.
    ///
    /// # Safety
    pub unsafe fn total_size(&self, pointer: *const c_void) -> usize {
        unsafe { ((*self.vtable).TotalSize)(self, pointer) }
    }

    /// Get the heap statistics.
    ///
    /// # Safety
    pub unsafe fn get_heap_stats(&mut self, stats: *mut HeapStats, full_block_info: bool) {
        unsafe { ((*self.vtable).GetHeapStats)(self, stats, full_block_info) };
    }

    /// Check if small block pools should be tried.
    ///
    /// # Safety
    pub unsafe fn should_try_small_block_pools(&self, size: usize, context: MEM_CONTEXT) -> bool {
        unsafe { ((*self.vtable).ShouldTrySmallBlockPools)(self, size, context) }
    }

    /// Get the page size.
    ///
    /// # Safety
    pub unsafe fn get_page_size(&self) -> u32 {
        unsafe { ((*self.vtable).GetPageSize)(self) }
    }
}
