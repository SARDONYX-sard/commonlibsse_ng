// Assuming BSBitFieldHeapAllocator and BSBitField are part of the RE namespace

use core::{marker::PhantomData, ptr::NonNull};

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BSBitFieldHeapAllocator {
    data: Option<NonNull<u32>>,
}

impl BSBitFieldHeapAllocator {
    #[inline]
    pub const fn new() -> Self {
        Self { data: None }
    }
}

// Buffer union equivalent in Rust
#[repr(C)]
pub union Buffer {
    local: u32,
    heap: *mut u32,
}

impl Default for Buffer {
    #[inline]
    fn default() -> Self {
        Self { local: 0 }
    }
}

impl core::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unsafe {
            f.debug_struct("Buffer").field("local", &self.local).field("heap", &self.heap).finish()
        }
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct BSBitField<A = BSBitFieldHeapAllocator> {
    buffer: Buffer,
    size: u32,
    _phantom: PhantomData<A>,
}

impl<Allocator: Default> BSBitField<Allocator> {
    #[inline]
    pub const fn new() -> Self {
        Self { buffer: Buffer { local: 0 }, size: 0, _phantom: PhantomData }
    }
}
