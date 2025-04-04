use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ptr::NonNull;

pub trait RefCountable {
    /// Increment ref count
    fn inc_ref_count(&self);
    /// Decrement ref count
    fn dec_ref_count(&mut self);
}

#[derive(Debug)]
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

    #[inline]
    pub fn from_raw(ptr: *mut T) -> Self {
        let ni_pointer = Self { ptr: NonNull::new(ptr), _marker: PhantomData };
        ni_pointer.try_attach();
        ni_pointer
    }

    fn try_attach(&self) {
        if let Some(ref_ptr) = self.ptr {
            unsafe { ref_ptr.as_ref().inc_ref_count() };
        }
    }

    fn try_detach(&mut self) {
        if let Some(mut ptr) = self.ptr.take() {
            unsafe { ptr.as_mut().dec_ref_count() };
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.try_detach();
    }

    #[inline]
    pub const fn is_null(&self) -> bool {
        self.ptr.is_none()
    }

    #[inline]
    pub const fn as_ptr(&self) -> Option<NonNull<T>> {
        self.ptr
    }

    #[inline]
    pub fn as_ref<'a>(&self) -> Option<&'a T> {
        self.ptr.as_ref().map(|ptr| unsafe { ptr.as_ref() })
    }

    #[inline]
    pub fn as_mut<'a>(&mut self) -> Option<&'a mut T> {
        self.ptr.as_mut().map(|ptr| unsafe { ptr.as_mut() })
    }
}

impl<T: RefCountable> Clone for NiPointer<T> {
    #[inline]
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
    #[inline]
    fn drop(&mut self) {
        self.try_detach();
    }
}

impl<T: RefCountable> PartialEq for NiPointer<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<T: RefCountable> Eq for NiPointer<T> {}

impl<T: RefCountable> Hash for NiPointer<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(ptr) = self.ptr {
            ptr.as_ptr().hash(state);
        }
    }
}

impl<T: RefCountable> From<*mut T> for NiPointer<T> {
    #[inline]
    fn from(ptr: *mut T) -> Self {
        Self::from_raw(ptr)
    }
}

impl<T: RefCountable> From<Option<NonNull<T>>> for NiPointer<T> {
    #[inline]
    fn from(ptr: Option<NonNull<T>>) -> Self {
        let ret = Self { ptr, _marker: PhantomData };
        ret.try_attach();
        ret
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use core::sync::atomic::{AtomicU32, Ordering};

//     #[derive(Debug, Default)]
//     struct TestBase {
//         count: AtomicU32,
//     }

//     impl RefCountable for TestBase {
//         #[inline]
//         fn inc_ref_count(&self) {
//             self.count.fetch_add(1, Ordering::AcqRel);
//         }

//         #[inline]
//         fn dec_ref_count(&mut self) {
//             if self.count.fetch_sub(1, Ordering::AcqRel) == 1 {};
//         }
//     }

//     #[derive(Debug, Default)]
//     struct TestRefTarget {
//         __base: TestBase,
//     }

//     impl RefCountable for TestRefTarget {
//         #[inline]
//         fn inc_ref_count(&self) {
//             self.__base.inc_ref_count();
//         }

//         #[inline]
//         fn dec_ref_count(&mut self) {
//             self.__base.dec_ref_count();
//         }
//     }

//     #[derive(Debug, Default)]
//     struct TestDerived {
//         __base: TestBase,
//         ptr: NiPointer<TestRefTarget>,
//     }

//     #[test]
//     fn test_ni_pointer() {
//         let mut item = TestDerived::default();
//         assert_eq!(item.__base.count.load(Ordering::Acquire), 1);
//         item.ptr.reset();
//         assert_eq!(item.__base.count.load(Ordering::Acquire), 0);
//     }
// }
