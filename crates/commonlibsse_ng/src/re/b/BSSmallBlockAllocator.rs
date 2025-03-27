use crate::re::{BSAtomic::BSCriticalSection, IMemoryStore::IMemoryStore};
use core::ptr::NonNull;

/// Represents a free block in the small block allocator.
#[repr(C)]
#[derive(Debug)]
pub struct FreeBlock {
    pub next: *mut FreeBlock, // 0x00
}

const _: () = {
    assert!(std::mem::size_of::<FreeBlock>() == 0x8);
};

impl Default for FreeBlock {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

/// Represents a block page used by the allocator.
#[repr(C)]
#[derive(Debug)]
pub struct BlockPage {
    pub left: *mut BlockPage,   // 0x00
    pub right: *mut BlockPage,  // 0x08
    pub blocks: *mut FreeBlock, // 0x10
    pub totalElem: u16,         // 0x18
    pub freeElem: u16,          // 0x1A
    pub pad1C: u32,             // 0x1C
}

const _: () = {
    assert!(std::mem::size_of::<BlockPage>() == 0x20);
};

impl Default for BlockPage {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

/// Represents an internal block page.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockPageInternal {
    pub left: *mut BlockPageInternal,  // 0x00
    pub right: *mut BlockPageInternal, // 0x08
    pub blocks: *mut FreeBlock,        // 0x10
    pub totalElem: u16,                // 0x18
    pub freeElem: u16,                 // 0x1A
    pub elemSize: u16,                 // 0x1C
    pub check: u16,                    // 0x1E
}
const _: () = {
    assert!(std::mem::size_of::<BlockPageInternal>() == 0x20);
};

impl Default for BlockPageInternal {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

/// Represents a pool for memory allocation.
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Pool {
    pub pageList: *mut BlockPage,  // 0x00
    pub currAlloc: *mut BlockPage, // 0x08
    pub totalFreeBlocks: u32,      // 0x10
    pub totalAllocatedBlocks: u32, // 0x14
    pub totalBytes: u32,           // 0x18
    pub elementSize: u32,          // 0x1C
    pub lock: BSCriticalSection,   // 0x20
}
const _: () = {
    assert!(std::mem::size_of::<Pool>() == 0x48);
};
impl Default for Pool {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

/// Represents a mega block page.
#[repr(C)]
#[derive(Debug)]
pub struct MegaBlockPage {
    pub mem: [u8; 0x1FE000],                   // 0x000000 - Raw memory block
    pub blockPages: [BlockPageInternal; 255],  // 0x1FE000
    pub left: Option<NonNull<MegaBlockPage>>,  // 0x1FFFE0
    pub right: Option<NonNull<MegaBlockPage>>, // 0x1FFFE8
    pub freeBlockPages: *mut BlockPage,        // 0x1FFFF0
    pub numFreeBlockPages: u16,                // 0x1FFFF8
    pub nextBlockPageAlloc: u16,               // 0x1FFFFA
    pub decommitted: bool,                     // 0x1FFFFC
    pub pad1FFFFD: u8,                         // 0x1FFFFD
    pub pad1FFFFE: u16,                        // 0x1FFFFE
}
const _: () = {
    assert!(core::mem::size_of::<MegaBlockPage>() == 0x200000); // 2MiB
};

impl MegaBlockPage {
    /// # Safety
    #[inline]
    pub unsafe fn alloc_zeroed() -> Option<NonNull<Self>> {
        const SELF_ALIGN: usize = core::mem::align_of::<MegaBlockPage>();
        const SELF_SIZE: usize = core::mem::size_of::<MegaBlockPage>();

        // Validate Layout at compile time.
        const _: () = {
            assert!(SELF_ALIGN != 0);
            assert!(SELF_ALIGN % 2 == 0);
            assert!(SELF_SIZE <= (isize::MAX as usize));
        };

        unsafe {
            let layout = core::alloc::Layout::from_size_align_unchecked(SELF_SIZE, SELF_ALIGN);
            NonNull::new(std::alloc::alloc_zeroed(layout).cast::<Self>())
        }
    }
}

/// The main small block allocator struct.
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BSSmallBlockAllocator {
    pub __base: IMemoryStore,                   // 0x0000
    pub pools: [Pool; 64],                      // 0x0008
    pub lock: BSCriticalSection,                // 0x1208
    pub addressSpaceSize: u32,                  // 0x1230
    pub pad1234: u32,                           // 0x1234
    pub allocBase: *mut u8,                     // 0x1238
    pub blockPageCommitMin: *mut u8,            // 0x1240
    pub blockPageCommit: *mut u8,               // 0x1248
    pub megaBlockPageList: *mut MegaBlockPage,  // 0x1250
    pub megaBlockCurrAlloc: *mut MegaBlockPage, // 0x1258
    pub totalFreeBlockPages: u32,               // 0x1260
    pub allowDecommits: bool,                   // 0x1264
    pub pad1265: u8,                            // 0x1265
    pub pad1266: u16,                           // 0x1266
}

const _: () = {
    assert!(std::mem::size_of::<BSSmallBlockAllocator>() == 0x1268);
};

impl Default for BSSmallBlockAllocator {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

/// Virtual table for `BSSmallBlockAllocator` to simulate the C++ vtable.
#[repr(C)]
pub struct BSSmallBlockAllocatorVtbl {
    /// C++ `virtual ~BSSmallBlockAllocator()`
    pub CxxDrop: unsafe extern "C" fn(this: &mut BSSmallBlockAllocator),

    pub Size: unsafe extern "C" fn(this: &BSSmallBlockAllocator, block: *const u8) -> usize,
    pub GetMemoryStats: unsafe extern "C" fn(this: &BSSmallBlockAllocator, stats: *mut ()) -> (),
    pub ContainsBlockImpl:
        unsafe extern "C" fn(this: &BSSmallBlockAllocator, block: *const u8) -> bool,
    pub AllocateAlignImpl:
        unsafe extern "C" fn(this: &mut BSSmallBlockAllocator, size: usize, align: u32) -> *mut u8,
    pub DeallocateAlignImpl:
        unsafe extern "C" fn(this: &mut BSSmallBlockAllocator, free_block: *mut *mut u8),
    pub TryAllocateImpl:
        unsafe extern "C" fn(this: &mut BSSmallBlockAllocator, size: usize, align: u32) -> *mut u8,
}
