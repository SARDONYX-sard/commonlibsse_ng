//! # BSTSmartPointer
//!
//! A smart pointer with custom reference counting and auto-ptr management strategies.

use stdx::unique::Unique;

use crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCountedTrait;
use crate::re::TESBox::TESBox;
use core::fmt::{self, Debug};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::ptr::{self, NonNull};

/// Trait for managing smart pointer lifetimes.
pub trait ManageBSTSmartPointer {
    /// No-op for acquire.
    #[inline]
    fn acquire<T>(_ptr: *mut T)
    where
        T: BSIntrusiveRefCountedTrait,
    {
    }

    /// Releases the object held by the smart pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be valid and allocated by `TESBox`.
    #[inline]
    unsafe fn release<T>(ptr: *mut T)
    where
        T: BSIntrusiveRefCountedTrait,
    {
        if !ptr.is_null() {
            drop(unsafe { TESBox::from_raw(ptr) });
        }
    }
}

/// Intrusive reference counting manager.
#[derive(Debug)]
pub struct BSTSmartPointerIntrusiveRefCount;

impl ManageBSTSmartPointer for BSTSmartPointerIntrusiveRefCount {
    #[inline]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn acquire<T>(ptr: *mut T)
    where
        T: BSIntrusiveRefCountedTrait,
    {
        if !ptr.is_null() {
            (unsafe { &*ptr }).inc_ref();
        }
    }

    #[inline]
    unsafe fn release<T>(ptr: *mut T)
    where
        T: BSIntrusiveRefCountedTrait,
    {
        if !ptr.is_null() && (unsafe { &*ptr }).dec_ref() == 0 {
            drop(unsafe { TESBox::from_raw(ptr) });
        }
    }
}

/// Auto-pointer manager without reference counting.
#[derive(Debug)]
pub struct BSTSmartPointerAutoPtr;
impl ManageBSTSmartPointer for BSTSmartPointerAutoPtr {}

pub type BSTAutoPointer<T> = BSTSmartPointer<T, BSTSmartPointerAutoPtr>;

/// Smart pointer with customizable reference management.
///
/// This smart pointer optionally manages reference counts depending on the `M` parameter.
///
/// # Panics
///
/// Dereferencing a null pointer via `Deref` or `DerefMut` will **panic**.
/// Use [`as_ref`] or [`is_null`] to avoid this panic.
#[repr(C)]
pub struct BSTSmartPointer<T, M = BSTSmartPointerIntrusiveRefCount>
where
    T: BSIntrusiveRefCountedTrait,
    M: ManageBSTSmartPointer,
{
    /// C++ equivalent: `T*`
    ptr: Option<Unique<T>>,
    _marker: PhantomData<M>,
}

impl<T, M> BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: ManageBSTSmartPointer,
{
    /// Creates a new smart pointer from a raw pointer.
    /// # Safety
    /// The pointer must be created from `TESBox`(`MemoryManager::allocate`).
    #[inline]
    pub unsafe fn new(ptr: *mut T) -> Self {
        M::acquire(ptr);
        Self { ptr: Unique::new(ptr), _marker: PhantomData }
    }

    /// Creates a new smart pointer from a raw pointer.
    #[inline]
    pub fn from_non_null(ptr: NonNull<T>) -> Self {
        M::acquire(ptr.as_ptr());
        Self { ptr: Some(Unique::from(ptr)), _marker: PhantomData }
    }

    /// Replaces the internal pointer with null and releases the old object.
    ///
    /// After calling this, [`is_null`] returns `true`.
    #[inline]
    pub fn reset(&mut self) {
        unsafe {
            M::release(self.as_ptr());
            self.ptr = None; // <- ptr::null_mut()
        }
    }

    /// Creates a smart pointer from a [`TESBox`], taking ownership.
    #[inline]
    pub fn from_box(value: TESBox<T>) -> Self {
        let ptr = TESBox::into_raw(value);
        // Safety: The pointer is valid and managed by `TESBox`.
        unsafe { Self::new(ptr) }
    }

    /// Returns an immutable reference to the managed object, or `None` if the pointer is null.
    ///
    /// Use this to safely access the internal object without risking a panic.
    #[inline]
    pub const fn as_ref(&self) -> Option<&T> {
        match &self.ptr {
            Some(p) => unsafe { Some(p.as_ref()) },
            None => None,
        }
    }

    /// Returns a mutable reference to the managed object, or `None` if the pointer is null.
    #[inline]
    pub const fn as_mut(&mut self) -> Option<&mut T> {
        match &mut self.ptr {
            Some(p) => unsafe { Some(p.as_mut()) },
            None => None,
        }
    }

