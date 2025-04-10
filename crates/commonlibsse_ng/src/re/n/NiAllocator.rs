use core::ffi::{c_char, c_void};

use crate::re::offsets_rtti::RTTI_NiAllocator;
use crate::re::offsets_vtable::VTABLE_NiAllocator;
use crate::rel::id::VariantID;

#[derive(Debug)]
#[repr(C)]
pub struct NiAllocator {
    pub vtable: *const NiAllocatorVtbl,
}
const _: () = assert!(core::mem::size_of::<NiAllocator>() == 0x8);

impl NiAllocator {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_NiAllocator;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_NiAllocator;

    #[inline]
    pub fn vtable(&self) -> &NiAllocatorVtbl {
        debug_assert!(!self.vtable.is_null());
        unsafe { &*self.vtable }
    }
}

/// 9 methods
#[repr(C)]
pub struct NiAllocatorVtbl {
    pub Allocate: extern "C" fn(
        this: *mut NiAllocator,
        sizeInBytes: &mut usize,
        alignment: &mut usize,
        eventType: NiMemEventType,
        provideAccurateSizeOnDeallocate: bool,
        file: *const c_char,
        line: i32,
        function: *const c_char,
    ) -> *mut c_void,

    pub Deallocate: extern "C" fn(
        this: *mut NiAllocator,
        memory: *mut c_void,
        eventType: NiMemEventType,
        sizeInBytes: usize,
    ),

    pub Reallocate: extern "C" fn(
        this: *mut NiAllocator,
        memory: *mut c_void,
        sizeInBytes: *mut usize,
        alignment: *mut usize,
        eventType: NiMemEventType,
        provideAccurateSizeOnDeallocate: bool,
        size_current: usize,
        file: *const c_char,
        line: i32,
        function: *const c_char,
    ) -> *mut c_void,

    pub TrackAllocate: extern "C" fn(
        this: *mut NiAllocator,
        memory: *const c_void,
        sizeInBytes: usize,
        eventType: NiMemEventType,
        file: *const c_char,
        line: i32,
        function: *const c_char,
    ) -> bool,

    pub TrackDeallocate: extern "C" fn(
        this: *mut NiAllocator,
        memory: *const c_void,
        eventType: NiMemEventType,
    ) -> bool,

    pub Unk_06: extern "C" fn(this: *mut NiAllocator),

    pub Initialize: extern "C" fn(this: *mut NiAllocator),

    pub Shutdown: extern "C" fn(this: *mut NiAllocator),

    pub VerifyAddress: extern "C" fn(this: *mut NiAllocator, memory: *const c_void) -> bool,
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NiMemEventType {
    #[default]
    Unknown = 0,
    OperNew = 1,
    OperNewArray = 2,
    OperDelete = 3,
    OperDeleteArray = 4,
    Malloc = 5,
    Realloc = 6,
    AlignedMalloc = 7,
    AlignedRealloc = 8,
    Free = 9,
    AlignedFree = 10,
    ExternalAlloc = 11,
    ExternalFree = 12,
}
