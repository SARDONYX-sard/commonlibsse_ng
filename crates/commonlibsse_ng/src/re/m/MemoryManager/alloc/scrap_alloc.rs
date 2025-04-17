//! Scrap heap allocation rust like API(But for reasons of scrap heap reuse, it is a little different)
use core::alloc::Layout;
use core::ptr::NonNull;

use crate::re::MemoryManager::MemoryManager;
use crate::re::ScrapHeap::ScrapHeap;
use stdx::alloc::AllocError;

/// Attempts to allocate memory using the thread-local scrap heap managed by the custom memory system.
///
/// # Errors
/// Returns a pair of non-null pointers to the scrap heap and the allocated memory on success, or an `AllocError` if allocation fails.
///
/// # Safety
///
/// - The returned memory is uninitialized.
/// - The caller must ensure proper usage and deallocation of the returned pointer.
#[inline]
pub unsafe fn alloc(layout: Layout) -> Result<(NonNull<ScrapHeap>, NonNull<u8>), AllocError> {
    unsafe {
        let mem_manager = MemoryManager::GetSingleton().as_mut().ok_or(AllocError)?;

        let scrap_heap = mem_manager.GetThreadScrapHeap().as_mut().ok_or(AllocError)?;
        let ptr = scrap_heap.allocate(layout.size(), layout.align()).cast::<u8>();

        NonNull::new(ptr).map(|nn_ptr| (NonNull::from(scrap_heap), nn_ptr)).ok_or(AllocError)
    }
}

/// Attempts to allocate zero-initialized memory using the thread-local scrap heap.
///
/// # Errors
/// Returns a pair of non-null pointers to the scrap heap and the allocated memory on success, or an `AllocError` if allocation fails.
///
/// # Safety
///
/// - The caller must ensure proper usage and deallocation of the returned pointer.
/// - This function may write zero bytes to uninitialized memory.
#[inline]
pub unsafe fn alloc_zeroed(
    layout: Layout,
) -> Result<(NonNull<ScrapHeap>, NonNull<u8>), AllocError> {
    unsafe {
        let (scrap_heap, ptr) = alloc(layout)?;
        ptr.as_ptr().write_bytes(0, layout.size());
        Ok((scrap_heap, ptr))
    }
}

/// Attempts to reallocate memory previously allocated from a thread-local scrap heap.
///
/// # Errors
/// Returns a pair of non-null pointers to the scrap heap and the newly allocated memory on success,
/// or an `AllocError` if reallocation fails.
///
/// The old memory region is not automatically copied or deallocated.
///
/// # Safety
///
/// - `old_ptr` must be a pointer previously returned by [`alloc`] or [`alloc_zeroed`] using the same scrap heap and layout.
/// - The caller is responsible for copying any existing data and deallocating the old pointer if needed.
/// - The returned memory is uninitialized.
///
/// This function behaves more like a fresh allocation than a standard realloc.
#[inline]
pub unsafe fn realloc(
    mut scrap_heap: NonNull<ScrapHeap>,
    new_layout: Layout,
) -> Result<(NonNull<ScrapHeap>, NonNull<u8>), AllocError> {
    unsafe {
        let scrap_heap_ref = scrap_heap.as_mut();
        let new_ptr = scrap_heap_ref.allocate(new_layout.size(), new_layout.align()).cast::<u8>();

        NonNull::new(new_ptr).map(|nn_ptr| (scrap_heap, nn_ptr)).ok_or(AllocError)
    }
}

/// Deallocates memory previously allocated using [`alloc`] or [`alloc_zeroed`] with the same scrap heap.
///
/// # Safety
///
/// - `ptr` must have been returned by a previous call to [`alloc`] or [`alloc_zeroed`] with the same layout and scrap heap.
/// - Using an invalid pointer or mismatched layout may cause undefined behavior.
#[inline]
pub unsafe fn dealloc(mut scrap_heap: NonNull<ScrapHeap>, ptr: NonNull<u8>, layout: Layout) {
    let _ = layout;
    unsafe {
        let scrap_heap = scrap_heap.as_mut();
        let _ = scrap_heap.deallocate(ptr.cast().as_ptr());
    }
}
