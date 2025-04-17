mod scrap;

pub use self::scrap::BSScrapArrayAllocator;

use core::alloc::Layout;
use core::ffi::c_void;

/// A trait representing a custom memory allocator.
///
/// # Safety
///
/// Implementors must ensure all allocation and deallocation operations are safe
/// and consistent with Rust’s memory safety guarantees.
pub unsafe trait Allocator {
    /// Creates a new instance of the allocator.
    fn new() -> Self;

    /// Returns a memory layout for a given size using pointer alignment.
    ///
    /// Panics if the layout is invalid, which should never happen for valid inputs.
    #[inline]
    fn ptr_layout(size: usize) -> Layout {
        const PTR_ALIGN_SIZE: usize = align_of::<*mut c_void>();
        Layout::from_size_align(size, PTR_ALIGN_SIZE).expect("Valid layout")
    }

    /// Returns an immutable raw pointer to the allocator's memory region.
    fn as_ptr(&self) -> *const c_void;

    /// Returns a mutable raw pointer to the allocator's memory region.
    fn as_mut_ptr(&mut self) -> *mut c_void;

    /// Returns the total capacity of the allocator in bytes.
    fn capacity(&self) -> u32;

    /// Allocates memory using the given layout and returns a zero-initialized pointer.
    ///
    /// Returns a null pointer on allocation failure.
    ///
    /// # Safety
    ///
    /// The caller must ensure the layout is valid. The returned pointer must be used in
    /// accordance with Rust's aliasing and alignment rules.
    unsafe fn allocate(&mut self, layout: Layout) -> *mut c_void;

    /// Deallocates the memory at the given pointer.
    ///
    /// # Safety
    ///
    /// The pointer must have been previously allocated by this allocator and must
    /// match the layout used during allocation.
    unsafe fn deallocate(&mut self, ptr: *mut c_void);

    /// Sets the internal state of the allocator.
    ///
    /// - `data`: Pointer to the start of the memory region.
    /// - `capacity`: Total number of bytes the allocator can manage.
    /// - `type_size`: Size of the type to be allocated in this allocator.
    fn set_allocator_traits(&mut self, data: *mut c_void, capacity: u32, type_size: usize);
}
