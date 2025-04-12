use core::marker::PhantomData;

use crate::re::Actor::Actor;
use crate::re::NiSmartPointer::{NiPointer, RefCountable};
use crate::re::Projectile;
use crate::re::TESObjectREFR::TESObjectREFR;

/// A raw 32-bit untyped handle used internally for object references.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BSUntypedPointerHandle(u32);

impl BSUntypedPointerHandle {
    /// Creates a new handle with a value of 0 (null).
    #[inline]
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a handle from a raw [`u32`] value.
    /// - Equivalent C++ method: `from_value`
    #[inline]
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }

    /// Returns `true` if this handle is non-zero.
    #[inline]
    pub const fn has_value(&self) -> bool {
        self.0 != 0
    }

    /// Returns the raw [`u32`] value of the handle.
    /// - Equivalent C++ method: `value`
    #[inline]
    pub const fn as_raw(&self) -> u32 {
        self.0
    }

    /// Resets this handle to 0.
    #[inline]
    pub const fn reset(&mut self) {
        self.0 = 0;
    }
}

/// A typed handle referencing an object of type `T`.
///
/// This provides type safety for handles, and can be used to retrieve a smart pointer or reference.
#[repr(C)]
#[derive(Debug)]
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

impl<T> Eq for BSPointerHandle<T> {}

impl<T> BSPointerHandle<T> {
    /// Creates a new, null handle.
    #[inline]
    pub const fn new() -> Self {
        Self { handle: BSUntypedPointerHandle::new(), _phantom: PhantomData }
    }

    /// Creates a handle from an untyped handle.
    #[inline]
    pub const fn from_handle(handle: BSUntypedPointerHandle) -> Self {
        Self { handle, _phantom: PhantomData }
    }

    /// Creates a handle from a raw [`u32`] value.
    #[inline]
    const fn from_raw(value: u32) -> Self {
        Self { handle: BSUntypedPointerHandle::from_raw(value), _phantom: PhantomData }
    }

    /// Returns the raw native handle value.
    pub const fn native_handle(&self) -> u32 {
        self.handle.as_raw()
    }

    /// Returns true if the handle has a non-zero value.
    pub const fn has_value(&self) -> bool {
        self.handle.has_value()
    }

    /// Resets the handle to 0.
    pub const fn reset(&mut self) {
        self.handle.reset();
    }
}

impl<T: RefCountable> BSPointerHandle<T> {
    /// Retrieves a smart pointer to the underlying object.
    #[inline]
    pub fn get(&self) -> NiPointer<T> {
        let mut smart_ptr = NiPointer::<T>::new();
        let _ = BSPointerHandleManagerInterface::<T>::GetSmartPointer(self, &mut smart_ptr);
        smart_ptr
    }

    /// Attempts to get a shared reference to the underlying object.
    #[inline]
    pub fn as_ref(&self) -> Option<&T> {
        self.get().as_ptr().map(|ptr| unsafe { ptr.as_ref() })
    }
}

/// Interface for converting between raw pointers and handles.
#[repr(C)]
pub struct BSPointerHandleManagerInterface<T> {
    _phantom: core::marker::PhantomData<T>,
}

impl<T> BSPointerHandleManagerInterface<T> {
    /// Gets a typed handle from a raw pointer.
    pub fn GetHandle(ptr: *mut T) -> BSPointerHandle<T> {
        BSPointerHandle::from_raw(unsafe { Self::GetHandleRaw(ptr.cast()) })
    }

    /// FFI to retrieve a raw handle from a pointer.
    ///
    /// # Safety
    /// This uses relocation and works with raw void pointers.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15967, ae_id = 16212)]
    #[inline]
    pub unsafe extern "C" fn GetHandleRaw(ptr: *mut ()) -> u32 {}
}

impl<T: RefCountable> BSPointerHandleManagerInterface<T> {
    /// Gets a smart pointer from the handle and stores it in `smart_ptr`.
    pub fn GetSmartPointer(handle: &BSPointerHandle<T>, smart_ptr: &mut NiPointer<T>) -> bool {
        let handle = (handle as *const BSPointerHandle<T>).cast();
        let smart_ptr = (smart_ptr as *mut NiPointer<T>).cast();
        // Safety: handle & smart_ptr are correct type.
        unsafe { Self::GetSmartPointerRaw(handle, smart_ptr) }
    }

    /// FFI to retrieve a smart pointer from a handle.
    ///
    /// # Safety
    /// This uses relocation and raw pointers for compatibility with native layout.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 12204, ae_id = 12332)]
    #[inline]
    pub unsafe extern "C" fn GetSmartPointerRaw(handle: *const (), smart_ptr: *mut ()) -> bool {}
}

/// Macro to generate strongly-typed handle wrappers over [`BSPointerHandle<T>`].
///
/// Provides new type-style wrappers and utility methods for specific handle types.
macro_rules! impl_handle_wrapper {
    ($(
        $(#[$meta:meta])*
        $vis:vis struct $name:ident : $inner:ty;
    )*) => {
        $(
            $(#[$meta])*
            /// Strongly-typed handle to an object.
            ///
            /// Use `get` or `as_ref` to access the underlying object.
            #[repr(transparent)]
            #[derive(Debug, Clone, Default, PartialEq)]
            $vis struct $name(BSPointerHandle<$inner>);

            impl $name {
                /// Creates a new, null handle.
                #[inline]
                pub const fn new() -> Self {
                    Self(BSPointerHandle::new())
                }

                /// Creates a typed handle from an untyped handle.
                #[inline]
                pub const fn from_handle(handle: BSUntypedPointerHandle) -> Self {
                    Self(BSPointerHandle::from_handle(handle))
                }

                /// Creates a typed handle from a raw [`u32`] value.
                #[inline]
                pub const fn from_raw(value: u32) -> Self {
                    Self(BSPointerHandle::from_raw(value))
                }

                /// Gets the raw [`u32`] value of the handle.
                #[inline]
                pub const fn as_raw(&self) -> u32 {
                    self.0.native_handle()
                }

                /// Returns `true` if the handle is non-zero.
                #[inline]
                pub const fn has_value(&self) -> bool {
                    self.0.has_value()
                }

                /// Resets the handle to 0.
                #[inline]
                pub const fn reset(&mut self) {
                    self.0.reset()
                }
            }

            impl From<BSPointerHandle<$inner>> for $name {
                #[inline]
                fn from(inner: BSPointerHandle<$inner>) -> Self {
                    Self(inner)
                }
            }

            impl From<$name> for BSPointerHandle<$inner> {
                #[inline]
                fn from(wrapper: $name) -> Self {
                    wrapper.0
                }
            }

            impl $name {
                /// Retrieves a smart pointer to the underlying object.
                #[inline]
                pub fn get(&self) -> NiPointer<$inner> {
                    self.0.get()
                }

                /// Attempts to get a shared reference to the object.
                #[inline]
                pub fn as_ref(&self) -> Option<&$inner> {
                    self.0.as_ref()
                }
            }
        )*
    };
}

impl_handle_wrapper! {
    pub struct ActorHandle : Actor;
    pub struct ProjectileHandle : Projectile;
    pub struct ObjectRefHandle : TESObjectREFR;
}
