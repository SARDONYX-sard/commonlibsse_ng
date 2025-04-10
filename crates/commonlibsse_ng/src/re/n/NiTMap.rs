use core::ffi::c_void;
use core::fmt::Debug;
use core::hash::{Hash, Hasher};
use core::ptr;

use crate::re::NiTDefaultAllocator::NiTDefaultAllocator;
use crate::re::NiTMapBase::{NiTMapBase, NiTMapBaseVtbl, NiTMapItem};

/// A generic map data structure that supports basic operations like insertion, removal, and lookup.
///
/// This type is designed to be compatible with C++'s `NiTMap` and is exposed through FFI bindings.
///
/// # Note
/// As far as the requirements in the vtable are concerned, key is supposed to be a `Copy` type.
pub struct NiTMap<K, V> {
    pub __base: NiTMapBase<K, V, NiTDefaultAllocator<NiTMapItem<K, V>>>,
}
const _: () = assert!(core::mem::size_of::<NiTMap<*mut c_void, *mut c_void>>() == 0x20);

impl<K, V> NiTMap<K, V> {
    /// Clears all elements from the map.
    ///
    /// This function removes all key-value pairs from the map and resets its state.
    /// The map will be empty after this operation.
    #[inline]
    pub fn clear(&mut self) {
        self.__base.clear();
    }

    /// Returns the number of elements in the map.
    #[inline]
    pub const fn len(&self) -> usize {
        self.__base.len()
    }

    /// Returns `true` if the map is empty, otherwise `false`.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct NiTMapVtbl<K, V> {
    pub __base: NiTMapBaseVtbl<K, V, NiTDefaultAllocator<NiTMapItem<K, V>>>,
}

impl<K, V> NiTMap<K, V>
where
    K: Copy,
{
    /// Gets a reference to the value associated with the given key.
    #[inline]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.__base.get(key)
    }

    /// Gets a mutable reference to the value associated with the given key.
    #[inline]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.__base.get_mut(key)
    }

    /// Checks if the map contains the specified key.
    #[inline]
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Inserts a new key-value pair into the map.
    ///
    /// If the key already exists in the map, this will replace the existing value.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> bool {
        self.__base.insert(key, value)
    }

    /// Removes the key-value pair associated with the given key.
    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.__base.remove(key)
    }

    /// Returns an iterator over the map's key-value pairs.
    #[inline]
    pub const fn iter(&self) -> NiTMapIter<K, V> {
        NiTMapIter { map: &self.__base, bucket_index: 0, current_item: ptr::null_mut() }
    }

    /// Returns an iterator over the keys in the map.
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.iter().map(|(k, _)| k)
    }

    /// Returns an iterator over the values in the map.
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.iter().map(|(_, v)| v)
    }
}

// =====================================================================================================================
// Debug, PartialEq, Eq, PartialOrd, Ord, Hash

impl<K, V> core::fmt::Debug for NiTMap<K, V>
where
    K: Debug + Copy,
    V: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K: Copy + Clone, V: Clone> Clone for NiTMap<K, V> {
    #[inline]
    fn clone(&self) -> Self {
        // If vptr specified static fn table.
        let mut new_map = Self { __base: NiTMapBase::new(self.__base.vtable) };

        for (k, v) in self {
            new_map.insert(*k, v.clone());
        }

        new_map
    }
}

impl<K, V> PartialEq for NiTMap<K, V>
where
    K: Eq + Copy,
    V: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<K, V> Eq for NiTMap<K, V>
where
    K: Eq + Copy,
    V: Eq,
{
}

impl<K, V> PartialOrd for NiTMap<K, V>
where
    K: Ord + Copy,
    V: PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl<K, V> Ord for NiTMap<K, V>
where
    K: Ord + Copy,
    V: Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<K, V> Hash for NiTMap<K, V>
where
    K: Hash + Copy,
    V: Hash,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in self.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}

// =====================================================================================================================

pub struct NiTMapIter<'a, K, V> {
    map: &'a NiTMapBase<K, V, NiTDefaultAllocator<NiTMapItem<K, V>>>,
    bucket_index: usize,
    current_item: *mut NiTMapItem<K, V>,
}

impl<'a, K, V> Iterator for NiTMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.bucket_index < self.map.capacity as usize {
            if !self.current_item.is_null() {
                let item = unsafe { &*self.current_item };
                self.current_item = item.next;
                return Some((&item.key, &item.value));
            }

            self.current_item = unsafe { *self.map.data.add(self.bucket_index) };
            self.bucket_index += 1;
        }
        None
    }
}

impl<'a, K, V> IntoIterator for &'a NiTMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = NiTMapIter<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        NiTMapIter { map: &self.__base, bucket_index: 0, current_item: ptr::null_mut() }
    }
}

pub struct NiTMapIntoIter<K, V> {
    inner: NiTMap<K, V>,
    bucket_index: usize,
    current_item: *mut NiTMapItem<K, V>,
}

impl<K, V> Iterator for NiTMapIntoIter<K, V>
where
    K: Copy,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.bucket_index < self.inner.__base.capacity as usize {
            if !self.current_item.is_null() {
                let item = unsafe { &*self.current_item };
                let result = Some((item.key, unsafe { ptr::read(&item.value) }));
                self.current_item = item.next;
                return result;
            }

            self.current_item = unsafe { *self.inner.__base.data.add(self.bucket_index) };
            self.bucket_index += 1;
        }
        None
    }
}

impl<K, V> IntoIterator for NiTMap<K, V>
where
    K: Copy,
{
    type Item = (K, V);
    type IntoIter = NiTMapIntoIter<K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        NiTMapIntoIter { inner: self, bucket_index: 0, current_item: ptr::null_mut() }
    }
}

impl<K: Copy, V> Extend<(K, V)> for NiTMap<K, V> {
    #[inline]
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

// =====================================================================================================================
