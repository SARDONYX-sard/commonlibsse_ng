use core::ptr;

use crate::re::BSExtraData::{BSExtraDataIter, BSExtraDataIterMut};
use crate::re::ExtraDataType::ExtraDataType;
use crate::re::{BSAtomic::BSReadWriteLock, BSExtraData::BSExtraData};
use crate::rel::relocation::PhantomMember;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExtraDataList {
    extra_data: BaseExtraList,
    /// Zero-size dummy member
    lock: PhantomMember<BSReadWriteLock, 0x10, 0x18>,
}

impl ExtraDataList {
    /// - calc cost: O(n)
    #[inline]
    pub fn has_type(&self, type_: ExtraDataType) -> bool {
        let _lock = self.lock.get();
        match self.extra_data.presence.get() {
            Ok(presence) => {
                unsafe { presence.as_ref() }.is_some_and(|presence| presence.has_type(type_.bits()))
            }
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Error getting presence address: {err}");
                false
            }
        }
    }

    /// The target `to_remove` is removed from the LinkedList, but the memory itself is not removed in this function.
    ///
    /// If `to_remove` is not removed by the user, it will remain a memory leak.
    pub fn remove(&mut self, type_: ExtraDataType, to_remove: *mut BSExtraData) -> bool {
        let _lock = match self.lock.get_mut() {
            Ok(lock) => lock.write(),
            Err(_err) => return false,
        };

        if to_remove.is_null() {
            return false;
        }

        let mut removed = false;

        let data = match self.extra_data.data.0.get_mut() {
            Ok(data) => data,
            Err(_) => return false,
        };
        if (*data) == to_remove {
            *data = (unsafe { &**data }).next;
            removed = true;
        } else {
            let mut iter = self.extra_data.data.iter();
            while iter.next().is_some() {
                if iter.remove_current().is_some() {
                    removed = true;
                };
            }
        }

        if removed {
            if let Ok(presence) = self.extra_data.presence.get_mut() {
                match unsafe { presence.as_mut() } {
                    Some(presence) => presence.mark_type(type_.bits(), true),
                    None => return false,
                };
            };
        }

        removed
    }

    /// The type to be removed is removed from the LinkedList and the memory itself is `Drop::drop`.
    pub fn remove_by_type(&mut self, type_: ExtraDataType) -> bool {
        let _lock = match self.lock.get_mut() {
            Ok(lock) => lock.write(),
            Err(_err) => return false,
        };

        let mut removed = false;

        let mut iter = self.extra_data.data.iter_mut();
        while let Some(data) = iter.next() {
            if (unsafe { &*data }).get_type() == type_ {
                iter.remove_current();
            }
            removed = true;
        }

        if removed {
            if let Ok(presence) = self.extra_data.presence.get_mut() {
                match unsafe { presence.as_mut() } {
                    Some(presence) => presence.mark_type(type_.bits(), true),
                    None => return false,
                };
            };
        }

        removed
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 12176, ae_id = 12315)]
    pub fn add(&mut self, to_add: *mut BSExtraData) -> *mut BSExtraData {}

    /// - calc cost: O(n)
    #[inline]
    pub fn get_by_type(&self, type_: ExtraDataType) -> Option<*mut BSExtraData> {
        let _lock = self.lock.get();
        self.extra_data
            .data
            .iter()
            .find(|data| unsafe { data.as_ref() }.is_some_and(|data| data.get_type() == type_))
    }

    /// - calc cost: O(n)
    #[inline]
    pub fn get_by_type_mut(&mut self, type_: ExtraDataType) -> Option<*mut BSExtraData> {
        let _lock = self.lock.get_mut();
        self.extra_data
            .data
            .iter_mut()
            .find(|data| unsafe { data.as_ref() }.is_some_and(|data| data.get_type() == type_))
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BaseExtraList {
    opaque: [u8; 1],
    data: Data,
    presence: PhantomMember<*mut PresenceBitfield, 0x8, 0x10>,
}

/// Wrapper type for iterator
#[derive(Debug, Clone)]
pub struct Data(pub PhantomMember<*mut BSExtraData, 0x0, 0x8>);

impl Data {
    ///It has a `remove current` method, which only concatenates prev and next without `drop` the current pointer.
    /// This means memory leaks.
    #[inline]
    pub fn iter(&self) -> BSExtraDataIter<'_> {
        BSExtraDataIter::new(self.0.get().copied().unwrap_or(ptr::null_mut()))
    }

    /// It has a `remove current` method, which only `drops` the current pointer and concatenates prev and next.
    /// In other words, it completely deletes the data itself.
    #[inline]
    pub fn iter_mut(&mut self) -> BSExtraDataIterMut<'_> {
        BSExtraDataIterMut::new(self.0.get().copied().unwrap_or(ptr::null_mut()))
    }
}

impl<'a> IntoIterator for &'a Data {
    type Item = *mut BSExtraData;
    type IntoIter = BSExtraDataIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// `IntoIterator` for mutable references
impl<'a> IntoIterator for &'a mut Data {
    type Item = *mut BSExtraData;
    type IntoIter = BSExtraDataIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PresenceBitfield {
    pub bits: [u8; 0x18], // size: 24
}
const _: () = assert!(core::mem::size_of::<PresenceBitfield>() == 0x18);

impl PresenceBitfield {
    pub const fn has_type(&self, type_: u32) -> bool {
        let index = (type_ >> 3) as usize;
        if index >= self.bits.len() {
            return false;
        }
        let bit_mask = 1 << (type_ % 8);
        (self.bits[index] & bit_mask) != 0
    }

    pub const fn mark_type(&mut self, type_: u32, cleared: bool) {
        let index = (type_ >> 3) as usize;
        if index >= self.bits.len() {
            return;
        }
        let bit_mask = 1 << (type_ % 8);
        let flag = &mut self.bits[index];
        if cleared {
            *flag &= !bit_mask;
        } else {
            *flag |= bit_mask;
        }
    }
}
