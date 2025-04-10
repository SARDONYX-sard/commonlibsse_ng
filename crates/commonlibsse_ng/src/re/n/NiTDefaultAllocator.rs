use core::{ffi::c_void, marker::PhantomData, ptr};

use crate::re::NiTCollection::{NiFree, NiMalloc};

#[derive(Debug)]
#[repr(C)]
pub struct NiTDefaultAllocator<T> {
    marker: PhantomData<T>,
}

impl<T> NiTDefaultAllocator<T> {
    #[inline]
    pub fn allocate(&mut self) -> *mut c_void {
        NiMalloc(core::mem::size_of::<T>()).map_or(ptr::null_mut(), |p| p.cast().as_ptr())
    }

    #[inline]
    pub fn deallocate(&mut self, ptr: *mut c_void) {
        NiFree(ptr);
    }
}
