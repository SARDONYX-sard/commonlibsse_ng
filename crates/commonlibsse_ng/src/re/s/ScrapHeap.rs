use crate::re::IMemoryStore::{IMemoryStore, IMemoryStoreVtbl};
use crate::re::offsets_vtable::VTABLE_ScrapHeap;
use crate::{re::offsets_rtti::RTTI_ScrapHeap, rel::id::VariantID};
use core::ffi::c_void;
use core::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct ScrapHeap {
    pub __base: IMemoryStore,
    pub smallBlocks: [*mut FreeBlock; 6], // 08
    pub freeList: *mut FreeTreeNode,      // 38
    pub lastBlock: *mut Block,            // 40
    pub baseAddress: *mut c_void,         // 48
    pub endAddress: *mut c_void,          // 50
    pub commitEnd: *mut c_void,           // 58
    pub reserveSize: usize,               // 60
    pub minCommit: usize,                 // 68
    pub totalAllocated: usize,            // 70
    pub keepPagesRequest: u32,            // 78
    pub totalFreeBlocks: u32,             // 7C
    pub freeSmallBlocks: u32,             // 80
    pub totalAllocatedBlocks: u32,        // 84
    pub pmpBarrier: u32,                  // 88
}
const _: () = assert!(core::mem::size_of::<ScrapHeap>() == 0x90);

impl Default for ScrapHeap {
    fn default() -> Self {
        Self::new()
    }
}

impl ScrapHeap {
    pub const RTTI: VariantID = RTTI_ScrapHeap;
    pub const VTABLE: [VariantID; 1] = VTABLE_ScrapHeap;

    pub const fn new() -> Self {
        Self {
            __base: IMemoryStore::new(),
            smallBlocks: [ptr::null_mut(); 6],
            freeList: ptr::null_mut(),
            lastBlock: ptr::null_mut(),
            baseAddress: ptr::null_mut(),
            endAddress: ptr::null_mut(),
            commitEnd: ptr::null_mut(),
            reserveSize: 1 << 26,
            minCommit: 1 << 17,
            totalAllocated: 0,
            keepPagesRequest: 0,
            totalFreeBlocks: 0,
            freeSmallBlocks: 0,
            totalAllocatedBlocks: 0,
            pmpBarrier: 0,
        }
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 66884, ae_id = 68144)]
    pub unsafe fn allocate(&mut self, size: usize, alignment: usize) -> *mut c_void {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 66885, ae_id = 68146)]
    pub unsafe fn deallocate(&mut self, mem: *mut c_void) -> *mut c_void {}
}

impl Drop for ScrapHeap {
    fn drop(&mut self) {
        use windows::Win32::System::Memory::{MEM_RELEASE, VirtualFree};

        if let Err(_err) = unsafe { VirtualFree(self.baseAddress, 0, MEM_RELEASE) } {
            #[cfg(feature = "tracing")]
            tracing::error!("Failed `VirtualFree`: {_err}")
        };
    }
}

pub struct ScrapHeapVtbl {
    pub __base: IMemoryStoreVtbl,
}

#[repr(C)]
#[derive(Debug)]
pub struct Block {
    pub sizeFlags: usize, // 00
    pub prev: *mut Block, // 08
}
const _: () = assert!(core::mem::size_of::<Block>() == 0x10);

#[repr(C)]
#[derive(Debug)]
pub struct FreeBlock {
    pub __base: Block,         // 00
    pub left: *mut FreeBlock,  // 10
    pub right: *mut FreeBlock, // 18
}
const _: () = assert!(core::mem::size_of::<FreeBlock>() == 0x20);

#[repr(C)]
#[derive(Debug)]
pub struct FreeTreeNode {
    pub __base: Block,                // 00
    pub root: *mut *mut FreeTreeNode, // 10
    pub leftNode: *mut FreeTreeNode,  // 18
    pub rightNode: *mut FreeTreeNode, // 20
    pub parentAndBlack: usize,        // 28
}
const _: () = assert!(core::mem::size_of::<FreeTreeNode>() == 0x30);
