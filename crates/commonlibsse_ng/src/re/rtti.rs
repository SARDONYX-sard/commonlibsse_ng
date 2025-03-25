//! # RTTI Structure Overview
//!
//! ASCII art representing the RTTI (Run-Time Type Information) structure in C++.
//!
//! - C++ src: https://godbolt.org/z/xn5f96E9s
//! -  AA ref: https://www.openrce.org/articles/full_view/23
//!
//! ```text
//!                               +------------------+
//!              +-------------+  | A::RTTI Complete |
//! +---------+  | A::vftable  |  | Object Locator   |
//! | class A |  |-------------|  |------------------|
//! |---------|  | &A_meta     |->| signature (0)    |         +-------------------------+   +-------------------------------+
//! | _vfptr  |->| &A::A_virt1 |  | offset (0)       |         | A::RTTI Type Descriptor |   | A::RTTI Base Class Descriptor |
//! | a1      |  | &A::A_virt2 |  | cdOffset (0)     |         |-------------------------|   |-------------------------------|
//! +---------+  +-------------+  | pTypeDescriptor  |-------->| pVFTable                |-->| pTypeDescriptor               |<-------+
//!                               | pClassDescriptor |---+     | spare                   |   | numContainedBases (0)         |        |
//!                               +------------------+   |     | name (?AVA@@)           |   | PMD where (0, -1, 0)          |        |
//!                                                      |     +-------------------------+   | attributes (0)                |        |
//!                                                      |                                   +-------------------------------+        |
//!                                                      |     +------------------------------------+                                 |
//!                                                      |     | A::RTTI Class Hierarchy Descriptor |                                 |
//!                                                      |     |------------------------------------|                                 |
//!                                                      +---->| signature (0)                      |   +--------------------------+  |
//!                                                            | attributes (0)                     |   | A::RTTI Base Class Array |  |
//!                                                            | numBaseClasses (1)                 |   |--------------------------|  |
//!                                                            | pBaseClassArray                    |-->| &A_BCD                   |--+
//!                                                            +------------------------------------+   +--------------------------+
//!
//!                               +------------------+
//!              +-------------+  | B::RTTI Complete |
//! +---------+  | B::vftable  |  | Object Locator   |
//! | class B |  |-------------|  |------------------|
//! |---------|  | &B_meta     |->| signature (0)    |         +-------------------------+   +-------------------------------+
//! | _vfptr  |->| &B::B_virt1 |  | offset (0)       |         | B::RTTI Type Descriptor |   | B::RTTI Base Class Descriptor |
//! | b1      |  | &B::B_virt2 |  | cdOffset (0)     |         |-------------------------|   |-------------------------------|
//! | b2      |  +-------------+  | pTypeDescriptor  |-------->| pVFTable                |-->| pTypeDescriptor               |<-------+
//! +---------+                   | pClassDescriptor |---+     | spare                   |   | numContainedBases (0)         |        |
//!                               +------------------+   |     | name (?AVA@@)           |   | PMD where (0, -1, 0)          |        |
//!                                                      |     +-------------------------+   | attributes (0)                |        |
//!                                                      |                                   +-------------------------------+        |
//!                                                      |     +------------------------------------+                                 |
//!                                                      |     | B::RTTI Class Hierarchy Descriptor |                                 |
//!                                                      |     |------------------------------------|                                 |
//!                                                      +---->| signature (0)                      |   +--------------------------+  |
//!                                                            | attributes (0)                     |   | B::RTTI Base Class Array |  |
//!                                                            | numBaseClasses (1)                 |   |--------------------------|  |
//!                                                            | pBaseClassArray                    |-->| &B_BCD                   |--+
//!                                                            +------------------------------------+   +--------------------------+
//!
//!
//!                                         +----------------------+
//!                 +------------------+    | C::RTTI Complete     |
//! +----------+    | C::vftable for A |    | Object Locator for A |
//! | class C  |    |------------------|    |----------------------|
//! |----------|    | &C_meta_A        |--->| signature (0)        |      +-------------------------+   +-------------------------------+
//! | _vfptr_A |--> | &C::A_virt1      |    | offset (0)           |      | C::RTTI Type Descriptor |   | C::RTTI Base Class Descriptor |
//! | a1       |    | &C::A_virt2      |    | cdOffset (0)         |      |-------------------------|   |-------------------------------|
//! | _vfptr_B |-+  +------------------+    | pTypeDescriptor      |----->| pVFTable                |-->| pTypeDescriptor               |
//! | b1       | |                          | pClassDescriptor     |--+   | spare                   |   | numContainedBases (2)         |
//! | b2       | |                          +----------------------+  |   | name (?AVC@@)           |   | PMD where (8, -1, 0)          |
//! | c1       | |                          +----------------------+  |   +-------------------------+   | attributes (0)                |
//! +----------+ |   +------------------+   | C::RTTI Complete     |  |                                 +-------------------------------+
//!              |   | C::vftable for B |   | Object Locator for B |  |
//!              |   |------------------|   |----------------------|  |
//!              +-> | &C_meta_A        |-->| signature (0)        |  |
//!                  | &C::A_virt1      |   | offset (8)           |  |  +----------------------+
//!                  | &C::A_virt2      |   | cdOffset (0)         |  |  | C::RTTI Class        |
//!                  +------------------+   | pTypeDescriptor      |  |  | Hierarchy Descriptor |
//!                                         | pClassDescriptor     |  |  |----------------------|
//!                                         +----------------------+  +->| signature (0)        |  +------------------------------------+
//!                                                                      | attributes (1)       |  | C::RTTI Base Class Array           |
//!                                                                      | numBaseClasses (3)   |  |------------------------------------|
//!                                                                      | pBaseClassArray      |->| &C_BCD -> C::RTTI Base Class Desc. |
//!                                                                      +----------------------+  | &A_BCD -> A::RTTI Base Class Desc. |
//!                                                                                                | &B_BCD -> B::RTTI Base Class Desc. |
//!                                                                                                +------------------------------------+
//! ```

