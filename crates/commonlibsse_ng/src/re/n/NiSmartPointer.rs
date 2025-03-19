use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ptr::NonNull;

pub trait RefCountable {
    fn inc_ref_count(&self);
    fn dec_ref_count(&self);
}

pub struct NiPointer<T: RefCountable> {
    ptr: Option<NonNull<T>>,
    _marker: PhantomData<T>,
}

impl<T: RefCountable> Default for NiPointer<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: RefCountable> NiPointer<T> {
    #[inline]
    pub const fn new() -> Self {
        Self { ptr: None, _marker: PhantomData }
    }

    /// # Safety
    /// ptr is valid ptr.
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        let ptr = if !ptr.is_null() { Some(unsafe { NonNull::new_unchecked(ptr) }) } else { None };
        let mut ni_pointer = Self { ptr, _marker: PhantomData };
        ni_pointer.try_attach();
        ni_pointer
    }

    pub fn try_attach(&mut self) {
        if let Some(ref_ptr) = self.ptr {
            unsafe {
                ref_ptr.as_ref().inc_ref_count();
            }
        }
    }

    pub fn try_detach(&mut self) {
        if let Some(ref_ptr) = self.ptr.take() {
            unsafe {
                ref_ptr.as_ref().dec_ref_count();
            }
        }
    }

    pub fn reset(&mut self) {
        self.try_detach();
    }

    pub const fn get(&self) -> Option<NonNull<T>> {
        self.ptr
    }

    pub const fn is_some(&self) -> bool {
        self.ptr.is_some()
    }

    pub fn as_ref(&self) -> Option<&T> {
        self.ptr.as_ref().map(|ptr| unsafe { ptr.as_ref() })
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        self.ptr.as_mut().map(|ptr| unsafe { ptr.as_mut() })
    }
}

impl<T: RefCountable> Clone for NiPointer<T> {
    fn clone(&self) -> Self {
        let mut cloned = Self::new();
        if let Some(ptr) = self.ptr {
            cloned.ptr = Some(ptr);
            cloned.try_attach();
        }
        cloned
    }
}

impl<T: RefCountable> Drop for NiPointer<T> {
    fn drop(&mut self) {
        self.try_detach();
    }
}

impl<T: RefCountable> PartialEq for NiPointer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<T: RefCountable> Eq for NiPointer<T> {}

impl<T: RefCountable> Hash for NiPointer<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(ptr) = self.ptr {
            ptr.as_ptr().hash(state);
        }
    }
}

impl<T: RefCountable> From<*mut T> for NiPointer<T> {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[inline]
    fn from(ptr: *mut T) -> Self {
        unsafe { Self::from_raw(ptr) }
    }
}

impl<T: RefCountable> From<Option<NonNull<T>>> for NiPointer<T> {
    fn from(ptr: Option<NonNull<T>>) -> Self {
        Self { ptr, _marker: PhantomData }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::RefCell;

    struct TestRefCount {
        count: RefCell<u32>,
    }

    impl RefCountable for TestRefCount {
        fn inc_ref_count(&self) {
            *self.count.borrow_mut() += 1;
        }

        fn dec_ref_count(&self) {
            *self.count.borrow_mut() -= 1;
        }
    }

    #[test]
    fn test_nipointer() {
        let item = TestRefCount { count: RefCell::new(0) };
        let mut ptr = unsafe { NiPointer::from_raw(Box::into_raw(Box::new(item))) };

        assert_eq!(*ptr.as_ref().unwrap().count.borrow(), 1);
        ptr.reset();
        assert_eq!(*ptr.as_ref().unwrap().count.borrow(), 0);
    }
}
