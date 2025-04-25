//! # BSTHashMap
//!
//! C++: https://github.com/SARDONYX-forks/CommonLibVR/blob/feature/add-ng-release-ci/include/RE/B/BSTHashMap.h
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
mod hasher;
mod scatter_table;

pub use self::allocator::{Allocator, BSTScatterTableHeapAllocator, BSTStaticHashMapBaseAllocator};
pub use self::hasher::{BSTScatterKeyExtractor, KeyStrategy};

use self::entry::SentinelPtr;
use self::entry::{EntryType, Iter};
use self::scatter_table::BSTScatterTable;
use core::fmt;
use core::hash::Hash;

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
        let (pair, res) = self.0.insert((key, value));
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
            return Ok(String::from_utf8_lossy(&w).to_string());
        };

        writeln!(&mut w, "Memory Layout Visualization:")?;
        writeln!(&mut w, "Capacity: {}", self.0.capacity)?;
        writeln!(&mut w, "Free slots: {}", self.0.free)?;
        writeln!(&mut w, "------------------------------------------")?;

        for i in 0..self.0.capacity {
            let entry = unsafe { entries.add(i as usize) };

            if !Self::check_valid_ptr(entry) {
                continue;
            }

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
                if !Self::check_valid_ptr(next_ptr) {
                    continue;
                }

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

        Ok(String::from_utf8_lossy(&w).to_string())
    }

    /// The reason is unknown, but sometimes `usize::MAX` is included in the game for some reason.
    ///
    /// This is a check to avoid a dereference crash in that case.
    fn check_valid_ptr(entry: SentinelPtr<EntryType<(K, V)>>) -> bool {
        let is_ok = entry.as_ptr().addr() != 0xFFFFFFFFFFFFFFFF;

        #[cfg(feature = "tracing")]
        if !is_ok {
            tracing::error!("Couldn't access memory region: (ptr: {entry:?})");
        }

        is_ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap() {
        macro_rules! insert_and_expect {
            ($map:expr, ($key:expr, $val:expr) => $should_insert:expr => $expected_val:expr) => {{
                let (mut iter, inserted) = $map.insert($key, $val);
                assert_eq!(inserted, $should_insert, "inserted mismatch for key {}", $key);

                let actual = iter.next().map(|entry| entry.value_data);
                let expected = Some(($key, $expected_val));
                assert_eq!(actual, expected, "value mismatch for key {}", $key);
            }};
        }

        let mut map = BSTHashMap::new();

        insert_and_expect!(map, (0, c"Hey") => true => c"Hey");
        insert_and_expect!(map, (0, c"DuplicatedKey") => false => c"Hey");
        insert_and_expect!(map, (1, c"Hello") => true => c"Hello");
        insert_and_expect!(map, (2, c"World") => true => c"World");

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
