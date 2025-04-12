use core::marker::PhantomData;

use crate::re::Actor::Actor;
use crate::re::NiSmartPointer::{NiPointer, RefCountable};
use crate::re::Projectile;
use crate::re::TESObjectREFR::TESObjectREFR;

// === BSUntypedPointerHandle ===
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BSUntypedPointerHandle {
    handle: u32,
}

impl BSUntypedPointerHandle {
    #[inline]
    pub const fn new() -> Self {
        Self { handle: 0 }
    }

    #[inline]
    pub const fn from_value(value: u32) -> Self {
        Self { handle: value }
    }

    #[inline]
    pub const fn has_value(&self) -> bool {
        self.handle != 0
    }

    #[inline]
    pub const fn value(&self) -> u32 {
        self.handle
    }

    #[inline]
    pub const fn reset(&mut self) {
        self.handle = 0;
    }
}

// === BSPointerHandle ===
#[repr(C)]
#[derive(Debug, Eq, PartialOrd, Ord)]
pub struct BSPointerHandle<T> {
    handle: BSUntypedPointerHandle,
    _phantom: core::marker::PhantomData<T>,
}
const _: () = assert!(core::mem::size_of::<BSPointerHandle<()>>() == 0x4);

impl<T> Clone for BSPointerHandle<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self { handle: self.handle, _phantom: self._phantom }
    }
}

impl<T> Default for BSPointerHandle<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PartialEq<Self> for BSPointerHandle<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl<T> BSPointerHandle<T> {
    #[inline]
    pub const fn new() -> Self {
        Self { handle: BSUntypedPointerHandle::new(), _phantom: PhantomData }
    }

    #[inline]
    pub const fn from_handle(handle: BSUntypedPointerHandle) -> Self {
        Self { handle, _phantom: PhantomData }
    }

    #[inline]
    const fn from_raw(value: u32) -> Self {
        Self { handle: BSUntypedPointerHandle { handle: value }, _phantom: PhantomData }
    }

    pub const fn native_handle(&self) -> u32 {
        self.handle.value()
    }

    pub const fn has_value(&self) -> bool {
        self.handle.has_value()
    }

    pub const fn reset(&mut self) {
        self.handle.reset();
    }
}

impl<T: RefCountable> BSPointerHandle<T> {
    /// Gets a smart pointer from the handle
    pub fn get(&self) -> NiPointer<T> {
        let mut smart_ptr = NiPointer::<T>::new();
        let _ = BSPointerHandleManagerInterface::<T>::GetSmartPointer(self, &mut smart_ptr);
        smart_ptr
    }

    #[inline]
    pub fn as_ref(&self) -> Option<&T> {
        self.get().as_ptr().map(|ptr| unsafe { ptr.as_ref() })
    }
}

// === BSPointerHandleManagerInterface ===
#[repr(C)]
pub struct BSPointerHandleManagerInterface<T> {
    _phantom: core::marker::PhantomData<T>,
}

impl<T> BSPointerHandleManagerInterface<T> {
    pub fn GetHandle(ptr: *mut T) -> BSPointerHandle<T> {
        BSPointerHandle::from_raw(Self::GetHandleRaw(ptr.cast()))
    }

    /// C++ `GetHandle`
    ///
    /// NOTE: Since generics cannot be used with function pointers, use `c_void`(unit type) instead.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15967, ae_id = 16212)]
    #[inline]
    pub extern "C" fn GetHandleRaw(ptr: *mut ()) -> u32 {}
}

impl<T: RefCountable> BSPointerHandleManagerInterface<T> {
    pub fn GetSmartPointer(handle: &BSPointerHandle<T>, smart_ptr: &mut NiPointer<T>) -> bool {
        let handle = (handle as *const BSPointerHandle<T>).cast();
        let smart_ptr = (smart_ptr as *mut NiPointer<T>).cast();
        Self::GetSmartPointerRaw(handle, smart_ptr)
    }

    /// C++ `GetSmartPointer`
    ///
    /// NOTE: Since generics cannot be used with function pointers, use `c_void`(unit type) instead.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 12204, ae_id = 12332)]
    #[inline]
    pub extern "C" fn GetSmartPointerRaw(handle: *const (), smart_ptr: *mut ()) -> bool {}
}

// === Aliases ===
pub type ActorHandle = BSPointerHandle<Actor>;
pub type ProjectileHandle = BSPointerHandle<Projectile>;
pub type ObjectRefHandle = BSPointerHandle<TESObjectREFR>;
