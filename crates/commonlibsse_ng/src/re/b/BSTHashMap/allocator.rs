use core::alloc::Layout;
use core::{marker::PhantomData, ptr::NonNull};
use core::{mem, ptr};

use crate::re::MemoryManager::alloc::{alloc_zeroed, dealloc};

use generic_array::{ArrayLength, GenericArray};
use stdx::alloc::{AllocError, non_null_empty_slice};
use typenum::{U2, op};

/// A trait representing a generic memory allocator.
/// Provides methods for allocating and deallocating raw bytes.
pub trait Allocator {
    /// Allocates a block of memory of the specified size in bytes and initializes it to zero.
    ///
    /// # Safety
    /// See [`std::alloc::GlobalAlloc::alloc`].
    ///
    /// # Errors
    /// Returns `AllocError` if the allocation fails.
    unsafe fn allocate_zeroed(&mut self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>;

    /// Deallocates a previously allocated block of memory.
    ///
    /// # Safety
    /// This function is unsafe if called with an invalid or null pointer.
    unsafe fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout);

    /// Returns the minimum size that can be allocated by this allocator.
    ///
    /// Must be `> 0`.
    #[inline]
    fn min_size() -> u32 {
        1 << 3
    }

    /// Gets the current entries pointer.
    fn get_entries(&self) -> *mut u8;

    /// Sets the entries pointer.
    fn set_entries(&mut self, entries: *mut u8);
}

/// An allocator implementation for the BSTScatterTable.
///
/// This allocator uses the Rust standard library's `alloc` and `dealloc` functions
/// to manage dynamic memory allocation, similar to C++'s `malloc` and `free`.
#[repr(C)]
#[derive(Debug)]
pub struct BSTScatterTableHeapAllocator {
    /// 64-bit padding to match the original memory layout.
    pad00: u64,

    /// Pointer to the allocated memory block.
    entries: *mut u8,
}

impl Default for BSTScatterTableHeapAllocator {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl BSTScatterTableHeapAllocator {
    /// Creates a new `BSTScatterTableHeapAllocator` instance with null entries.
    #[inline]
    pub const fn new() -> Self {
        Self { pad00: 0, entries: ptr::null_mut() }
    }
}

impl Allocator for BSTScatterTableHeapAllocator {
    /// Allocates a block of memory of the specified size in bytes.
    ///
    /// # Panics
    /// Panics under the following conditions.
    /// - If `bytes_size` is not a multiple of usize.
    unsafe fn allocate_zeroed(&mut self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(layout.size() % mem::size_of::<usize>() == 0, "Bytes must be aligned");

        let ptr = unsafe { alloc_zeroed(layout) };
        if ptr.is_null() {
            return Err(AllocError);
        }

        self.entries = ptr;
        Ok(match layout.size() {
            0 => non_null_empty_slice(layout),
            size => NonNull::slice_from_raw_parts(unsafe { NonNull::new_unchecked(ptr) }, size),
        })
    }

    unsafe fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout) {
        unsafe { dealloc(ptr.as_ptr(), layout) }
    }

    #[inline]
    fn get_entries(&self) -> *mut u8 {
        self.entries
    }

    #[inline]
    fn set_entries(&mut self, entries: *mut u8) {
        self.entries = entries;
    }
}

/// `BSTStaticHashMapBase::Allocator` equivalent in Rust using `GenericArray`
#[repr(C)]
#[derive(Debug)]
pub struct BSTStaticHashMapBaseAllocator<N, A>
where
    N: ArrayLength,
    U2: core::ops::Mul<N>,
    A: ArrayLength,
    op!(U2 * N): ArrayLength,
{
    buffer: GenericArray<u8, N>, // Stack buffer
    entries: *mut u8,            // Pointer to entries
    _align: PhantomData<A>,      // Alignment marker
}

impl<N, A> Default for BSTStaticHashMapBaseAllocator<N, A>
where
    N: ArrayLength,
    U2: core::ops::Mul<N>,
    A: ArrayLength,
    op!(U2 * N): ArrayLength,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<N, A> BSTStaticHashMapBaseAllocator<N, A>
where
    N: ArrayLength,
    U2: core::ops::Mul<N>,
    A: ArrayLength,
    op!(U2 * N): ArrayLength,
{
    /// Creates a new allocator instance
    pub fn new() -> Self {
        const {
            assert!(N::USIZE > 0 && N::USIZE.is_power_of_two(), "N must be a power of two");
        };

        Self { buffer: GenericArray::default(), entries: ptr::null_mut(), _align: PhantomData }
    }
}

impl<N, A> Allocator for BSTStaticHashMapBaseAllocator<N, A>
where
    N: ArrayLength,
    A: ArrayLength,
    U2: core::ops::Mul<N>,
    op!(U2 * N): ArrayLength,
{
    /// Returns the minimum size
    #[inline]
    fn min_size() -> u32 {
        N::U32
    }

    /// Allocates memory
    ///
    /// # Panics
    /// Panics if the size is not a multiple of `N::USIZE`.
    #[inline]
    unsafe fn allocate_zeroed(&mut self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let size = layout.size();
        assert!(size % N::USIZE == 0, "Bytes must be aligned to S");

        let len = self.buffer.len();
        let Some(ptr) = NonNull::new(self.buffer.as_mut_ptr()) else {
            return Err(AllocError);
        };
        if size <= len {
            return Ok(match size {
                0 => non_null_empty_slice(layout),
                size => NonNull::slice_from_raw_parts(ptr, size),
            });
        }

        Err(AllocError)
    }

    /// No-op deallocation
    #[inline]
    unsafe fn deallocate(&mut self, ptr: NonNull<u8>, _: Layout) {
        assert!(ptr.as_ptr() == self.buffer.as_mut_ptr(), "Invalid pointer");
    }

    #[inline]
    fn get_entries(&self) -> *mut u8 {
        self.entries
    }

    #[inline]
    fn set_entries(&mut self, entries: *mut u8) {
        assert!(entries == self.buffer.as_mut_ptr() || entries.is_null());
        self.entries = entries;
    }
}
