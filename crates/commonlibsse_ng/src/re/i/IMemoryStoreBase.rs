use crate::re::offsets_rtti::RTTI_IMemoryStoreBase;
use crate::re::offsets_vtable::VTABLE_IMemoryStoreBase;
use crate::rel::id::VariantID;
use core::ffi::c_char;

/// Represents memory statistics.
#[repr(C)]
#[derive(Debug)]
pub struct MemoryStats {
    pub name: *const c_char,  // 0x00
    pub usedSize: usize,      // 0x08
    pub committedSize: usize, // 0x10
    pub reservedSize: usize,  // 0x18
    pub overhead: u32,        // 0x20
    pub pad24: u32,           // 0x24
    pub freeSize: usize,      // 0x28
}

const _: () = {
    assert!(std::mem::size_of::<MemoryStats>() == 0x30);
};

impl Default for MemoryStats {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

/// Virtual table for `IMemoryStoreBase`.
#[repr(C)]
pub struct IMemoryStoreBaseVtbl {
    /// C++ `virtual ~IMemoryStoreBase`
    pub CxxDrop: unsafe extern "C" fn(this: *mut IMemoryStoreBase),
    pub Size: unsafe extern "C" fn(this: *const IMemoryStoreBase, mem: *const u8) -> usize,
    pub GetMemoryStats: unsafe extern "C" fn(this: *mut IMemoryStoreBase, stats: *mut MemoryStats),
    pub ContainsBlockImpl:
        unsafe extern "C" fn(this: *const IMemoryStoreBase, block: *const u8) -> bool,
}

/// Base memory store interface with virtual functions.
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct IMemoryStoreBase {
    pub vtable: *const IMemoryStoreBaseVtbl,
}

impl IMemoryStoreBase {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_IMemoryStoreBase;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_IMemoryStoreBase;

    pub const fn new(vtable: *const IMemoryStoreBaseVtbl) -> Self {
        Self { vtable }
    }

    /// Destructor
    ///
    /// # Safety
    /// Avoid double free
    pub unsafe fn drop(&mut self) {
        unsafe {
            ((*self.vtable).CxxDrop)(self);
        }
    }

    /// Get size of a memory block.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn size(&self, mem: *const u8) -> usize {
        unsafe { ((*self.vtable).Size)(self, mem) }
    }

    /// Get memory statistics.
    pub fn get_memory_stats(&mut self, stats: &mut MemoryStats) {
        unsafe {
            ((*self.vtable).GetMemoryStats)(self, stats);
        }
    }

    /// Check if the block is contained in the memory store.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn contains_block_impl(&self, block: *const u8) -> bool {
        unsafe { ((*self.vtable).ContainsBlockImpl)(self, block) }
    }
}

const _: () = {
    assert!(std::mem::size_of::<IMemoryStoreBase>() == 0x8);
};
