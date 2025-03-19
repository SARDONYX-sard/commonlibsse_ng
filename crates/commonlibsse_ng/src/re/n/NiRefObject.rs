use std::sync::atomic::{AtomicU32, Ordering};

use crate::re::offsets_rtti::RTTI_NiRefObject;
use crate::re::offsets_vtable::VTABLE_NiRefObject;
use crate::rel::id::VariantID;

#[repr(C)]
pub struct NiRefObject {
    vtbl: *const Vtbl,
    _ref_count: AtomicU32,
    _pad: u32,
}

#[repr(C)]
pub struct Vtbl {
    /// C++ virtual destructor
    _drop: unsafe extern "C" fn(*mut NiRefObject),

    delete_this: unsafe extern "C" fn(*mut NiRefObject),
}

impl NiRefObject {
    pub const RTTI: VariantID = RTTI_NiRefObject;
    pub const VTABLE: [VariantID; 1] = VTABLE_NiRefObject;

    // Destructor
    pub fn delete_this(&self) {
        unsafe {
            let delete_fn = (*self.vtbl).delete_this;
            delete_fn(self as *const _ as *mut _);
        }
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

// Ensure the size of NiRefObject matches C++ equivalent size
#[cfg(target_arch = "x86_64")]
const _: () = assert!(std::mem::size_of::<NiRefObject>() == 0x10);

#[cfg(target_arch = "x86_64")]
const _: () = assert!(std::mem::size_of::<Vtbl>() == 0x10); // Size of vtable
