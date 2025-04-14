/// Trait for reference counting.
pub trait RefCounted {
    fn add_ref(&mut self);
    fn release(&mut self);
}

/// Similar to `Arc`, but does not call `release` while the reference count is
/// greater than or equal to zero.
///
/// It differs from `Arc` in that it does not have its own reference-counting field. In other words, it manipulates the pointer it holds via an external reference count and drop implementation.
#[repr(transparent)]
pub struct GPtr<T: RefCounted> {
    ptr: *mut T,
}

impl<T: RefCounted> GPtr<T> {
    #[inline]
    pub const fn new(ptr: *mut T) -> Self {
        Self { ptr }
    }

    #[inline]
    pub const fn null() -> Self {
        Self { ptr: core::ptr::null_mut() }
    }

    #[inline]
    pub fn from_raw(ptr: *mut T) -> Self {
        let mut g_ptr = Self { ptr };
        g_ptr.try_attach();
        g_ptr
    }

    #[inline]
    pub fn reset(&mut self) {
        self.try_detach();
    }

    #[inline]
    pub fn reset_with(&mut self, ptr: *mut T) {
        if self.ptr != ptr {
            self.try_detach();
            self.ptr = ptr;
            self.try_attach();
        }
    }

    #[inline]
    pub const fn get(&self) -> *mut T {
        self.ptr
    }

    #[inline]
    pub const fn as_ref(&self) -> Option<&T> {
        unsafe { self.ptr.as_ref() }
    }

    #[inline]
    pub const fn as_mut(&mut self) -> Option<&mut T> {
        unsafe { self.ptr.as_mut() }
    }

    #[inline]
    pub fn cast<U>(self) -> GPtr<U>
    where
        U: RefCounted,
    {
        GPtr::<U>::new(self.ptr.cast())
    }

    fn try_attach(&mut self) {
        if let Some(ptr) = self.as_mut() {
            ptr.add_ref();
        }
    }

    fn try_detach(&mut self) {
        if let Some(ptr) = self.as_mut() {
            ptr.release();
        }
        self.ptr = core::ptr::null_mut();
    }
}

impl<T: RefCounted> Clone for GPtr<T> {
    #[inline]
    fn clone(&self) -> Self {
        let mut cloned = Self { ptr: self.ptr };
        cloned.try_attach();
        cloned
    }
}

impl<T: RefCounted> Drop for GPtr<T> {
    #[inline]
    fn drop(&mut self) {
        self.try_detach();
    }
}

impl<T: RefCounted> core::ops::Deref for GPtr<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref().expect("GPtr is null")
    }
}

impl<T: RefCounted> core::ops::DerefMut for GPtr<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut().expect("GPtr is null")
    }
}

impl<T: RefCounted> From<*mut T> for GPtr<T> {
    #[inline]
    fn from(ptr: *mut T) -> Self {
        Self::from_raw(ptr)
    }
}

impl<T: RefCounted> PartialEq for GPtr<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<T: RefCounted> Eq for GPtr<T> {}

impl<T: RefCounted> Default for GPtr<T> {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

// impl<T: RefCounted> core::fmt::Debug for GPtr<T> {
//     #[inline]
//     fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
//         f.debug_struct("GPtr").field("ptr", &self.ptr).finish()
//     }
// }
impl<T: RefCounted + core::fmt::Debug> core::fmt::Debug for GPtr<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.as_ref() {
            Some(value) => {
                f.debug_struct("GPtr").field("ptr", &self.ptr).field("value", value).finish()
            }
            None => f.debug_struct("GPtr").field("ptr", &self.ptr).finish(),
        }
    }
}

/// Factory function: `make_g_ptr`
#[inline]
pub fn make_g_ptr<T: RefCounted>(value: T) -> GPtr<T> {
    // FIXME: Write NiMemoryManager
    let boxed = Box::new(value);
    let raw = Box::into_raw(boxed);
    let ptr = GPtr::from_raw(raw);
    // Balance the internal AddRef with this manual release
    unsafe {
        (*raw).release();
    }
    ptr
}
