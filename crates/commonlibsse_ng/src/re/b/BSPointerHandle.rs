use core::marker::PhantomData;

use crate::re::{
    Actor::Actor,
    NiSmartPointer::{NiPointer, RefCountable},
    TESObjectREFR::TESObjectREFR,
};

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
    pub fn reset(&mut self) {
        self.handle = 0;
    }
}

// === BSPointerHandle ===
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

    pub fn reset(&mut self) {
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
    pub extern "C" fn GetHandle(ptr: *mut T) -> BSPointerHandle<T> {
        type SelfSignature = fn(*mut ()) -> u32; // NOTE: Since generics cannot be used with function pointers, use `void` instead.
        {
            static FUNC: std::sync::LazyLock<SelfSignature> = std::sync::LazyLock::new(|| {
                use crate::rel::ResolvableAddress as _;
                use crate::rel::id::RelocationID;
                use core::ffi::c_void;
                use core::ptr::NonNull;

                const SE_ID: u64 = 15967;
                const AE_ID: u64 = 16212;

                let fn_ptr =
                    RelocationID::new(SE_ID, AE_ID, SE_ID).address().unwrap_or_else(|err| {
                        #[cfg(feature = "tracing")]
                        tracing::error!("[Critical Error] Failed to resolve address: {err}");
                        panic!("Failed to resolve address: {err}")
                    });
                unsafe { core::mem::transmute::<NonNull<c_void>, SelfSignature>(fn_ptr) }
            });
            BSPointerHandle::from_raw(FUNC(ptr.cast()))
        }
    }
}

impl<T: RefCountable> BSPointerHandleManagerInterface<T> {
    pub extern "C" fn GetSmartPointer(
        handle: &BSPointerHandle<T>,
        smart_ptr: &mut NiPointer<T>,
    ) -> bool {
        // NOTE: Since generics cannot be used with function pointers, use `void` instead.
        type SelfSignature = fn(handle: *const (), smart_ptr: *mut ()) -> bool;
        {
            static FUNC: std::sync::LazyLock<SelfSignature> = std::sync::LazyLock::new(|| {
                use crate::rel::ResolvableAddress as _;
                use crate::rel::id::RelocationID;
                use core::ffi::c_void;
                use core::ptr::NonNull;

                const SE_ID: u64 = 12204;
                const AE_ID: u64 = 12332;

                let fn_ptr =
                    RelocationID::new(SE_ID, AE_ID, SE_ID).address().unwrap_or_else(|err| {
                        #[cfg(feature = "tracing")]
                        tracing::error!("[Critical Error] Failed to resolve address: {err}");
                        panic!("Failed to resolve address: {err}")
                    });
                unsafe { core::mem::transmute::<NonNull<c_void>, SelfSignature>(fn_ptr) }
            });
            FUNC(
                (handle as *const BSPointerHandle<T>).cast(),
                (smart_ptr as *mut NiPointer<T>).cast(),
            )
        }
    }
}

// === Aliases ===
pub type ActorHandle = BSPointerHandle<Actor>;
pub type ProjectileHandle = BSPointerHandle<Projectile>;
pub type ObjectRefHandle = BSPointerHandle<TESObjectREFR>;

// === Extern C++ ABI Types ===

#[repr(C)]
#[derive(Debug)]
pub struct Projectile;
