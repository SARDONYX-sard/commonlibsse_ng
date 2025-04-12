use crate::re::hkMemoryAllocator::hkMemoryAllocatorVtbl;
use crate::re::offsets_rtti::RTTI_hkContainerHeapAllocator__Allocator;
use crate::re::offsets_vtable::VTABLE_hkContainerHeapAllocator__Allocator;
use crate::rel::id::VariantID;

/// C++ classes without members.
///
/// Note: Inheritance of this when reproducing class inheritance in Rust types will result in an undersized class due to EBO (Empty Base Optiomization).
///
/// Do not use this when inheriting.
#[repr(C)]
pub struct hkContainerHeapAllocator {
    // Unlike Rust, C++ has a 1-byte address even when there is no member.
    address: u8,
}
const _: () = assert!(core::mem::size_of::<hkContainerHeapAllocator>() == 0x1);

impl hkContainerHeapAllocator {
    /// Gets the singleton instance of `Allocator`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut Allocator",
        default = "None",
        deref_once,
        id(se = 510713, ae = 383828)
    )]
    pub fn get_singleton() -> Option<&'static Allocator> {
        |deref_type: DerefType| unsafe { deref_type.as_ref() }
    }

    /// Gets the mutable singleton instance of `Allocator`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut Allocator",
        default = "None",
        deref_once,
        id(se = 510713, ae = 383828)
    )]
    pub fn get_singleton_mut() -> Option<&'static mut Allocator> {
        |deref_type: DerefType| unsafe { deref_type.as_mut() }
    }
}

/// hkContainerHeapAllocator namespace C++ class
#[repr(C)]
pub struct Allocator {
    pub vtbl: *const AllocatorVtbl,
}
const _: () = assert!(core::mem::size_of::<Allocator>() == 0x8);

impl Allocator {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_hkContainerHeapAllocator__Allocator;
    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_hkContainerHeapAllocator__Allocator;
}

#[repr(C)]
pub struct AllocatorVtbl {
    pub __base: hkMemoryAllocatorVtbl,
}