    /// Returns the raw pointer to the managed object.
    #[inline]
    pub const fn as_ptr(&self) -> *mut T {
        match self.ptr {
            Some(p) => p.as_ptr(),
            None => ptr::null_mut(),
        }
    }

    /// Returns true if the internal pointer is null.
    #[inline]
    pub const fn is_null(&self) -> bool {
        self.ptr.is_none()
    }
}

impl<T, M> fmt::Debug for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait + fmt::Debug,
    M: ManageBSTSmartPointer,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("BSTSmartPointer");
        match self.as_ref() {
            Some(value) => s.field("ptr", value),
            None => s.field("ptr", &"null"),
        };
        s.finish()
    }
}

impl<T, M> Default for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: ManageBSTSmartPointer,
{
    #[inline]
    fn default() -> Self {
        Self { ptr: None, _marker: PhantomData }
    }
}

impl<T, M> Drop for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: ManageBSTSmartPointer,
{
    #[inline]
    fn drop(&mut self) {
        self.reset();
    }
}

impl<T, M> Deref for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: ManageBSTSmartPointer,
{
    type Target = T;

    /// Dereferences the smart pointer to the managed object.
    ///
    /// # Panics
    ///
    /// Panics if the internal pointer is null.
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref().expect("Dereferencing null pointer")
    }
}

impl<T, M> DerefMut for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: ManageBSTSmartPointer,
{
    /// Dereferences the smart pointer mutably.
    ///
    /// # Panics
    ///
    /// Panics if the internal pointer is null.
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut().expect("Dereferencing null pointer")
    }
}

impl<T, M> Clone for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: ManageBSTSmartPointer,
{
    #[inline]
    fn clone(&self) -> Self {
        M::acquire(self.as_ptr());
        Self { ptr: self.ptr, _marker: PhantomData }
    }
}

impl<T, M> PartialEq for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: ManageBSTSmartPointer,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self.ptr, other.ptr) {
            (Some(p1), Some(p2)) => p1.as_non_null_ptr() == p2.as_non_null_ptr(),
            (None, None) => true,
            _ => false,
        }
    }
}

impl<T, M> Eq for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: ManageBSTSmartPointer,
{
}

impl<T, M> PartialEq<*mut T> for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: ManageBSTSmartPointer,
{
    #[inline]
    fn eq(&self, other: &*mut T) -> bool {
        self.as_ptr() == *other
    }
}

impl<T, M> PartialEq<Option<*mut T>> for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: ManageBSTSmartPointer,
{
    #[inline]
    fn eq(&self, other: &Option<*mut T>) -> bool {
        Some(self.as_ptr()) == *other
    }
}

impl<T, M> PartialEq<ptr::NonNull<T>> for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: ManageBSTSmartPointer,
{
    #[inline]
    fn eq(&self, other: &ptr::NonNull<T>) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::sync::atomic::{AtomicU32, Ordering};

    #[repr(C)]
    #[derive(Debug)]
    struct TestObject {
        ref_count: AtomicU32,
        value: i32,
    }

    impl TestObject {
        const fn new(value: i32) -> Self {
            Self { ref_count: AtomicU32::new(0), value }
        }
    }

    impl BSIntrusiveRefCountedTrait for TestObject {
        fn inc_ref(&self) -> u32 {
            self.ref_count.fetch_add(1, Ordering::AcqRel) + 1
        }

        fn dec_ref(&self) -> u32 {
            self.ref_count.fetch_sub(1, Ordering::AcqRel) - 1
        }
    }
    // BSTSmartPointerTrait,

    #[test]
    fn test_smart_pointer() {
        {
            let obj = TESBox::new(TestObject::new(42));
            let mut ptr = BSTSmartPointer::<TestObject>::from_box(obj);
            assert_eq!(ptr.value, 42);
            assert!(ptr.as_ref().is_some());
            assert_eq!(ptr.as_ref().map(|p| p.ref_count.load(Ordering::Acquire)), Some(1));

            // Clone and check ref count
            let mut ptr2 = ptr.clone();
            assert_eq!(ptr2.value, 42);
            assert_eq!(ptr.as_ref().map(|p| p.ref_count.load(Ordering::Acquire)), Some(2));

            ptr.reset();
            assert_eq!(ptr2.as_ref().map(|p| p.ref_count.load(Ordering::Acquire)), Some(1));

            assert!(ptr.as_ref().is_none());
            assert!(ptr2.as_ref().is_some());
            ptr2.reset();
        }
    }

    #[test]
    fn test_auto_pointer() {
        let obj = TESBox::new(TestObject::new(123));
        let mut auto_ptr = BSTAutoPointer::from_box(obj);
        assert_eq!(auto_ptr.value, 123);

        auto_ptr.reset();
        assert!(auto_ptr.as_ref().is_none());
    }
}
