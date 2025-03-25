use crate::re::CxxVirtClass;
use crate::re::ExtraDataType::ExtraDataType;
use crate::re::offsets_rtti::RTTI_BSExtraData;
use crate::re::offsets_vtable::VTABLE_BSExtraData;
use crate::rel::ResolvableAddress as _;
use crate::rel::id::VariantID;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::{self, NonNull};

/// Represents the base structure for extra data nodes in a singly linked list.
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct BSExtraData {
    /// Pointer to the virtual function table (vtable).
    pub vtbl: *const BSExtraDataVtbl,
    /// Pointer to the next `BSExtraData` node.
    ///
    /// This is a raw pointer to ensure FFI compatibility. `Option<NonNull<T>>`
    /// would be safer, but raw pointers are required for interoperability.
    /// # Note
    /// This is assumed to be a heap-derived **unique pointer** created from `Box::into_raw`, etc.
    ///
    /// If this process is broken, UB happens.
    pub next: *mut BSExtraData,
    /// Marker indicating that the pointer to next is owned and unique.
    marker: PhantomData<BSExtraData>,
}

const _: () = assert!(core::mem::size_of::<BSExtraData>() == 0x10);

impl BSExtraData {
    /// Address & offset of RTTI for `BSExtraData`.
    pub const RTTI: VariantID = RTTI_BSExtraData;

    /// Address & offset of Virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_BSExtraData;

    /// Default extra data type (None).
    pub const EXTRA_DATA_TYPE: ExtraDataType = ExtraDataType::None;

    /// Creates a new instance of `BSExtraData` with default values.
    #[inline]
    pub const fn new() -> Self {
        Self { vtbl: &BS_EXTRA_DATA_VTBL, next: ptr::null_mut(), marker: PhantomData }
    }

    /// Get the extra data type by invoking the `GetType` virtual function.
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

    /// Compares two `BSExtraData` instances for inequality.
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

    /// Creates a new instance of `T` that implements `CxxVirtClass`.
    ///
    /// Returns an `Option<NonNull<T>>` containing a pointer to the allocated instance, or `None` on failure.
    pub fn create<T: CxxVirtClass>() -> Option<NonNull<T>> {
        let t = &T::vtable()[0];
        Self::create_with(core::mem::size_of::<T>(), t.address().ok()?).map(|void| void.cast())
    }

    /// Allocates and initializes a new `BSExtraData` instance with a given size and vtable.
    ///
    /// # Returns
    /// The pointer to the allocated instance.
    #[inline]
    pub fn create_with(size: usize, vtbl: NonNull<c_void>) -> Option<NonNull<c_void>> {
        use core::alloc::Layout;
        use core::mem::align_of;
        use std::alloc::alloc_zeroed;

        unsafe {
            let memory = {
                let layout = Layout::from_size_align(size, align_of::<Self>()).ok()?;
                NonNull::new(alloc_zeroed(layout))?
            };

            let vtable_ptr = memory.cast();
            vtable_ptr.write(vtbl);

            Some(memory.cast())
        }
    }
}

#[inline]
pub fn downcast_as<T>(extra_data: *mut BSExtraData) -> Option<NonNull<T>> {
    if extra_data.is_null() || !extra_data.is_aligned() {
        return None;
    }

    if crate::rex::win32::is_valid_range(extra_data.cast(), core::mem::size_of::<T>()) {
        return Some(unsafe { NonNull::new_unchecked(extra_data.cast::<T>()) });
    };
    None
}

/// Trait indicating whether or not `BSExtraData` is inherited in the C++ sense.
///
/// Used for downcast availability and linked list traversal.
pub trait DerivedBSExtraData {
    /// Type used for downcast-ing availability and linked list search.
    fn get_extra_data(&self) -> &BSExtraData;
    /// Function for testing whether `BSExtraData` is really inherited. It will not be called in practice.
    fn get_extra_data_type() -> ExtraDataType;
}

impl DerivedBSExtraData for BSExtraData {
    #[inline]
    fn get_extra_data(&self) -> &BSExtraData {
        self
    }

    #[inline]
    fn get_extra_data_type() -> ExtraDataType {
        Self::EXTRA_DATA_TYPE
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

/// Default implemented vtable.
static BS_EXTRA_DATA_VTBL: BSExtraDataVtbl = BSExtraDataVtbl::new();

/// Virtual function table for `BSExtraData`.
#[repr(C)]
pub struct BSExtraDataVtbl {
    /// Destructor (`~BSExtraData` in C++).
    pub CxxDrop: fn(this: &mut BSExtraData),

    /// Gets the type of extra data.
    pub GetType: fn(this: &BSExtraData) -> ExtraDataType,

    /// Checks inequality between two `BSExtraData` instances.
    pub IsNotEqual: fn(this: &BSExtraData, rhs: &BSExtraData) -> bool,
}

impl BSExtraDataVtbl {
    /// Creates a new default virtual function table with no-op functions.
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

/// Iterator over `BSExtraData` nodes.
pub struct BSExtraDataIter<'a> {
    /// Pointer to the current node.
    pub(crate) current: *mut BSExtraData,

    /// Pointer to the previous node.
    pub(crate) prev: *mut BSExtraData,

    /// Lifetime marker for Rust's borrow checker.
    marker: PhantomData<&'a BSExtraData>,
}

impl BSExtraDataIter<'_> {
    /// Creates a new iterator starting at `start`.
    #[inline]
    pub const fn new(start: *mut BSExtraData) -> Self {
        Self { current: start, prev: std::ptr::null_mut(), marker: PhantomData }
    }

    /// Removes the current node and concatenates the previous and next nodes.
    ///
    /// # Returns
    /// Pointer to the removed node.
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

/// Mutable iterator over `BSExtraData` nodes.
pub struct BSExtraDataIterMut<'a> {
    cur: *mut BSExtraData,
    prev: *mut BSExtraData,
    marker: PhantomData<&'a BSExtraData>,
}

impl BSExtraDataIterMut<'_> {
    /// Creates a new iterator starting at `start`.
    #[inline]
    pub const fn new(start: *mut BSExtraData) -> Self {
        Self { cur: start, prev: std::ptr::null_mut(), marker: PhantomData }
    }

    /// Removes the current node and concatenates the previous and next nodes.
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

        // Drop the node, assuming it was heap-allocated.
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_bs_extra_data_creation() {
//         let data = BSExtraData::new();
//         assert_eq!(data.get_type(), ExtraDataType::None);
//     }
// }
