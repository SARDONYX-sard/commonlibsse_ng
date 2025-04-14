mod allocator;

use crate::re::CRC::Crc32Hasher;

pub use self::allocator::{Allocator, BSTScatterTableHeapAllocator, BSTStaticHashMapBaseAllocator};

use core::fmt;
use core::hash::Hash;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ptr::{self, NonNull};

#[repr(transparent)]
pub struct BSTHashMap<K, V>(
    BSTScatterTable<BSTScatterKeyExtractor<K, V>, BSTScatterTableHeapAllocator>,
)
where
    K: Hash + Eq;
const _: () = assert!(core::mem::size_of::<BSTHashMap<(), ()>>() == 0x30);

impl<K, V> fmt::Debug for BSTHashMap<K, V>
where
    K: fmt::Debug + Hash + Eq,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        if let Some(entries) = self.0.get_entry_start() {
            for i in 0..self.0.capacity {
                unsafe {
                    let entry = entries.add(i as usize).as_ref();
                    if let Some((k, v)) = entry.value_data.as_ref() {
                        map.entry(k, v);
                        let mut next = entry.next;
                        while let Some(n) = next {
                            let e = n.as_ref();
                            if let Some((k, v)) = e.value_data.as_ref() {
                                map.entry(k, v);
                            }
                            if e.is_sentinel() {
                                break;
                            }
                            next = e.next;
                        }
                    }
                }
            }
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
            value.as_ref().value_data.as_ref().is_some_and(|value| value.0 == *key)
        })?;

        let pair = unsafe { value.as_ref().value_data.as_ref() }?;
        Some(&pair.1)
    }

    // TODO: Return prev Option<(K, V)>
    pub fn insert(&mut self, key: K, value: V) -> bool {
        self.0.do_insert((key, value)).1
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.0.get_entry_start().into_iter().flat_map(|entries| {
            (0..self.0.capacity).filter_map(move |i| {
                let entry = unsafe { entries.add(i as usize).as_ref() };
                let pair = entry.value_data.as_ref()?;
                Some(pair)
            })
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (K, V)> {
        self.0.get_entry_start().into_iter().flat_map(|entries| {
            (0..self.0.capacity).filter_map(move |i| unsafe {
                let entry = entries.add(i as usize).as_mut();
                entry.value_data.as_mut()
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
    sentinel: Option<NonNull<EntryType<S::Pair>>>,
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
            sentinel: None, // EntryType::BST_SCATTER_TABLE_SENTINEL.as_ptr().cast()
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
        // if let Some(iter) = self.find(S::get_key(&pair)) {
        //     return (iter, false);
        // }

        if self.free == 0 {
            self.reserve(self.capacity + 1);
            assert!(self.free > 0);
        }

        let mut entry = self.get_entry_for(S::get_key(&pair));

        if entry.is_some_and(|entry| unsafe { entry.as_ref().has_next() }) {
            // 3a. Resolve conflict
            let free = self.get_free_entry();
            let would_ve = unsafe {
                self.get_entry_for(S::get_key(
                    entry.unwrap().as_ref().value_data.as_ref().expect("value must be exists"),
                ))
            };

            if would_ve == entry {
                // Hash conflict. then add chain
                let prev_next =
                    core::mem::replace(unsafe { &mut entry.unwrap().as_mut().next }, free);
                unsafe { free.unwrap().as_mut().push(pair, prev_next) };
                return (Iter::new(free), true);
            };

            // Interrupted in the middle of the chain, so replace and correct the chain
            let mut prev = would_ve;
            while let Some(next) = unsafe { prev.unwrap().as_ref().next } {
                if Some(next) == entry {
                    break;
                }
                prev = Some(next);
            }

            // evict current value and detach from chain
            if let Some(mut free) = free {
                if let Some(entry) = entry.take() {
                    let free = unsafe { free.as_mut() };
                    *free = unsafe { entry.read() };
                }
            }
            unsafe { prev.unwrap().as_mut().next = free };
        };

        unsafe { entry.unwrap().as_mut().push(pair, None) };
        (Iter::new(entry), true)
    }

    #[allow(unused)]
    #[allow(clippy::unwrap_used)]
    #[allow(clippy::unwrap_in_result)]
    fn find(&mut self, key: &S::Key) -> Option<Iter<S::Pair>> {
        if self.is_empty() {
            return None;
        }

        let mut current_entry = self.get_entry_for(key);
        unsafe {
            if { current_entry?.as_ref() }.has_next() {
                loop {
                    let entry_ref = current_entry.as_ref()?.as_ref();
                    let pair = entry_ref.value_data.as_ref().expect("must has value");
                    if S::get_key(pair) == key {
                        return Some(entry_ref.iter());
                    }

                    current_entry = entry_ref.next;
                    if current_entry.is_some_and(|e| e.as_ref().is_sentinel()) {
                        break;
                    }
                }
            }
        }

        None
    }

    fn get_entry_for(&mut self, key: &S::Key) -> Option<NonNull<EntryType<S::Pair>>> {
        assert!(self.get_entry_start().is_some());
        assert!(self.capacity.is_power_of_two());

        let hash = S::hash(key);
        let idx = hash & (self.capacity - 1);

        let entries = self.get_entry_start()?;
        while unsafe { (entries.add(self.good as usize)).as_ref().has_next() } {
            self.good = (self.good + 1) & (self.capacity - 1); // wrap around w/ quick modulo
        }

        unsafe { Some(entries.add(idx as usize)) }
    }

    /// Get one free entry.
    fn get_free_entry(&mut self) -> Option<NonNull<EntryType<S::Pair>>> {
        assert!(self.free > 0);
        assert!(self.get_entry_start().is_some());
        assert!(self.capacity.is_power_of_two());

        let entries = self.get_entry_start()?;
        debug_assert!(unsafe { entries.as_ref().iter().any(|e| !e.has_next()) }); // check has free entry.

        while (unsafe { entries.as_ref() }[self.good]).has_next() {
            self.good = (self.good + 1) & (self.capacity - 1); //  wrap around w/ quick modulo
        }

        Some(unsafe { entries.add(self.good as usize) })
    }

    fn reserve(&mut self, elements_count: u32) {
        if elements_count <= self.capacity {
            return;
        }

        let old_capacity = self.capacity;
        let old_entries = self.get_entry_start();

        let (new_capacity, mut new_entries) = {
            let min = A::min_size();
            assert!((0..(u8::MAX as u32)).contains(&min), "Must be > 0");
            let new_capacity = elements_count.max(min);
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
                std::ptr::write_bytes(
                    old_entries.unwrap().add(old_capacity as usize).as_ptr(),
                    0,
                    (new_capacity - old_capacity) as usize
                        * std::mem::size_of::<EntryType<S::Pair>>(),
                );
            }

            let mut todo = Vec::with_capacity(self.len() as usize);
            for i in 0..old_capacity {
                let entry = unsafe { old_entries.unwrap().add(i as usize).as_mut() };
                if entry.has_next() {
                    todo.push(entry.steal());
                }
            }

            self.capacity = new_capacity;
            self.free = new_capacity;
            self.good = 0;

            self.insert_range(todo);
        } else {
            // SAFETY: 呼び出し元が、全要素が初期化済みであることを保証する必要がある
            const unsafe fn assume_init_slice<T>(ptr: NonNull<[MaybeUninit<T>]>) -> NonNull<[T]> {
                let raw = ptr.as_ptr() as *mut [T];
                unsafe { NonNull::new_unchecked(raw) }
            }

            for entry in unsafe { new_entries.as_mut() } {
                entry.write(EntryType::new());
            }
            self.set_entries(unsafe { assume_init_slice(new_entries) });

            self.capacity = new_capacity;
            self.free = new_capacity;
            self.good = 0;

            if let Some(old_entries) = old_entries {
                unsafe {
                    for i in 0..old_capacity {
                        let entry = old_entries.add(i as usize).as_mut();
                        if entry.has_next() {
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

    /// NOTE: count not bytes size
    #[allow(clippy::type_complexity)]
    unsafe fn allocate_entries(
        &mut self,
        count: u32,
    ) -> Option<NonNull<[MaybeUninit<EntryType<S::Pair>>]>> {
        let count = count as usize;
        let layout = core::alloc::Layout::array::<EntryType<S::Pair>>(count).ok()?;
        let raw_ptr = NonNull::new(unsafe { self.allocator.allocate_bytes(layout.size()) })?;
        Some(NonNull::slice_from_raw_parts(raw_ptr.cast(), count))
    }

    unsafe fn deallocate_entries(&mut self, ptr: NonNull<EntryType<S::Pair>>) {
        unsafe { self.allocator.deallocate_bytes(ptr.as_ptr().cast()) };
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub const fn len(&self) -> u32 {
        self.capacity - self.free
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

    /// Get entries start.
    pub fn get_entry_start(&self) -> Option<NonNull<EntryType<S::Pair>>> {
        let base_ptr: *mut EntryType<S::Pair> = self.allocator.get_entries().cast();
        NonNull::new(base_ptr)
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
            current = unsafe { entry.next.map(|ptr| &mut *ptr.as_ptr()) };
        }
    }

    fn set_entries(&mut self, entires: NonNull<[EntryType<S::Pair>]>) {
        self.allocator.set_entries(entires.as_ptr().cast());
    }
}

/// Data storage unidirectional linked list of hashmaps formed on the heap or stack.
///
/// There is a possibility that this storage may always be zero
/// because of the timing of the zero initialization when the `reserve` method of the hashmap is called.
///
/// Therefore, use [`Option`].
#[repr(C)]
pub struct EntryType<Pair> {
    /// key, value pair
    value_data: Option<Pair>,
    next: Option<NonNull<EntryType<Pair>>>,
}
const _: () = {
    // To avoid memory access violations, the smallest type (e.g., u8) other than the zero size
    // type must be larger than the sentinel size (4 bytes). or larger.
    const SIZE: usize = core::mem::size_of::<EntryType<u8>>();
    assert!(SIZE == 0x10);
};

impl<Pair> EntryType<Pair> where Pair: Default {}

impl<Pair> Default for EntryType<Pair> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Pair> EntryType<Pair> {
    /// Used as a sentinel value(a special value indicating the end of a table) used within a table
    pub const BST_SCATTER_TABLE_SENTINEL: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];

    /// Create a new `EntryType<V>` as alloc zeroed
    #[inline]
    pub const fn new() -> Self {
        Self { value_data: None, next: None }
    }

    /// Is iter end
    pub fn is_sentinel(&self) -> bool {
        let is_zst = core::mem::size_of::<Self>() == 0;
        if is_zst {
            return false;
        }

        let bytes = Self::BST_SCATTER_TABLE_SENTINEL.as_slice();
        // Safety: Except for the zero size type, the size is 16 bytes even for the smallest type u8.
        // SENTINEL is 4bytes, so it is not an access violation.
        let entry_bytes = unsafe {
            let ptr: *const u8 = (self as *const Self).cast();
            core::slice::from_raw_parts(ptr, Self::BST_SCATTER_TABLE_SENTINEL.len())
        };
        bytes == entry_bytes
    }

    /// Has `next`?
    pub const fn has_next(&self) -> bool {
        self.next.is_some()
    }

    pub fn destroy(&mut self) {
        if self.has_next() {
            let _ = self.value_data.take();
            self.next = None;
        }
        debug_assert!(!self.has_next());
    }

    /// Set value_data & next
    pub fn push(&mut self, value: Pair, next: Option<NonNull<Self>>) {
        self.value_data = Some(value);
        self.next = next;
        // debug_assert!(self.has_next());
    }

    #[inline]
    pub const fn steal(&mut self) -> Option<Pair> {
        self.value_data.take()
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, Pair> {
        Iter { current: Some(NonNull::from(self)), _marker: PhantomData }
    }
}

pub struct Iter<'a, Pair> {
    current: Option<NonNull<EntryType<Pair>>>,
    _marker: PhantomData<&'a Pair>,
}

impl<Pair> Iter<'_, Pair> {
    #[inline]
    pub const fn new(current: Option<NonNull<EntryType<Pair>>>) -> Self {
        Self { current, _marker: PhantomData }
    }
}

impl<'a, Pair> Iterator for Iter<'a, Pair> {
    type Item = &'a EntryType<Pair>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_ptr = self.current?;
        let current_ref = unsafe { current_ptr.as_ref() };
        self.current = current_ref.next;
        Some(current_ref)
    }
}

impl<'a, Pair> IntoIterator for &'a EntryType<Pair> {
    type Item = &'a EntryType<Pair>;
    type IntoIter = Iter<'a, Pair>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { current: Some(NonNull::from(self)), _marker: PhantomData }
    }
}

pub struct IterMut<'a, Pair> {
    current: Option<NonNull<EntryType<Pair>>>,
    _marker: PhantomData<&'a mut Pair>,
}

impl<'a, Pair> Iterator for IterMut<'a, Pair> {
    type Item = &'a mut EntryType<Pair>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_ptr = self.current?;
        let current_mut = unsafe { current_ptr.as_mut() };
        self.current = current_mut.next;
        Some(current_mut)
    }
}

impl<'a, Pair> IntoIterator for &'a mut EntryType<Pair> {
    type Item = &'a mut EntryType<Pair>;
    type IntoIter = IterMut<'a, Pair>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut { current: Some(NonNull::from(self)), _marker: PhantomData }
    }
}

pub struct IntoIter<Pair> {
    current: Option<NonNull<EntryType<Pair>>>,
    _marker: PhantomData<EntryType<Pair>>,
}

impl<Pair> Iterator for IntoIter<Pair> {
    type Item = Pair;

    fn next(&mut self) -> Option<Self::Item> {
        let ptr = self.current?;
        let owned_entry = unsafe { ptr.as_ptr().read() };
        self.current = owned_entry.next;
        owned_entry.value_data
    }
}

impl<Pair> IntoIterator for EntryType<Pair> {
    type Item = Pair;
    type IntoIter = IntoIter<Pair>;

    fn into_iter(self) -> Self::IntoIter {
        let ptr = NonNull::new(&self as *const _ as *mut Self);
        #[allow(clippy::mem_forget)]
        core::mem::forget(self); // prevent drop
        IntoIter { current: ptr, _marker: PhantomData }
    }
}

impl<Pair> core::ops::Index<u32> for EntryType<Pair> {
    type Output = Self;

    fn index(&self, mut index: u32) -> &Self::Output {
        let mut current = self;

        while index > 0 {
            let next_ptr = current.next.expect("Index out of bounds");
            current = unsafe { next_ptr.as_ref() };
            index -= 1;
        }

        current
    }
}

impl<Pair> core::ops::IndexMut<u32> for EntryType<Pair> {
    fn index_mut(&mut self, mut index: u32) -> &mut Self::Output {
        let mut current = self;

        while index > 0 {
            let mut next_ptr = current.next.expect("Index out of bounds");
            current = unsafe { next_ptr.as_mut() };
            index -= 1;
        }

        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap() {
        let mut map = BSTHashMap::new();
        map.insert(0, "Hey");
        map.insert(0, "Hello");
        map.insert(1, "World");
        for (k, v) in map.iter() {
            dbg!(k, v);
        }
        dbg!(map);
    }
}