use crate::re::CxxVirtClass;
use crate::rel::ResolvableAddress as _;
use crate::rel::id::DataBaseError;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::NonNull;

pub mod msvc {
    use core::ffi::c_char;

    #[repr(C)]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TypeInfo {
        // This field is hidden ptr to virtual function table.
        __vtbl: &'static TypeInfoVTable, // 0

        _data: *mut std::ffi::c_void, // 08
        _name: [c_char; 1],           // 10
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct TypeInfoVTable {
        /// C++ Destructor
        delete: fn(), // 00
    }

    const _: () = {
        assert!(0x08 == core::mem::offset_of!(TypeInfo, _data));
        assert!(0x10 == core::mem::offset_of!(TypeInfo, _name));
        assert!(core::mem::size_of::<TypeInfo>() == 0x18);
    };

    impl TypeInfo {
        /// Get mangled name of this class.
        pub const fn mangled_name(&self) -> *const c_char {
            self._name.as_ptr()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RVA<T> {
    _rva: u32, // 00
    _maker: PhantomData<T>,
}

const _: () = {
    assert!(core::mem::size_of::<RVA<*const c_void>>() == 0x4);
};

impl<T> RVA<T> {
    pub const fn new(rva: u32) -> Self {
        Self { _rva: rva, _maker: PhantomData::<T> }
    }

    pub fn get(&self) -> Option<NonNull<T>> {
        use crate::rel::offset::Offset;

        if self.is_good() {
            Some(Offset::new(self._rva as usize).address().ok()?.cast())
        } else {
            None
        }
    }

    pub const fn offset(&self) -> u32 {
        self._rva
    }

    const fn is_good(&self) -> bool {
        self._rva != 0
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PMD {
    m_disp: i32, // 0
    p_disp: i32, // 4
    v_disp: i32, // 8
}

const _: () = {
    assert!(0x00 == core::mem::offset_of!(PMD, m_disp));
    assert!(0x04 == core::mem::offset_of!(PMD, p_disp));
    assert!(0x08 == core::mem::offset_of!(PMD, v_disp));

    assert!(core::mem::size_of::<PMD>() == 0xc);
};

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct BaseClassDescriptorAttribute: u32 {
        const NONE = 0;
        const NOT_VISIBLE = 1 << 0;
        const Ambiguous = 1 << 1;
        const PRIVATE = 1 << 2;
        const PRIVATE_OR_PROTECTED_BASE = 1 << 3;
        const VIRTUAL = 1 << 4;
        const NON_POLYMORPHIC = 1 << 5;
        const HAS_HIERARCHY_DESCRIPTOR = 1 << 6;
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BaseClassDescriptor {
    type_descriptor: RVA<msvc::TypeInfo>,     // 00
    num_contained_bases: u32,                 // 04
    pmd: PMD,                                 // 08
    attributes: BaseClassDescriptorAttribute, // 14
}

const _: () = {
    assert!(0x00 == core::mem::offset_of!(BaseClassDescriptor, type_descriptor));
    assert!(0x04 == core::mem::offset_of!(BaseClassDescriptor, num_contained_bases));
    assert!(0x08 == core::mem::offset_of!(BaseClassDescriptor, pmd));
    assert!(0x14 == core::mem::offset_of!(BaseClassDescriptor, attributes));

    assert!(core::mem::size_of::<BaseClassDescriptor>() == 0x18);
};

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct ClassHierarchyDescriptorAttribute: u32 {
        const NO_INHERITANCE        = 0;
        const MULTIPLE_INHERITANCE  = 1 << 0;
        const VIRTUAL_INHERITANCE   = 1 << 1;
        const AMBIGUOUS_INHERITANCE = 1 << 2;
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassHierarchyDescriptor {
    signature: u32,                                // 00
    attributes: ClassHierarchyDescriptorAttribute, // 04
    num_base_classes: u32,                         // 08
    base_class_array: RVA<BaseClassDescriptor>,    // 0C
}

const _: () = {
    assert!(0x00 == core::mem::offset_of!(ClassHierarchyDescriptor, signature));
    assert!(0x04 == core::mem::offset_of!(ClassHierarchyDescriptor, attributes));
    assert!(0x08 == core::mem::offset_of!(ClassHierarchyDescriptor, num_base_classes));
    assert!(0x0C == core::mem::offset_of!(ClassHierarchyDescriptor, base_class_array));

    assert!(core::mem::size_of::<ClassHierarchyDescriptor>() == 0x10);
};

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompleteObjectLocator {
    signature: Signature,                            // 00
    offset: u32,                                     // 04
    ctor_disp_offset: u32,                           // 08
    type_descriptor: RVA<msvc::TypeInfo>,            // 0C
    class_descriptor: RVA<ClassHierarchyDescriptor>, // 10
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Signature {
    X86 = 0,
    X64 = 1,
}

const _: () = {
    assert!(0x00 == core::mem::offset_of!(CompleteObjectLocator, signature));
    assert!(0x04 == core::mem::offset_of!(CompleteObjectLocator, offset));
    assert!(0x08 == core::mem::offset_of!(CompleteObjectLocator, ctor_disp_offset));
    assert!(0x0C == core::mem::offset_of!(CompleteObjectLocator, type_descriptor));
    assert!(0x10 == core::mem::offset_of!(CompleteObjectLocator, class_descriptor));

    assert!(core::mem::size_of::<CompleteObjectLocator>() == 0x14);
};

/// RTTI Dynamic Cast function
///
/// # Safety
/// # Errors
#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 102238, ae_id = 109689, vr_id = 102238)]
pub unsafe fn rt_dynamic_cast(
    in_ptr: *mut c_void,
    vf_delta: i32,
    src_type: *mut c_void,
    target_type: *mut c_void,
    is_reference: i32,
) -> *mut c_void {
}

// TODO: Write tests
// FIXME: remove unwrap
/// # Safety
/// # Errors
/// # Panics
pub unsafe fn skyrim_cast<To, From>(from: *mut From) -> Result<*mut To, DataBaseError>
where
    From: 'static + CxxVirtClass,
    To: 'static + CxxVirtClass,
{
    use crate::rel::relocation::Relocation;
    let from_rtti = Relocation::new(From::rtti().address()?).cast::<c_void>();
    let to_rtti = Relocation::new(To::rtti().address()?).cast::<c_void>();

    Ok(unsafe {
        rt_dynamic_cast(from.cast::<c_void>(), 0, from_rtti.as_ptr(), to_rtti.as_ptr(), 0)
    }
    .cast())
}
