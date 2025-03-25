use core::{ffi::c_void, marker::PhantomData};

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectRefHandle {
    ptr: u32,
    marker: PhantomData<c_void>,
}

impl Default for ObjectRefHandle {
    fn default() -> Self {
        Self::null()
    }
}

impl ObjectRefHandle {
    pub const fn null() -> Self {
        Self { ptr: 0, marker: PhantomData }
    }

    pub fn get<T>(&self) -> T {
        // ( self.ptr.cast() )
        todo!()
    }
}
