use crate::re::offsets_rtti::RTTI_NiRefObject;
use crate::re::offsets_vtable::VTABLE_NiRefObject;
use crate::rel::id::VariantID;
use core::sync::atomic::{AtomicU32, Ordering};

#[repr(C)]
pub struct NiRefObject {
    pub vtbl: *const NiRefObjectVtbl,
    pub _ref_count: AtomicU32,
    _pad: u32,
}
const _: () = assert!(core::mem::size_of::<NiRefObject>() == 0x10);

#[repr(C)]
pub struct NiRefObjectVtbl {
    /// C++ virtual class Destructor equivalent
    /// - override: `NiRefObject`
    pub CxxDrop: unsafe extern "C" fn(this: *mut NiRefObject), // 0x00
    /// `NiRefObject` virtual member function
    pub DeleteThis: unsafe extern "C" fn(this: *mut NiRefObject), // 0x01
}
const _: () = assert!(core::mem::size_of::<NiRefObjectVtbl>() == 0x10);

impl NiRefObject {
    pub const RTTI: VariantID = RTTI_NiRefObject;
    pub const VTABLE: [VariantID; 1] = VTABLE_NiRefObject;

    // Destructor
    pub fn delete_this(&self) {
        unsafe { ((*self.vtbl).DeleteThis)(self as *const _ as *mut _) };
    }

    // Increment ref count
    pub fn inc_ref_count(&self) {
        self._ref_count.fetch_add(1, Ordering::SeqCst);
    }

    // Decrement ref count
    pub fn dec_ref_count(&self) {
        if self._ref_count.fetch_sub(1, Ordering::SeqCst) == 1 {
            self.delete_this();
        }
    }

    // Get ref count
    pub fn get_ref_count(&self) -> u32 {
        self._ref_count.load(Ordering::SeqCst)
    }

    // Static method to get total object count
    pub fn get_total_object_count() -> &'static AtomicU32 {
        // Replace with actual relocation code as needed
        static TOTAL_OBJECT_COUNT: AtomicU32 = AtomicU32::new(0);
        &TOTAL_OBJECT_COUNT
    }
}
