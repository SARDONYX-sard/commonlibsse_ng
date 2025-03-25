use core::{marker::PhantomData, ops::Deref};

pub struct NiPointer<T> {
    ptr: *const T,
    marker: PhantomData<T>,
}

impl<T> Deref for NiPointer<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}
