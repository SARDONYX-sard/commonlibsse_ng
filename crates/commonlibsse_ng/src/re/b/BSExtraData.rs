use crate::re::CxxVirtClass;
use crate::re::ExtraDataType::ExtraDataType;
use crate::re::offsets_rtti::RTTI_BSExtraData;
use crate::re::offsets_vtable::VTABLE_BSExtraData;
use crate::rel::ResolvableAddress as _;
use crate::rel::id::VariantID;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::{self, NonNull};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BSExtraData {
    pub vtbl: *const BSExtraDataVtbl,
    /// This pointer is a unique pointer assumption that is made from the heap.
    /// (e.g. Ptr from `Box::into_raw`)
    ///
    /// Originally, `Option<NonNull<BSExtraData>>` would be the best choice, but since it is an FFI type, null must always be considered.
    /// Therefore, a raw pointer is used.
    pub next: *mut BSExtraData,
}
const _: () = assert!(core::mem::size_of::<BSExtraData>() == 0x10);

impl BSExtraData {
    pub const RTTI: VariantID = RTTI_BSExtraData;
    pub const VTABLE: [VariantID; 1] = VTABLE_BSExtraData;
    pub const EXTRA_DATA_TYPE: ExtraDataType = ExtraDataType::None;

    #[inline]
    pub const fn new() -> Self {
        Self { vtbl: &BS_EXTRA_DATA_VTBL, next: ptr::null_mut() }
    }

    #[inline]
    pub fn get_type(&self) -> ExtraDataType {
        let vtable = match Self::VTABLE[0].address() {
            Ok(addr) => addr.cast::<BSExtraDataVtbl>(),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to get address of vtable: {err}");
                return (BS_EXTRA_DATA_VTBL.GetType)(self);
            }
        };
        unsafe { (vtable.as_ref().GetType)(self) }
    }

    #[inline]
    pub fn is_not_equal(&self, rhs: &Self) -> bool {
        let vtable = match Self::VTABLE[0].address() {
            Ok(addr) => addr.cast::<BSExtraDataVtbl>(),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to get address of vtable: {err}");
                return false;
            }
        };
        unsafe { (vtable.as_ref().IsNotEqual)(self, rhs) }
    }

    pub fn create<T: CxxVirtClass>() -> Option<NonNull<T>> {
        let t = &T::vtable()[0];
        Self::create_with(core::mem::size_of::<T>(), t.address().ok()?).map(|void| void.cast())
    }

    #[inline]
    pub fn create_with(size: usize, vtbl: NonNull<c_void>) -> Option<NonNull<c_void>> {
        use core::alloc::Layout;
        use core::mem::align_of;
        use std::alloc::alloc_zeroed;

        unsafe {
            // Step 1: Allocate memory
            let memory = {
                let layout = Layout::from_size_align(size, align_of::<Self>()).ok()?;
                NonNull::new(alloc_zeroed(layout))?
            };

            // Step 2: Set the vtable pointer
            let vtable_ptr = memory.cast();
            vtable_ptr.write(vtbl);

            // Step 3: Cast to BSExtraData and return the pointer
            Some(memory.cast())
        }
    }
}

impl CxxVirtClass for BSExtraData {
    #[inline]
    fn rtti() -> &'static VariantID {
        &Self::RTTI
    }

    #[inline]
    fn vtable() -> &'static [VariantID] {
        &Self::VTABLE
    }
}

impl Default for BSExtraData {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

static BS_EXTRA_DATA_VTBL: BSExtraDataVtbl = BSExtraDataVtbl::new();

#[repr(C)]
pub struct BSExtraDataVtbl {
    /// C++: `~BSExtraData`
    CxxDrop: fn(this: &mut BSExtraData),
    GetType: fn(this: &BSExtraData) -> ExtraDataType,
    IsNotEqual: fn(this: &BSExtraData, rhs: &BSExtraData) -> bool,
}

impl BSExtraDataVtbl {
    /// Create a new default virtual function table
    #[inline]
    const fn new() -> Self {
        const fn CxxDrop(_this: &mut BSExtraData) {}

        const fn GetType(_this: &BSExtraData) -> ExtraDataType {
            BSExtraData::EXTRA_DATA_TYPE
        }

        const fn IsNotEqual(_this: &BSExtraData, _rhs: &BSExtraData) -> bool {
            false
        }

        Self { CxxDrop, GetType, IsNotEqual }
    }
}

pub struct BSExtraDataIter<'a> {
    pub(crate) current: *mut BSExtraData,
    pub(crate) prev: *mut BSExtraData,
    // Zero size type for giving a lifetime annotation for the compiler
    // to detect the duration of the pointer.
    marker: PhantomData<&'a BSExtraData>,
}

impl BSExtraDataIter<'_> {
    #[inline]
    pub const fn new(start: *mut BSExtraData) -> Self {
        Self { current: start, prev: std::ptr::null_mut(), marker: PhantomData }
    }

    /// Deletes the current node and concatenates prev and next
    /// Return value: pointer to the deleted node
    pub fn remove_current(&mut self) -> Option<*mut BSExtraData> {
        if self.current.is_null() {
            return None;
        }

        let removed = self.current;
        unsafe {
            let next = (*removed).next;

            if !self.prev.is_null() {
                (*self.prev).next = next;
            }

            self.current = next;
        }

        Some(removed)
    }
}

impl Iterator for BSExtraDataIter<'_> {
    type Item = *mut BSExtraData;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }

        let current = self.current;
        unsafe {
            self.prev = current;
            self.current = (*current).next;
        }
        Some(current)
    }
}

pub struct BSExtraDataIterMut<'a> {
    cur: *mut BSExtraData,
    prev: *mut BSExtraData,
    // Zero size type for giving a lifetime annotation for the compiler
    // to detect the duration of the pointer.
    marker: PhantomData<&'a BSExtraData>,
}

impl BSExtraDataIterMut<'_> {
    #[inline]
    pub const fn new(start: *mut BSExtraData) -> Self {
        Self { cur: start, prev: std::ptr::null_mut(), marker: PhantomData }
    }

    /// Delete current node to free memory, concatenate prev and next
    pub fn remove_current(&mut self) {
        if self.cur.is_null() {
            return;
        }

        let next = (unsafe { &*self.cur }).next;

        if !self.prev.is_null() {
            (unsafe { &mut *self.prev }).next = next;
        }

        let to_delete = self.cur;
        self.cur = next;

        // NOTE: UB probably occurs if pointer is not made from heap allocation
        drop(unsafe { Box::from_raw(to_delete) });
    }
}

impl Iterator for BSExtraDataIterMut<'_> {
    type Item = *mut BSExtraData;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.is_null() {
            return None;
        }

        let current = self.cur;
        unsafe {
            self.prev = current;
            self.cur = (*current).next;
        }
        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bs_extra_data_creation() {
        let data = BSExtraData::new();
        assert_eq!(data.get_type(), ExtraDataType::None);
    }
}
