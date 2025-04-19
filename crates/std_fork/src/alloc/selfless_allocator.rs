use core::alloc::Layout;
use core::ptr::{self, NonNull};

use stdx::alloc::AllocError;

/// A global allocator interface that does not rely on `self`.
///
/// `SelflessAllocator` is suitable for stateless or global allocators
/// where the allocation logic does not require access to instance state.
///
/// This trait is intended for use cases such as custom global allocators,
/// embedded environments, or statically known allocators.
///
/// # Safety
///
/// Implementors must ensure that all memory returned:
/// - is non-null,
/// - properly aligned as per the `Layout`,
/// - does not alias any existing live memory,
/// - and is freed safely using `deallocate`.
///
/// Additionally, `grow`, `grow_zeroed`, and `shrink` must behave correctly
/// even if called in rapid succession.
pub unsafe trait SelflessAllocator {
    /// Allocates a block of memory described by the given layout.
    ///
    /// # Errors
    ///
    /// Returns `Err(AllocError)` if:
    /// - the system is out of memory,
    /// - the alignment is invalid or unsupported,
    /// - or the size exceeds allocator or platform limits.
    fn allocate(layout: Layout) -> Result<NonNull<[u8]>, AllocError>;

    /// Allocates zero-initialized memory.
    ///
    /// This is equivalent to `allocate` followed by `write_bytes` with `0`.
    ///
    /// # Errors
    ///
    /// Returns `Err(AllocError)` under the same conditions as `allocate`,
    /// or if zeroing the memory would cause undefined behavior.
    fn allocate_zeroed(layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = Self::allocate(layout)?;
        unsafe { ptr.cast::<u8>().as_ptr().write_bytes(0, ptr.len()) }
        Ok(ptr)
    }

    /// Deallocates a previously allocated block of memory.
    ///
    /// # Safety
    ///
    /// - `ptr` must have been returned by a previous call to `allocate` or `allocate_zeroed`.
    /// - `layout` must match the original layout used in the allocation.
    /// - Double-free or invalid layout can lead to undefined behavior.
    unsafe fn deallocate(ptr: NonNull<u8>, layout: Layout);

    /// Grows a memory block to a new layout.
    ///
    /// The contents up to the lesser of the old and new sizes are preserved.
    /// The old memory is deallocated.
    ///
    /// # Errors
    ///
    /// Returns `Err(AllocError)` if:
    /// - a new allocation fails,
    /// - or copying or deallocation fails.
    ///
    /// # Safety
    ///
    /// - `ptr` must have been allocated by this allocator.
    /// - `old_layout` must accurately reflect the original allocation.
    /// - `new_layout.size()` must be greater than or equal to `old_layout.size()`.
    unsafe fn grow(
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(
            new_layout.size() >= old_layout.size(),
            "`new_layout.size()` must be >= `old_layout.size()`"
        );

        let new_ptr = Self::allocate(new_layout)?;

        unsafe {
            ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.cast().as_ptr(), old_layout.size());
            Self::deallocate(ptr, old_layout);
        }

        Ok(new_ptr)
    }

    /// Grows and zero-initializes additional memory beyond the old size.
    ///
    /// The contents of the original allocation are preserved. The added
    /// space between `old_layout.size()` and `new_layout.size()` is set to zero.
    ///
    /// # Errors
    ///
    /// Returns `Err(AllocError)` if:
    /// - memory cannot be allocated,
    /// - or copying/zeroing fails.
    ///
    /// # Safety
    ///
    /// Same as `grow`.
    unsafe fn grow_zeroed(
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(
            new_layout.size() >= old_layout.size(),
            "`new_layout.size()` must be >= `old_layout.size()`"
        );

        let new_ptr = Self::allocate_zeroed(new_layout)?;

        unsafe {
            ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.cast().as_ptr(), old_layout.size());
            Self::deallocate(ptr, old_layout);
        }

        Ok(new_ptr)
    }

    /// Attempts to shrink a memory block to a smaller size.
    ///
    /// The contents up to the new layout size are preserved.
    ///
    /// # Errors
    ///
    /// Returns `Err(AllocError)` if:
    /// - memory cannot be reallocated,
    /// - or copying fails.
    ///
    /// # Safety
    ///
    /// - `ptr` must have been allocated by this allocator.
    /// - `old_layout` must match the original allocation.
    /// - `new_layout.size()` must be less than or equal to `old_layout.size()`.
    unsafe fn shrink(
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(
            new_layout.size() <= old_layout.size(),
            "`new_layout.size()` must be <= `old_layout.size()`"
        );

        let new_ptr = Self::allocate(new_layout)?;

        unsafe {
            ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.cast().as_ptr(), new_layout.size());
            Self::deallocate(ptr, old_layout);
        }

        Ok(new_ptr)
    }
}
