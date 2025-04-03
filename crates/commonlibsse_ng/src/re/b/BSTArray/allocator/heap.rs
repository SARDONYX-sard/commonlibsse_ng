use core::ptr::{self, NonNull};
use std::alloc::{self, Layout};

use super::error::AllocatorError;

#[repr(C)]
#[derive(Debug)]
pub struct BSTArrayHeapAllocator {
    data: Option<NonNull<u8>>,
    capacity: u32,
}
const_assert_eq!(core::mem::size_of::<BSTArrayHeapAllocator>(), 0x10);

impl BSTArrayHeapAllocator {
    #[inline]
    pub const fn new() -> Self {
        Self { data: None, capacity: 0 }
    }

    #[inline]
    pub const unsafe fn data(&self) -> Option<NonNull<u8>> {
        self.data
    }

    #[inline]
    pub const fn capacity(&self) -> u32 {
        self.capacity
    }

    /// # Errors
    /// - If size > `isize::MAX`
    /// - If allocation failed
    pub fn allocate(&mut self, size: usize) -> Result<(), AllocatorError> {
        if size == 0 {
            return Ok(());
        }
        let layout = Layout::array::<u8>(size)
            .map_err(|_| AllocatorError::InvalidLayout { requested_size: size })?;
        let mem = unsafe { alloc::alloc_zeroed(layout) };

        self.data =
            Some(NonNull::new(mem).ok_or_else(|| AllocatorError::AllocationFailed { layout })?);
        self.capacity = size as u32;
        Ok(())
    }

    /// # Errors
    /// - If `self.capacity` > `isize::MAX`
    pub fn deallocate(&mut self) -> Result<(), AllocatorError> {
        let capacity = self.capacity as usize;
        let layout = Layout::array::<u8>(capacity)
            .map_err(|_| AllocatorError::InvalidLayout { requested_size: capacity })?;

        if let Some(data) = self.data.take() {
            unsafe { alloc::dealloc(data.as_ptr(), layout) };
            self.capacity = 0;
        }
        Ok(())
    }
}

impl Drop for BSTArrayHeapAllocator {
    fn drop(&mut self) {
        let _ = self.deallocate();
    }
}

impl Clone for BSTArrayHeapAllocator {
    fn clone(&self) -> Self {
        let mut new_alloc = Self::new();

        if let Some(data) = self.data {
            let capacity = self.capacity as usize;
            if capacity > 0 && matches!(new_alloc.allocate(self.capacity as usize), Ok(())) {
                let new_ptr = new_alloc.data.expect("valid ptr").as_ptr();
                unsafe {
                    ptr::copy_nonoverlapping(data.as_ptr(), new_ptr, capacity);
                }
            }
        }

        new_alloc
    }
}

impl Default for BSTArrayHeapAllocator {
    fn default() -> Self {
        Self::new()
    }
}
