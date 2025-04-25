//! # BSTHashMap
//!
//! ## Expected Memory Layout (Simplified)
//!
//! This section explains the internal structure of `BSTHashMap`, focusing on how
//! entries are stored and how chains are formed when collisions occur.
//!
//! ---
//!
//! ### Basic Structure (Before Insertion)
//!
//! Empty is represented by a byte slice of 0 bytes. This is because it is zero initialized at the same time as the memory allocation.
//! However, it is impossible to determine whether it is already initialized or `false`.
//! Therefore, when pushing, the pointer to the next entry is filled with a sentinel value(0xdeadbeef address).
//!
//! In this way, it is possible to distinguish between empty and occupied.
//! In other words, by looking at next, it is possible to tell if data is contained.
//!
//! ```text
//! // Entry<T> array
//! // Accessed via get_entry_for(...)
//!
//! ┌────────────┬────────────┬────────────┬────────────┐
//! │ Entry[0]   │ Entry[1]   │ Entry[2]   │ ...        │  ← capacity N
//! ├────────────┼────────────┼────────────┼────────────┤
//! │ empty      │ empty      │ empty      │ ...        │
//! └────────────┴────────────┴────────────┴────────────┘
//! ```
//!
//! ---
//!
//! ### Basic Insertion Case (Slot Is Empty)
//!
//! ```text
//! // Emplace directly into Entry[i] corresponding to unwrap_key(a_value)!
//!
//! ┌────────────┬────────────┬────────────┬────────────┐
//! │ Entry[0]   │ Entry[1]   │ Entry[2]   │ ...        │
//! ├────────────┼────────────┼────────────┼────────────┤
//! │ empty      │            │ value_data: a_value     │
//! │            │            │ next: _sentinel         │
//! └────────────┴────────────┴────────────┴────────────┘
//!                                    ↑
//!                     get_entry_for(key)
//! ```
//!
//! ---
//!
//! ### Collision Case (Chaining with `next` Pointer)
//!
//! ```text
//! // Entry[2] is occupied → emplace into a new free Entry (e.g., Entry[4])
//! // Update Entry[2].next to point to Entry[4]
//!
//! ┌────────────┬────────────┬────────────┬────────────┬────────────┐
//! │ Entry[0]   │ Entry[1]   │ Entry[2]   │ Entry[3]   │ Entry[4]   │
//! ├────────────┼────────────┼────────────┼────────────┼────────────┤
//! │ empty      │            │ value_data: A           │ unused     │ value_data: B
//! │            │            │ next ___________________/            │ next: _sentinel
//! └────────────┴────────────┴────────────┴────────────┴────────────┘
//!                             ↑
//!                                entry->next = &Entry[4]
//! ```
//!
//! ---
//!
//! ### Eviction Case (Entry Needs Relocation)
//!
//! ```text
//! // A is in Entry[2] but belongs in Entry[0]
//! // Move A to Entry[4] → Emplace B into Entry[2]
//!
//! ┌────────────┬────────────┬────────────┬────────────┬────────────┐
//! │ Entry[0]   │ Entry[1]   │ Entry[2]   │ Entry[3]   │ Entry[4]   │
//! ├────────────┼────────────┼────────────┼────────────┼────────────┤
//! │            │            │ value_data: B           │ unused     │ value_data: A
//! │            │            │ next: _sentinel         │            │ next: _sentinel
//! │            │            │                         │            │
//! │            │            │ ← emplace here          │ ← evicted  │
//! └────────────┴────────────┴────────────┴────────────┴────────────┘
//! ```
// # Implementation notes
// The value is initialized to 0 when allocated, but this is not always valid as Key and Value (e.g. &CStr, NonZeroUsize, etc.), and if it is accessed as Key and Value by mistake, it is undefined behavior.

#![allow(clippy::type_repetition_in_bounds)]
mod allocator;
mod entry;

use entry::SentinelPtr;

pub use self::allocator::{Allocator, BSTScatterTableHeapAllocator, BSTStaticHashMapBaseAllocator};

use self::entry::{EntryType, Iter};
use crate::re::CRC::Crc32Hasher;
use core::alloc::Layout;
use core::fmt;
use core::hash::Hash;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ptr::{self, NonNull};

