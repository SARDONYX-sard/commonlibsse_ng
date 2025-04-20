use core::sync::atomic::{AtomicU32, Ordering};

#[repr(C)]
#[derive(Debug, Default)]
pub struct BSIntrusiveRefCounted {
    pub refCount: AtomicU32,
}
const _: () = assert!(core::mem::size_of::<BSIntrusiveRefCounted>() == 0x4);

impl BSIntrusiveRefCounted {
    #[inline]
    pub const fn new() -> Self {
        Self { refCount: AtomicU32::new(0) }
    }
}

pub trait BSIntrusiveRefCountedTrait {
    /// Returns the value after +1 to the current value.
    ///
    /// # Note
    /// Implementations must ensure that the returned value is the *new* reference count
    /// **after incrementing**, not the old one.
    ///
    /// For example, the following implementation is incorrect:
    /// ```rust
    /// self.refCount.fetch_add(1, Ordering::AcqRel) // Wrong implementation! Returns the prev value!
    /// ```
    /// `fetch_add` returns the previous value, so this would return the *old* count,
    /// not the incremented one.
    ///
    /// Correct implementation should add 1 to the result:
    /// ```rust
    /// self.refCount.fetch_add(1, Ordering::AcqRel) + 1
    /// ```
    fn inc_ref(&self) -> u32;

    /// Decrements the reference count and returns the value *after* decrementing.
    ///
    /// # Note
    /// Implementations must ensure that the returned value is the *new* reference count
    /// **after decrementing**, not the old one.
    ///
    /// For example, the following implementation is incorrect:
    /// ```rust
    /// self.refCount.fetch_sub(1, Ordering::AcqRel) // Wrong implementation! Returns the prev value!
    /// ```
    /// `fetch_sub` returns the previous value, so this would return the *old* count,
    /// not the decremented one.
    ///
    /// Correct implementation should subtract 1 from the result:
    /// ```rust
    /// self.refCount.fetch_sub(1, Ordering::AcqRel) - 1
    /// ```
    fn dec_ref(&self) -> u32;
}

impl BSIntrusiveRefCountedTrait for BSIntrusiveRefCounted {
    #[inline]
    fn inc_ref(&self) -> u32 {
        // Reproduction of post-increment of C++ atomic_ref
        self.refCount.fetch_add(1, Ordering::AcqRel) + 1
    }

    #[inline]
    fn dec_ref(&self) -> u32 {
        // Reproduction of post-decrement of C++ atomic_ref
        self.refCount.fetch_sub(1, Ordering::AcqRel) - 1
    }
}
