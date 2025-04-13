use core::marker::PhantomData;
use core::mem;
use core::ops::Mul;
use core::ptr;
use std::alloc::{Layout, alloc, dealloc};

use generic_array::{ArrayLength, GenericArray};
use typenum::{U2, op};

/// A trait representing a generic memory allocator.
/// Provides methods for allocating and deallocating raw bytes.
pub trait Allocator {
    /// Allocates a block of memory of the specified size in bytes.
    ///
    /// # Notes
    /// If the allocation fails, null is returned.
    ///
    /// # Safety
    /// See [`GlobalAlloc::alloc`].
    unsafe fn allocate_bytes(&mut self, bytes_size: usize) -> *mut u8;

    /// Deallocates a previously allocated block of memory.
    ///
    /// # Safety
    /// This function is unsafe if called with an invalid or null pointer.
    unsafe fn deallocate_bytes(&mut self, ptr: *mut u8);

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
    unsafe fn allocate_bytes(&mut self, bytes_size: usize) -> *mut u8 {
        debug_assert!(bytes_size % mem::size_of::<usize>() == 0, "Bytes must be aligned");

        let layout =
            Layout::from_size_align(bytes_size, mem::align_of::<usize>()).expect("Invalid layout");

        unsafe { alloc(layout) }
    }

    unsafe fn deallocate_bytes(&mut self, ptr: *mut u8) {
        if !ptr.is_null() {
            let layout = Layout::from_size_align(mem::size_of::<usize>(), mem::align_of::<usize>())
                .expect("Invalid layout");

            unsafe { dealloc(ptr, layout) }
        }
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
    U2: Mul<N>,
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
    U2: Mul<N>,
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
    U2: Mul<N>,
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
    U2: Mul<N>,
    op!(U2 * N): ArrayLength,
{
    /// Returns the minimum size
    #[inline]
    fn min_size() -> u32 {
        N::U32
    }

    /// Allocates memory
    #[inline]
    unsafe fn allocate_bytes(&mut self, bytes: usize) -> *mut u8 {
        assert!(bytes % N::USIZE == 0, "Bytes must be aligned to S");

        if bytes <= self.buffer.len() { self.buffer.as_mut_ptr() } else { ptr::null_mut() }
    }

    /// Deallocates memory
    #[inline]
    unsafe fn deallocate_bytes(&mut self, ptr: *mut u8) {
        assert!(ptr == self.buffer.as_mut_ptr(), "Invalid pointer");
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
