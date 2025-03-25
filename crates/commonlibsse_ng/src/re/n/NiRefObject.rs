use crate::re::offsets_rtti::RTTI_NiRefObject;
use crate::re::offsets_vtable::VTABLE_NiRefObject;
use crate::rel::id::VariantID;
use core::sync::atomic::{AtomicU32, Ordering};

#[repr(C)]
#[derive(Debug)]
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

    /// Manual Destructor
    ///
    /// # Safety
    /// As long as not double free.
    pub unsafe fn delete_this(&mut self) {
        unsafe { ((*self.vtbl).DeleteThis)(self) };
    }

    // Get ref count
    pub fn get_ref_count(&self) -> u32 {
        self._ref_count.load(Ordering::SeqCst)
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 523912, ae_id = 410493)]
    pub unsafe fn get_total_object_count(&mut self) -> *const AtomicU32 {}
}

impl crate::re::NiSmartPointer::RefCountable for NiRefObject {
    #[inline]
    fn inc_ref_count(&self) {
        self._ref_count.fetch_add(1, Ordering::SeqCst);
    }

    #[inline]
    fn dec_ref_count(&mut self) {
        if self._ref_count.fetch_sub(1, Ordering::SeqCst) == 1 {
            unsafe { self.delete_this() }; // FIXME: Maybe unsafe
        }
    }
}

impl Drop for NiRefObject {
    fn drop(&mut self) {
        if let Some(count) = unsafe { self.get_total_object_count().as_ref() } {
            count.fetch_sub(1, Ordering::AcqRel);
        }
    }
}
