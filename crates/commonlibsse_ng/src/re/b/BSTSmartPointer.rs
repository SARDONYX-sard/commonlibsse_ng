//! # BSTSmartPointer
//!
//! A smart pointer with custom reference counting and auto-ptr management strategies.
//!
//! # Memory Layout:
//! - `_ptr`: Raw pointer to the managed object (0x0)

use crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCountedTrait;
use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::ptr;

pub trait BSTSmartPointerTrait {
    /// No-op for acquire.
    fn acquire<T>(_ptr: *mut T)
    where
        T: BSIntrusiveRefCountedTrait,
    {
    }

    /// Deallocates the managed object.
    ///
    /// # Safety
    unsafe fn release<T>(ptr: *mut T)
    where
        T: BSIntrusiveRefCountedTrait,
    {
        if !ptr.is_null() {
            drop(unsafe { Box::from_raw(ptr) });
        }
    }
}

/// Intrusive reference counting manager.
#[derive(Debug)]
pub struct BSTSmartPointerIntrusiveRefCount;

impl BSTSmartPointerTrait for BSTSmartPointerIntrusiveRefCount {
    /// Increases the reference count of the managed object.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn acquire<T>(ptr: *mut T)
    where
        T: BSIntrusiveRefCountedTrait,
    {
        if !ptr.is_null() {
            (unsafe { &*ptr }).inc_ref();
        }
    }

    /// Decreases the reference count and deallocates if necessary.
    unsafe fn release<T>(ptr: *mut T)
    where
        T: BSIntrusiveRefCountedTrait,
    {
        if !ptr.is_null() && (unsafe { &*ptr }).dec_ref() == 0 {
            drop(unsafe { Box::from_raw(ptr) });
        }
    }
}

/// Auto-pointer manager without reference counting.
#[derive(Debug)]
pub struct BSTSmartPointerAutoPtr;

impl BSTSmartPointerTrait for BSTSmartPointerAutoPtr {}

/// Smart pointer with customizable reference management.
#[repr(C)]
#[derive(Debug)]
pub struct BSTSmartPointer<T, M = BSTSmartPointerIntrusiveRefCount>
where
    T: BSIntrusiveRefCountedTrait,
    M: BSTSmartPointerTrait,
{
    ptr: *mut T,
    _marker: PhantomData<M>,
}

impl<T, M> BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: BSTSmartPointerTrait,
{
    /// Creates a new `BSTSmartPointer` from a raw pointer.
    pub fn new(ptr: *mut T) -> Self {
        M::acquire(ptr);
        Self { ptr, _marker: PhantomData }
    }

    /// Resets the smart pointer, releasing the current object.
    pub fn reset(&mut self) {
        unsafe {
            M::release(self.ptr);
            self.ptr = ptr::null_mut();
        }
    }

    /// Creates a new `BSTSmartPointer` by moving ownership.
    pub fn from_box(value: Box<T>) -> Self {
        let ptr = Box::into_raw(value);
        Self::new(ptr)
    }

    /// Returns a reference to the managed object or `None` if null.
    pub const fn as_ref(&self) -> Option<&T> {
        unsafe { self.ptr.as_ref() }
    }

    /// Returns a mutable reference to the managed object or `None` if null.
    pub const fn as_mut(&mut self) -> Option<&mut T> {
        unsafe { self.ptr.as_mut() }
    }

    /// Gets the raw pointer.
    pub const fn get(&self) -> *mut T {
        self.ptr
    }
}

impl<T, M> Default for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: BSTSmartPointerTrait,
{
    fn default() -> Self {
        Self { ptr: ptr::null_mut(), _marker: PhantomData }
    }
}

impl<T, M> Drop for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: BSTSmartPointerTrait,
{
    fn drop(&mut self) {
        self.reset();
    }
}

impl<T, M> Deref for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: BSTSmartPointerTrait,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref().expect("Dereferencing null pointer")
    }
}

impl<T, M> DerefMut for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: BSTSmartPointerTrait,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut().expect("Dereferencing null pointer")
    }
}

impl<T, M> Clone for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: BSTSmartPointerTrait,
{
    fn clone(&self) -> Self {
        M::acquire(self.ptr);
        Self { ptr: self.ptr, _marker: PhantomData }
    }
}

impl<T, M> PartialEq for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: BSTSmartPointerTrait,
{
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<T, M> Eq for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: BSTSmartPointerTrait,
{
}

pub type BSTAutoPointer<T> = BSTSmartPointer<T, BSTSmartPointerAutoPtr>;
impl<T> BSTSmartPointer<T, BSTSmartPointerAutoPtr>
where
    T: BSIntrusiveRefCountedTrait,
{
    /// Creates an auto-pointer smart pointer.
    pub fn auto_ptr(value: Box<T>) -> Self {
        let ptr = Box::into_raw(value);
        Self::new(ptr)
    }
}

/// Helper function to create a `BSTSmartPointer`.
pub fn make_smart<T, M>(value: T) -> BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: BSTSmartPointerTrait,
{
    BSTSmartPointer::from_box(Box::new(value))
}

/// Equality operators for comparing smart pointers.
impl<T, M> PartialEq<*mut T> for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: BSTSmartPointerTrait,
{
    fn eq(&self, other: &*mut T) -> bool {
        self.ptr == *other
    }
}

impl<T, M> PartialEq<Option<*mut T>> for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: BSTSmartPointerTrait,
{
    fn eq(&self, other: &Option<*mut T>) -> bool {
        Some(self.ptr) == *other
    }
}

impl<T, M> PartialEq<std::ptr::NonNull<T>> for BSTSmartPointer<T, M>
where
    T: BSIntrusiveRefCountedTrait,
    M: BSTSmartPointerTrait,
{
    fn eq(&self, other: &std::ptr::NonNull<T>) -> bool {
        self.ptr == other.as_ptr()
    }
}

// /// Macro to define a smart pointer alias with intrusive ref-counting.
// #[macro_export]
// macro_rules! BSSmartPointer {
//     ($name:ident) => {
//         pub type $name##Ptr = BSTSmartPointer<$name, BSTSmartPointerIntrusiveRefCount>;
//     };
// }

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
            Self { ref_count: AtomicU32::new(1), value }
        }
    }

    impl BSIntrusiveRefCountedTrait for TestObject {
        fn inc_ref(&self) -> u32 {
            self.ref_count.fetch_add(1, Ordering::AcqRel)
        }

        fn dec_ref(&self) -> u32 {
            self.ref_count.fetch_sub(1, Ordering::AcqRel)
        }
    }
    // BSTSmartPointerTrait,

    #[test]
    fn test_smart_pointer() {
        let obj = Box::new(TestObject::new(42));
        let mut ptr = BSTSmartPointer::<TestObject>::from_box(obj);
        assert_eq!(ptr.value, 42);
        assert!(ptr.as_ref().is_some());

        // Clone and check ref count
        let ptr2 = ptr.clone();
        assert_eq!(ptr2.value, 42);

        ptr.reset();
        assert!(ptr.as_ref().is_none());
        assert!(ptr2.as_ref().is_some());
    }

    #[test]
    fn test_auto_pointer() {
        let obj = Box::new(TestObject::new(123));
        let mut auto_ptr = BSTSmartPointer::<TestObject, BSTSmartPointerAutoPtr>::from_box(obj);
        assert_eq!(auto_ptr.value, 123);

        auto_ptr.reset();
        assert!(auto_ptr.as_ref().is_none());
    }
}
