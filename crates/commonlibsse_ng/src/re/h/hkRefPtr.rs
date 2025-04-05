//! # hkRefPtr
//!
//! This module defines the `hkRefPtr` smart pointer, which is a reference-counted
//! pointer used in the Havok engine. It mimics C++ `hkRefPtr` behavior with
//! reference counting and smart pointer functionality.

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr;

use crate::re::hkaMirroredSkeleton;

/// A smart pointer with reference counting, modeled after Havok's `hkRefPtr`.
#[repr(C)]
pub struct hkRefPtr<T>
where
    T: hkRefPtrCounted,
{
    /// The raw pointer to the referenced object.
    /// - Offset: `0x0`
    _ptr: *mut T,
    /// Phantom type marker for the template type.
    _marker: PhantomData<T>,
}

/// Ensure the memory layout matches the C++ version.
const _: () = {
    assert!(core::mem::size_of::<hkRefPtr<hkaMirroredSkeleton>>() == 0x8);
};

pub trait hkRefPtrCounted {
    fn AddReference(&self) {}
    fn RemoveReference(&self) {}
}

impl<T> hkRefPtr<T>
where
    T: hkRefPtrCounted,
{
    /// Creates a new `hkRefPtr` with a `nullptr`.
    #[inline]
    pub const fn new() -> Self {
        Self { _ptr: ptr::null_mut(), _marker: PhantomData }
    }

    /// Creates an `hkRefPtr` from a raw pointer.
    #[inline]
    pub fn from_raw(ptr: *mut T) -> Self {
        let mut ref_ptr = Self { _ptr: ptr, _marker: PhantomData };
        ref_ptr.try_attach();
        ref_ptr
    }

    /// Resets the pointer, releasing the reference.
    #[inline]
    pub fn reset(&mut self) {
        self.try_detach();
    }

    /// Replaces the current pointer with a new one.
    #[inline]
    pub fn reset_with(&mut self, ptr: *mut T) {
        if self._ptr != ptr {
            self.try_detach();
            self._ptr = ptr;
            self.try_attach();
        }
    }

    /// Returns the raw pointer.
    #[inline]
    pub const fn get(&self) -> *mut T {
        self._ptr
    }

    /// Checks if the pointer is not `nullptr`.
    #[inline]
    pub const fn is_some(&self) -> bool {
        !self._ptr.is_null()
    }

    /// Tries to attach to the reference, increasing its count.
    #[inline]
    fn try_attach(&mut self) {
        if !self._ptr.is_null() {
            unsafe {
                (*self._ptr).AddReference();
            }
        }
    }

    /// Tries to detach from the reference, decreasing its count.
    #[inline]
    fn try_detach(&mut self) {
        if !self._ptr.is_null() {
            unsafe {
                (*self._ptr).RemoveReference();
            }
            self._ptr = ptr::null_mut();
        }
    }
}

impl<T> Default for hkRefPtr<T>
where
    T: hkRefPtrCounted,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for hkRefPtr<T>
where
    T: hkRefPtrCounted,
{
    #[inline]
    fn clone(&self) -> Self {
        let mut ref_ptr = Self { _ptr: self._ptr, _marker: PhantomData };
        ref_ptr.try_attach();
        ref_ptr
    }
}

impl<T> Drop for hkRefPtr<T>
where
    T: hkRefPtrCounted,
{
    #[inline]
    fn drop(&mut self) {
        self.try_detach();
    }
}

impl<T> Deref for hkRefPtr<T>
where
    T: hkRefPtrCounted,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        assert!(!self._ptr.is_null(), "Dereferencing a nullptr");
        unsafe { &*self._ptr }
    }
}

impl<T> DerefMut for hkRefPtr<T>
where
    T: hkRefPtrCounted,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(!self._ptr.is_null(), "Dereferencing a nullptr");
        unsafe { &mut *self._ptr }
    }
}

impl<T> PartialEq for hkRefPtr<T>
where
    T: hkRefPtrCounted,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl<T> Eq for hkRefPtr<T> where T: hkRefPtrCounted {}

impl<T> std::fmt::Debug for hkRefPtr<T>
where
    T: hkRefPtrCounted,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("hkRefPtr").field("ptr", &self._ptr).finish()
    }
}

/// Creates a new `hkRefPtr` with a constructed instance of `T`.
#[inline]
pub fn make_hkref<T>(value: T) -> hkRefPtr<T>
where
    T: hkRefPtrCounted,
{
    let boxed = Box::new(value);
    hkRefPtr::from_raw(Box::into_raw(boxed))
}

/// Equality operators with `nullptr`.
impl<T> PartialEq<std::ptr::NonNull<T>> for hkRefPtr<T>
where
    T: hkRefPtrCounted,
{
    #[inline]
    fn eq(&self, other: &std::ptr::NonNull<T>) -> bool {
        self.get() == other.as_ptr()
    }
}

impl<T> PartialEq<*mut T> for hkRefPtr<T>
where
    T: hkRefPtrCounted,
{
    #[inline]
    fn eq(&self, other: &*mut T) -> bool {
        self.get() == *other
    }
}
