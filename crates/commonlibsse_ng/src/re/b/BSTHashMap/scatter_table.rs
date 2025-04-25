pub use super::{Allocator, BSTScatterKeyExtractor, BSTScatterTableHeapAllocator, KeyStrategy};

use super::{EntryType, Iter, SentinelPtr};
use core::alloc::Layout;
use core::mem::MaybeUninit;
use core::ptr::{self, NonNull};

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
    pub(crate) capacity: u32,
    /// Number of free slot
    pub(crate) free: u32,
    /// last free index
    pub(crate) good: u32,
    /// signals end of chain
    pub(crate) sentinel: Option<SentinelPtr<EntryType<S::Pair>>>,
    pub(crate) allocator: A,
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
            return;
        }

        #[allow(clippy::missing_transmute_annotations)]
        self.set_entries(unsafe { core::mem::transmute(new_entries) }); // Assumption that alloc_zeroed is used. and does nothing.
        self.capacity = new_capacity;
        self.free = new_capacity;
        self.good = 0;

        // Move old entries to new entries
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
                self.deallocate_entries(old_entries, old_capacity);
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

    unsafe fn deallocate_entries(&mut self, ptr: SentinelPtr<EntryType<S::Pair>>, capacity: u32) {
        let layout = Self::new_entries_layout(capacity);
        unsafe {
            self.allocator.deallocate(ptr.as_non_null().cast(), layout);
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

    pub(crate) fn clear(&mut self) {
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

    fn free_resources(&mut self) {
        if self.capacity > 0 {
            if let Some(entries) = self.get_entry_start() {
                self.for_each_valid_entry_mut(|entry| {
                    entry.destroy();
                });
                unsafe { self.deallocate_entries(entries, self.capacity) };
                self.allocator.set_entries(ptr::null_mut());
            };
        }

        self.capacity = 0;
        self.free = 0;
        self.good = 0;
    }
}

impl<S, A> Drop for BSTScatterTable<S, A>
where
    S: KeyStrategy,
    A: Allocator,
{
    fn drop(&mut self) {
        self.free_resources();
    }
}
