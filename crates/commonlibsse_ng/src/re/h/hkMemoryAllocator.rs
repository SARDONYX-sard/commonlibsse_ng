use core::ffi::c_void;

use crate::re::hkBaseTypes::hkResult_CEnum;
use crate::re::offsets_rtti::RTTI_hkMemoryAllocator;
use crate::re::offsets_vtable::VTABLE_hkMemoryAllocator;
use crate::rel::id::VariantID;

#[repr(C)]
pub struct hkMemoryAllocator {
    pub vtbl: *const hkMemoryAllocatorVtbl,
}

const _: () = assert!(core::mem::size_of::<hkMemoryAllocator>() == 0x8);

impl hkMemoryAllocator {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_hkMemoryAllocator;
    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_hkMemoryAllocator;
}

/// NOTE: Unlike the C++ implementation, `*mut u8` instead of `*mut c_void` for allocated memory blocks
#[repr(C)]
pub struct hkMemoryAllocatorVtbl {
    pub CxxDrop: unsafe extern "C" fn(this: *mut hkMemoryAllocator), // 0x00

    pub BlockAlloc: extern "C" fn(this: *mut hkMemoryAllocator, num_bytes: i32) -> *mut u8, // 0x01
    pub BlockFree: extern "C" fn(this: *mut hkMemoryAllocator, ptr: *mut u8, num_bytes: i32), // 0x02
    pub BufAlloc:
        extern "C" fn(this: *mut hkMemoryAllocator, req_num_bytes_in_out: &mut i32) -> *mut u8, // 0x03
    pub BufFree: extern "C" fn(this: *mut hkMemoryAllocator, ptr: *mut u8, num_bytes: i32), // 0x04
    pub BufRealloc: extern "C" fn(
        this: *mut hkMemoryAllocator,
        old_ptr: *mut u8,
        old_num_bytes: i32,
        req_num_bytes_in_out: &mut i32,
    ) -> *mut u8, // 0x05
    pub BlockAllocBatch: extern "C" fn(
        this: *mut hkMemoryAllocator,
        ptrs_out: *mut *mut u8,
        num_ptrs: i32,
        block_size: i32,
    ), // 0x06
    pub BlockFreeBatch: extern "C" fn(
        this: *mut hkMemoryAllocator,
        ptrs_in: *mut *mut u8,
        num_ptrs: i32,
        block_size: i32,
    ), // 0x07
    pub GetMemoryStatistics:
        extern "C" fn(this: *mut hkMemoryAllocator, usage: &mut MemoryStatistics), // 0x08
    pub GetAllocatedSize:
        extern "C" fn(this: *mut hkMemoryAllocator, obj: *const c_void, num_bytes: i32) -> i32, // 0x09
    pub ResetPeakMemoryStatistics: extern "C" fn(this: *mut hkMemoryAllocator), // 0x0A
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum MemoryState {
    Ok = 0,
    OutOfMemory = 1,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct MemoryStatistics {
    pub allocated: i64,      // 0x00
    pub inUse: i64,          // 0x08
    pub peakInUse: i64,      // 0x10
    pub available: i64,      // 0x18
    pub totalAvailable: i64, // 0x20
    pub largestBlock: i64,   // 0x28
}
const _: () = assert!(core::mem::size_of::<MemoryStatistics>() == 0x30);

impl MemoryStatistics {
    pub const INFINITE_SIZE: i64 = -1;
}

#[repr(C)]
pub struct ExtendedInterface {
    pub vtbl: *const ExtendedInterfaceVtbl,
}
const _: () = assert!(core::mem::size_of::<ExtendedInterface>() == 0x8);

pub type MemoryWalkCallback =
    unsafe extern "C" fn(start: *mut (), size: usize, allocated: bool, pool: i32, param: *mut ());

#[repr(C)]
pub struct ExtendedInterfaceVtbl {
    pub CxxDrop: unsafe extern "C" fn(this: *mut ExtendedInterface), // 00
    pub GarbageCollect: extern "C" fn(this: *mut ExtendedInterface), // 01
    pub IncrementalGarbageCollect: extern "C" fn(this: *mut ExtendedInterface, num_blocks: i32), // 02
    pub SetMemorySoftLimit: extern "C" fn(this: *mut ExtendedInterface, max_memory: usize) -> i32, // 03
    pub GetMemorySoftLimit: extern "C" fn(this: *const ExtendedInterface) -> usize, // 04
    pub CanAllocTotal: extern "C" fn(this: *mut ExtendedInterface, num_bytes: i32) -> bool, // 05
    pub WalkMemory: extern "C" fn(
        this: *mut ExtendedInterface,
        callback: MemoryWalkCallback,
        param: *mut c_void,
    ) -> hkResult_CEnum,
}
