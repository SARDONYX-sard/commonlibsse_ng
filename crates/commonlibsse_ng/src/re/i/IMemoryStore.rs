use crate::re::IMemoryStoreBase::{IMemoryStoreBase, IMemoryStoreBaseVtbl};
use crate::re::offsets_rtti::RTTI_IMemoryStore;
use crate::re::offsets_vtable::VTABLE_IMemoryStore;
use crate::rel::id::VariantID;

/// Virtual table for `IMemoryStore`.
#[repr(C)]
pub struct IMemoryStoreVtbl {
    pub __base: IMemoryStoreBaseVtbl,

    pub AllocateAlignImpl:
        unsafe extern "C" fn(this: *mut IMemoryStore, size: usize, alignment: u32) -> *mut u8,
    pub DeallocateAlignImpl: unsafe extern "C" fn(this: *mut IMemoryStore, block: *mut *mut u8),
    pub TryAllocateImpl:
        unsafe extern "C" fn(this: *mut IMemoryStore, size: usize, alignment: u32) -> *mut u8,
}

/// Memory store interface extending `IMemoryStoreBase`.
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct IMemoryStore {
    pub __base: IMemoryStoreBase,
}

impl IMemoryStore {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_IMemoryStore;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_IMemoryStore;

    /// Allocate aligned memory.
    #[inline]
    pub fn allocate_align(&mut self, size: usize, alignment: u32) -> *mut u8 {
        unsafe { (self.vtable().AllocateAlignImpl)(self, size, alignment) }
    }

    /// Deallocate aligned memory.
    #[inline]
    pub fn deallocate_align(&mut self, block: &mut *mut u8) {
        unsafe { (self.vtable().DeallocateAlignImpl)(self, block) }
    }

    /// Try allocating memory.
    #[inline]
    pub fn try_allocate(&mut self, size: usize, alignment: u32) -> *mut u8 {
        unsafe { (self.vtable().TryAllocateImpl)(self, size, alignment) }
    }

    #[inline]
    const fn vtable(&self) -> &'static IMemoryStoreVtbl {
        unsafe { &*self.__base.vtable.cast::<IMemoryStoreVtbl>() }
    }
}

const _: () = {
    assert!(std::mem::size_of::<IMemoryStore>() == 0x8);
};
