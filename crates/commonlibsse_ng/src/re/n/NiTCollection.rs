use core::alloc::Layout;
use core::ffi::c_void;
use core::ptr::NonNull;

use crate::re::NiAllocator::NiMemEventType;
use crate::re::NiMemManager::NiMemManager;

#[inline]
pub fn NiMalloc(size_in_bytes: usize) -> Option<NonNull<u8>> {
    NiMemManager::get_singleton_mut()?.allocate(
        size_in_bytes,
        0,
        NiMemEventType::Malloc,
        false,
        None,
        None,
        None,
    )
}

#[inline]
pub fn NiAlignedMalloc(layout: Layout) -> Option<NonNull<u8>> {
    let size_in_bytes = layout.size();
    let alignment = layout.align();

    NiMemManager::get_singleton_mut()?.allocate(
        size_in_bytes,
        alignment,
        NiMemEventType::AlignedMalloc,
        false,
        None,
        None,
        None,
    )
}

#[inline]
pub fn NiFree(mem: *mut c_void) {
    if let Some(mem_manager) = NiMemManager::get_singleton_mut() {
        mem_manager.deallocate(mem, NiMemEventType::Free, None);
    }
}
