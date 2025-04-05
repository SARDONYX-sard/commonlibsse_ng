use crate::re::offsets_rtti::RTTI_BSTArrayBase__IAllocatorFunctor;
use crate::re::offsets_vtable::VTABLE_BSTArrayBase__IAllocatorFunctor;
use crate::rel::id::VariantID;
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IAllocatorFunctor {
    vtable_: *const IAllocatorFunctorVtbl,
}
const_assert_eq!(core::mem::size_of::<IAllocatorFunctor>(), 0x8);

impl IAllocatorFunctor {
    pub const RTTI: VariantID = RTTI_BSTArrayBase__IAllocatorFunctor;
    pub const VTABLE: [VariantID; 1] = VTABLE_BSTArrayBase__IAllocatorFunctor;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IAllocatorFunctorVtbl {
    pub Allocate: fn(this: *mut c_void, num: u32, elem_size: u32) -> bool,
    pub Reallocate: fn(
        this: *mut c_void,
        min_new_size_items: u32,
        front_copy_count: u32,
        back_copy_count: u32,
        elem_size: u32,
    ) -> bool,
    pub Deallocate: fn(this: *mut c_void) -> c_void,

    /// `~IAllocatorFunctor`
    pub CxxDrop: fn(this: *mut c_void) -> c_void,
}