/// dummy type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct UnkKey(i32);

/// dummy value
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct UnkValue(i32);

#[repr(transparent)]
pub struct BSTHashMap<K, V>(
    BSTScatterTable<BSTScatterKeyExtractor<K, V>, BSTScatterTableHeapAllocator>,
)
where
    K: Hash + Eq;
const _: () = assert!(core::mem::size_of::<BSTHashMap<(), ()>>() == 0x30);

impl<K, V> fmt::Debug for BSTHashMap<K, V>
where
    K: Hash + Eq,
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        for (k, v) in self.iter() {
            map.entry(k, v);
        }
        map.finish()
    }
}

impl<K, V> Default for BSTHashMap<K, V>
where
    K: Hash + Eq,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K, V> BSTHashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self(BSTScatterTable::default())
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let binding = self.0.get_entry_start();
        let value = binding.iter().find(|value| unsafe {
            value.as_non_sentinel_ref().is_some_and(|entry| entry.value_data.0 == *key)
        })?;

        let pair = unsafe { &value.as_non_sentinel_ref()?.value_data };
        Some(&pair.1)
    }

    // TODO: Return prev Option<(K, V)>
    pub fn insert(&mut self, key: K, value: V) -> (Iter<(K, V)>, bool) {
        let (pair, res) = self.0.do_insert((key, value));
        (pair, res)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (K, V)> {
        self.0.get_entry_start().into_iter().flat_map(|entries| {
            (0..self.0.capacity).filter_map(move |i| unsafe {
                Some(&mut entries.add(i as usize).as_non_sentinel_mut()?.value_data)
            })
        })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct BSTScatterTable<S, A>
where
    S: KeyStrategy,
    A: Allocator,
{
    pad00: u64,
    pad08: u32,
    /// total of slots, always a power of 2
    capacity: u32,
    /// Number of free slot
    free: u32,
    /// last free index
    good: u32,
    /// signals end of chain
    sentinel: Option<SentinelPtr<EntryType<S::Pair>>>,
    allocator: A,
}
const _: () = {
    type TestHashMap =
        BSTScatterTable<BSTScatterKeyExtractor<i32, i32>, BSTScatterTableHeapAllocator>;

    assert!(core::mem::offset_of!(TestHashMap, pad00) == 0x00);
    assert!(core::mem::offset_of!(TestHashMap, pad08) == 0x08);
    assert!(core::mem::offset_of!(TestHashMap, capacity) == 0x0C);
    assert!(core::mem::offset_of!(TestHashMap, free) == 0x10);
    assert!(core::mem::offset_of!(TestHashMap, good) == 0x14);
    assert!(core::mem::offset_of!(TestHashMap, sentinel) == 0x18);
    assert!(core::mem::offset_of!(TestHashMap, allocator) == 0x20);

    assert!(core::mem::size_of::<TestHashMap>() == 0x30);
};

#[derive(Debug, Clone, Copy, Default)]
pub struct BSTScatterKeyExtractor<K, V> {
    marker: PhantomData<(K, V)>,
}

pub trait KeyStrategy {
    type Key;
    type Value;
    /// Key value pair(or Single)
    type Pair;

    /// e.g. Gets first tuple(Value) element from tuple
    fn get_key(value: &Self::Pair) -> &Self::Key;

    fn hash(key: &Self::Key) -> u32;
}

impl<K, V> KeyStrategy for BSTScatterKeyExtractor<K, V>
where
    K: core::hash::Hash,
{
    type Key = K;

    type Value = V;

    type Pair = (K, V);

    #[inline]
    fn get_key(value: &Self::Pair) -> &Self::Key {
        &value.0
    }

    #[inline]
    fn hash(key: &Self::Key) -> u32 {
        use core::hash::{BuildHasher as _, BuildHasherDefault};

        type Crc32Hash = BuildHasherDefault<Crc32Hasher>;

        Crc32Hash::new().hash_one(key) as u32
    }
}

impl<S, A> Default for BSTScatterTable<S, A>
where
    S: KeyStrategy,
    A: Allocator + Default,
{
    fn default() -> Self {
        Self {
            pad00: 0,
            pad08: 0,
            capacity: 0,
            free: 0,
            good: 0,
            sentinel: Some(SentinelPtr::sentinel()),
            allocator: A::default(),
        }
    }
}

impl<S, A> BSTScatterTable<S, A>
where
    S: KeyStrategy<Key: PartialEq>,
    A: Allocator,
{
    #[inline]
    pub fn insert(&mut self, value: S::Pair) -> (Iter<S::Pair>, bool) {
        self.do_insert(value)
    }

    #[inline]
    pub fn insert_range<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Option<S::Pair>>,
    {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();
        self.reserve(self.len() + lower as u32);

        for value in iter.flatten() {
            let _ = self.insert(value);
        }
    }

    fn do_insert(&mut self, pair: S::Pair) -> (Iter<S::Pair>, bool) {
        if let Some(iter) = self.find(S::get_key(&pair)) {
            return (iter, false);
        }

        if self.free == 0 {
            self.reserve(self.capacity + 1);
            assert!(self.free > 0);
        }

        self.free -= 1;
        let mut entry = self.get_entry_for(S::get_key(&pair));

        if entry.is_some_and(|entry| unsafe {
            entry.as_non_sentinel_ref().is_some_and(|entry| entry.is_occupied())
        }) {
            // 3a. Resolve conflict
            let mut free = self.get_free_entry();

            let would_ve = entry
                .as_ref()
                .and_then(|e| unsafe { e.as_non_sentinel_ref() })
                .map(|e| S::get_key(&e.value_data))
                .and_then(|key| self.get_entry_for(key));

            if would_ve == entry {
                // Hash conflict. then add chain
                let prev_next =
                    core::mem::replace(unsafe { &mut entry.unwrap().as_mut().next }, free);
                unsafe { free.unwrap().as_mut().push(pair, prev_next) };
                return (Iter::new(free), true);
            };

            // Interrupted in the middle of the chain, so replace and correct the chain
            let mut prev = would_ve;
            while {
                let prev_ref = prev.and_then(|prev| unsafe { prev.as_non_sentinel_ref() });
                let next = prev_ref.and_then(|prev| prev.next);
                next != entry
            } {
                prev = prev.and_then(|prev| unsafe { prev.as_non_sentinel_ref() }?.next);
            }

            // evict current value and detach from chain
            if free.is_some() {
                if let Some(entry) = entry.take() {
                    free = Some(entry);
                }
            }
            unsafe { prev.unwrap().as_mut().next = free };
        };

        unsafe { entry.unwrap().as_mut().push(pair, Some(SentinelPtr::sentinel())) };
        (Iter::new(entry), true)
    }

    fn find<'a>(&self, key: &S::Key) -> Option<Iter<'a, S::Pair>> {
        if self.is_empty() {
            return None;
        }

        let mut current_entry = self.get_entry_for(key)?;

        while let Some(entry_ref) = unsafe { current_entry.as_non_sentinel_ref() } {
            // If the entry is not occupied, we've reached a sentinel — exit
            if !entry_ref.is_occupied() {
                break;
            }

            let current_key = S::get_key(&entry_ref.value_data);
            if current_key == key {
                return Some(entry_ref.iter());
            }

            current_entry = entry_ref.next?;
        }

        None
    }

    fn get_entry_for(&self, key: &S::Key) -> Option<SentinelPtr<EntryType<S::Pair>>> {
        assert!(self.get_entry_start().is_some());

        let hash = S::hash(key);
        let idx = hash & (self.capacity - 1); // quick modulo

        let entries = self.get_entry_start()?;
        unsafe { Some(entries.add(idx as usize)) }
    }

    /// Get one free entry.
    fn get_free_entry(&mut self) -> Option<SentinelPtr<EntryType<S::Pair>>> {
        assert!(self.free > 0);
        assert!(self.get_entry_start().is_some());
        assert!(self.capacity.is_power_of_two());
        debug_assert!(self.entries().is_some_and(|e| unsafe {
            e.as_non_sentinel_ref().is_some_and(|e| e.iter().any(|e| e.is_occupied()))
        })); // check has free entry.

        let entries = self.get_entry_start()?;

        while unsafe { entries.add(self.good as usize).as_non_sentinel_ref() }?.is_occupied() {
            self.good = (self.good + 1) & (self.capacity - 1); //  wrap around w/ quick modulo
        }

        Some(unsafe { entries.add(self.good as usize) })
    }

    fn reserve(&mut self, new_capacity: u32) {
        if new_capacity <= self.capacity {
            return;
        }

        let old_capacity = self.capacity;
        let old_entries = self.get_entry_start();

        let (new_capacity, new_entries) = {
            let min = A::min_size();
            assert!((0..(u8::MAX as u32)).contains(&min), "Must be > 0");
            let new_capacity = new_capacity.max(min);
            if new_capacity > (1 << 31) {
                panic!("BSTScatterTable: buffer grew too large");
            }

            unsafe { self.allocate_entries(new_capacity) }.map_or_else(
                || panic!("BSTScatterTable: allocation failed"),
                |entries| (new_capacity, entries),
            )
        };

        if old_entries.is_some_and(|old| old.addr() == new_entries.addr()) {
            // Instead of `std::uninitialized_default_construct_`
            unsafe {
                core::ptr::write_bytes(
                    old_entries.unwrap().add(old_capacity as usize).as_ptr(),
                    0,
                    (new_capacity - old_capacity) as usize
                        * core::mem::size_of::<EntryType<S::Pair>>(),
                );
            }

            let mut todo = Vec::with_capacity(self.len() as usize);
            for i in 0..old_capacity {
                if let Some(entry) =
                    unsafe { old_entries.unwrap().add(i as usize).as_non_sentinel_mut() }
                {
                    if entry.is_occupied() {
                        todo.push(entry.steal());
                    }
                };
            }

            self.capacity = new_capacity;
            self.free = new_capacity;
            self.good = 0;

            self.insert_range(todo);
        } else {
            // Assumption that alloc_zeroed is used. and does nothing.
            #[allow(clippy::missing_transmute_annotations)]
            self.set_entries(unsafe { core::mem::transmute(new_entries) });

            self.capacity = new_capacity;
            self.free = new_capacity;
            self.good = 0;

            if let Some(old_entries) = old_entries {
                unsafe {
                    for i in 0..old_capacity {
                        let Some(entry) = old_entries.add(i as usize).as_non_sentinel_mut() else {
                            continue;
                        };
                        if entry.is_occupied() {
                            if let Some(pair) = entry.steal() {
                                self.insert(pair);
                            }
                        }
                    }

                    ptr::drop_in_place(core::slice::from_raw_parts_mut(
                        old_entries.as_ptr(),
                        old_capacity as usize,
                    ));
                    self.deallocate_entries(old_entries);
                }
            }
        }
    }
}

impl<S, A> BSTScatterTable<S, A>
where
    S: KeyStrategy,
    A: Allocator,
{
    /// NOTE: count not bytes size
    #[allow(clippy::type_complexity)]
    unsafe fn allocate_entries(
        &mut self,
        count: u32,
    ) -> Option<SentinelPtr<[MaybeUninit<EntryType<S::Pair>>]>> {
        let ptr = unsafe { self.allocator.allocate(Self::new_entries_layout(count)).ok() }?;
        Some(SentinelPtr::slice_from_raw_parts(ptr.cast(), count as usize))
    }

    unsafe fn deallocate_entries(&mut self, ptr: SentinelPtr<EntryType<S::Pair>>) {
        unsafe {
            self.allocator.deallocate(ptr.as_non_null().cast(), self.current_entries_layout());
        };
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub const fn len(&self) -> u32 {
        self.capacity - self.free
    }

    /// Get entries start.
    pub fn get_entry_start(&self) -> Option<SentinelPtr<EntryType<S::Pair>>> {
        let base_ptr: *mut EntryType<S::Pair> = self.allocator.get_entries().cast();
        SentinelPtr::new(base_ptr)
    }

    fn clear(&mut self) {
        if self.is_empty() {
            return;
        }
        self.for_each_valid_entry_mut(|entry| {
            entry.destroy();
        });
        self.free = self.capacity;
        self.good = 0;

        debug_assert!(self.is_empty());
    }

    fn entries(&self) -> Option<SentinelPtr<[EntryType<S::Pair>]>> {
        let ptr = self.get_entry_start()?.as_non_null();
        Some(SentinelPtr::slice_from_raw_parts(ptr, self.capacity as usize))
    }

    fn for_each_valid_entry_mut<F>(&self, mut f: F)
    where
        F: FnMut(&mut EntryType<S::Pair>),
    {
        let mut current: Option<&mut EntryType<S::Pair>> =
            NonNull::new(self.allocator.get_entries().cast())
                .map(|mut ptr| unsafe { ptr.as_mut() });

        while let Some(entry) = current {
            f(entry);
            current = unsafe { entry.next.as_mut().and_then(|ptr| ptr.as_non_sentinel_mut()) };
        }
    }

    fn set_entries(&mut self, entires: SentinelPtr<[EntryType<S::Pair>]>) {
        self.allocator.set_entries(entires.as_ptr().cast());
    }

    fn new_entries_layout(capacity: u32) -> Layout {
        Layout::array::<EntryType<S::Pair>>(capacity as usize).expect("[BSTHashMap] valid Layout")
    }

    fn current_entries_layout(&self) -> Layout {
        Layout::array::<EntryType<S::Pair>>(self.capacity as usize)
            .expect("[BSTHashMap] valid Layout")
    }
}

impl<S, A> Drop for BSTScatterTable<S, A>
where
    S: KeyStrategy,
    A: Allocator,
{
    fn drop(&mut self) {
        self.clear();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Iterator
pub struct BSTHashMapIter<'a, K, V>
where
    K: Hash + Eq,
{
    entries: Option<SentinelPtr<EntryType<(K, V)>>>,
    capacity: u32,
    index: usize,
    current: Option<&'a EntryType<(K, V)>>,
}

impl<K, V> BSTHashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn iter(&self) -> BSTHashMapIter<'_, K, V> {
        let entries = self.0.get_entry_start();
        BSTHashMapIter { entries, capacity: self.0.capacity, index: 0, current: None }
    }
}

impl<'a, K, V> Iterator for BSTHashMapIter<'a, K, V>
where
    K: Hash + Eq,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            loop {
                // Process the next chain entry, if any
                if let Some(curr) = self.current {
                    match &curr.next {
                        Some(next) => {
                            let Some(next_ref) = next.as_non_sentinel_ref() else {
                                self.current = None;
                                continue;
                            };
                            self.current = Some(next_ref);
                            let (k, v) = &next_ref.value_data;
                            return Some((k, v));
                        }
                        None => self.current = None,
                    }
                }

                // Next entry after the chain is finished.
                if self.index >= (self.capacity as usize) {
                    return None;
                }

                let entry = self.entries?.add(self.index).as_non_sentinel_ref()?;
                self.index += 1;

                if !entry.is_occupied() {
                    continue;
                }

                let (k, v) = &entry.value_data;
                return Some((k, v));
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Debug

impl<K, V> BSTHashMap<K, V>
where
    K: Hash + Eq + fmt::Debug,
    V: fmt::Debug,
{
    /// Show the memory layout of the `BSTHashMap`.
    ///
    /// # Errors
    /// Failed to write string.
    /// # Panics
    /// Invalid utf8
    ///
    /// # Examples
    /// ```
    /// use commonlibsse_ng::re::BSTHashMap::BSTHashMap;
    ///
    /// let mut map = BSTHashMap::new();
    /// map.insert(42, "hello");
    /// map.insert(17, "world");
    /// map.insert(99, "extra");
    /// map.insert(13, "foo");
    ///
    /// print!("{}", map.show_memory_layout().unwrap());
    ///
    /// // Output =>
    ///
    /// // Memory Layout Visualization:
    /// // Capacity: 8
    /// // Free slots: 4
    /// // ------------------------------------------
    /// // [000] 17 => "world"
    /// // [001] 13 => "foo"
    /// // [002] 42 => "hello" -> 17 => "world"
    /// // [003] EMPTY (Non-occupied)
    /// // [004] EMPTY (Non-occupied)
    /// // [005] 99 => "extra" -> 13 => "foo"
    /// // [006] EMPTY (Non-occupied)
    /// // [007] EMPTY (Non-occupied)
    /// // ------------------------------------------
    /// ```
    #[allow(clippy::unwrap_in_result)]
    pub fn show_memory_layout(&self) -> std::io::Result<String> {
        use std::io::Write as _;

        let mut w = Vec::new();

        let Some(entries) = self.0.get_entry_start() else {
            writeln!(&mut w, "No entries allocated.")?;
            return Ok(String::from_utf8(w).expect("Invalid UTF-8"));
        };

        writeln!(&mut w, "Memory Layout Visualization:")?;
        writeln!(&mut w, "Capacity: {}", self.0.capacity)?;
        writeln!(&mut w, "Free slots: {}", self.0.free)?;
        writeln!(&mut w, "------------------------------------------")?;

        for i in 0..self.0.capacity {
            let entry = unsafe { entries.add(i as usize) };
            let Some(entry_ref) = (unsafe { entry.as_non_sentinel_ref() }) else {
                writeln!(&mut w, "[{:03}] EMPTY", i)?;
                continue;
            };

            if !entry_ref.is_occupied() {
                writeln!(&mut w, "[{:03}] EMPTY (Non-occupied)", i)?;
                continue;
            }

            let (k, v) = &entry_ref.value_data;
            let mut chain = vec![format!("{:?} => {:?}", k, v)];

            // follow chain if exists
            let mut next = entry_ref.next;
            while let Some(next_ptr) = next {
                let Some(next_ref) = (unsafe { next_ptr.as_non_sentinel_ref() }) else {
                    break;
                };
                if !next_ref.is_occupied() {
                    break;
                }

                let (nk, nv) = &next_ref.value_data;
                chain.push(format!("{:?} => {:?}", nk, nv));
                next = next_ref.next;
            }

            writeln!(&mut w, "[{:03}] {}", i, chain.join(" -> "))?;
        }

        writeln!(&mut w, "------------------------------------------")?;

        Ok(String::from_utf8(w).expect("Invalid UTF-8"))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap() {
        let mut map = BSTHashMap::new();

        {
            let (mut iter, is_inserted) = map.insert(0, c"Hey");
            assert!(is_inserted);

            let actual = iter.next().map(|entry| entry.value_data);
            let expected = Some((0, c"Hey"));
            assert_eq!(actual, expected);
        }
        {
            let (mut iter, is_inserted) = map.insert(0, c"DuplicatedKey");
            assert!(!is_inserted, "duplicate key, should not insert");

            let actual = iter.next().map(|entry| entry.value_data);
            let expected = Some((0, c"Hey"));
            assert_eq!(actual, expected);
        }

        {
            let (mut iter, is_inserted) = map.insert(1, c"Hello");
            assert!(is_inserted);

            let actual = iter.next().map(|entry| entry.value_data);
            let expected = Some((1, c"Hello"));
            assert_eq!(actual, expected);
        }
        {
            let (mut iter, is_inserted) = map.insert(2, c"World");
            assert!(is_inserted);

            let actual = iter.next().map(|entry| entry.value_data);
            let expected = Some((2, c"World"));
            assert_eq!(actual, expected);
        }

        for (k, v) in map.iter() {
            dbg!(k, v);
        }
    }

    #[test]
    fn show_memory() {
        let mut map = BSTHashMap::new();
        map.insert(42, "hello");
        map.insert(17, "world");
        map.insert(99, "extra");
        map.insert(13, "foo");

        print!("{}", map.show_memory_layout().unwrap());

        // Memory Layout Visualization:
        // Capacity: 8
        // Free slots: 4
        // ------------------------------------------
        // [000] 17 => "world"
        // [001] 13 => "foo"
        // [002] 42 => "hello" -> 17 => "world"
        // [003] EMPTY (Non-occupied)
        // [004] EMPTY (Non-occupied)
        // [005] 99 => "extra" -> 13 => "foo"
        // [006] EMPTY (Non-occupied)
        // [007] EMPTY (Non-occupied)
        // ------------------------------------------
    }
}
