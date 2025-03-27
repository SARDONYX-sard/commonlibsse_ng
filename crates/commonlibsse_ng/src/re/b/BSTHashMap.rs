mod allocator;

pub use self::allocator::{Allocator, BSTScatterTableHeapAllocator, BSTStaticHashMapBaseAllocator};

use core::ffi::c_void;
use core::hash::Hasher;
use core::marker::PhantomData;

pub struct BSTHashMap<K, V> {
    key: core::marker::PhantomData<K>,
    value: core::marker::PhantomData<V>,
}

impl<K, V> BSTHashMap<K, V> {
    pub fn get(&self, key: &K) -> Option<&V> {
        let _ = key;
        todo!()
    }
}

/// Used as a sentinel value(a special value indicating the end of a table) used within a table
const BST_SCATTER_TABLE_SENTINEL: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];

#[repr(C)]
#[derive(Debug)]
pub struct BSTScatterTable<Hash, KeyEq, Traits_, A>
where
    Hash: Hasher,
    KeyEq: PartialEq,
    Traits_: Traits,
    A: Allocator,
{
    pad00: u64,                                 // 00
    pad08: u32,                                 // 08
    capacity: u32,                              // 0C - total # of slots, always a power of 2
    free: u32,                                  // 10 - # of free slots
    good: u32,                                  // 14 - last free index
    sentinel: *const EntryType<Traits_::Value>, // 18 - signals end of chain
    allocator: A,                               // 20
    marker: PhantomData<(Hash, KeyEq)>,
}

impl<Hash, KeyEqual, Traits_, A> BSTScatterTable<Hash, KeyEqual, Traits_, A>
where
    Hash: Hasher,
    KeyEqual: PartialEq,
    Traits_: Traits,
    A: Allocator,
{
    // std::uint64_t     _pad00{ 0 };
    // std::uint32_t     _pad08{ 0 };
    // size_type         _capacity{ 0 };
    // size_type         _free{ 0 };
    // size_type         _good{ 0 };
    // const entry_type* _sentinel{ reinterpret_cast<const entry_type*>(detail::BSTScatterTableSentinel) };
    // allocator_type    _allocator;

    fn default() -> Self {
        Self {
            pad00: 0,
            pad08: 0,
            capacity: 0,
            free: 0,
            good: 0,
            sentinel: BST_SCATTER_TABLE_SENTINEL.as_ptr().cast(),
            // allocator: A::new(),
            allocator: todo!(),
            marker: PhantomData,
        }
    }
}

impl<Hash, KeyEqual, Traits_, A> BSTScatterTable<Hash, KeyEqual, Traits_, A>
where
    Hash: Hasher,
    KeyEqual: PartialEq,
    Traits_: Traits,
    A: Allocator,
{
    // pub type SizeType = u32;
    // pub type DifferenceType = i32;
    // pub type KeyType = Traits_::Key;
    // pub type MappedType = Traits_::Mapped;
    // pub type ValueType = Traits_::Value;
    // pub type Reference = ValueType;
    // pub type ConstReference = ValueType;
    // pub type Pointer = *mut ValueType;
    // pub type ConstPointer = *const ValueType;

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub const fn len(&self) -> u32 {
        self.capacity - self.free
    }

    fn clear(&mut self) {
        if !self.is_empty() {
            let entries = self.get_entries();
            assert!(!entries.is_null());

            for i in 0..self.capacity {
                unsafe { &mut *entries.wrapping_add(i as usize) }.destroy();
            }
            self.free = self.capacity;
            self.good = 0;
        }

        assert!(self.is_empty());
    }

    fn get_entries(&self) -> *mut EntryType<Traits_::Value> {
        self.allocator.get_entries().cast()
    }

    fn set_entries(&mut self, entires: *mut c_void) {
        self.allocator.set_entries(entires.cast());
    }
}

pub trait Traits {
    type Key;
    type Mapped;
    type Value;
}

struct EntryType<V> {
    value_data: V,
    next: *mut EntryType<V>,
}

impl<V> EntryType<V>
where
    V: Default,
{
    pub fn new() -> Self {
        Self { value_data: V::default(), next: std::ptr::null_mut() }
    }
}

impl<V> EntryType<V> {
    /// Has `next`?
    pub const fn has_value(&self) -> bool {
        !self.next.is_null()
    }

    pub fn destroy(&mut self) {
        if self.has_value() {
            // unsafe {
            // self.value_data.drop_in_place();
            // }
            self.next = core::ptr::null_mut();
        }
        assert!(!self.has_value());
    }

    /// Set value_data & next
    pub fn push(&mut self, value: V, next: *mut Self) {
        self.destroy();
        self.value_data = value;
        self.next = next;
        assert!(self.has_value());
    }

    pub fn steal(&mut self) -> V {
        assert!(self.has_value());
        // let val = self.value_data;
        self.destroy();
        assert!(!self.has_value());
        // val
        todo!()
    }
}

pub struct EntryIter<'a, V> {
    current: Option<&'a EntryType<V>>,
}

impl<'a, V> Iterator for EntryIter<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.current?;
        self.current = unsafe { node.next.as_ref() };
        Some(&node.value_data)
    }
}

// --- IntoIterator ---
impl<'a, V> IntoIterator for &'a EntryType<V> {
    type Item = &'a V;
    type IntoIter = EntryIter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        EntryIter { current: Some(self) }
    }
}

// --- Index ---
impl<V> core::ops::Index<u32> for EntryType<V> {
    type Output = V;

    fn index(&self, mut index: u32) -> &Self::Output {
        let mut current = self;
        while index > 0 {
            if current.next.is_null() {
                panic!("Index out of bounds");
            }
            current = unsafe { &*current.next };
            index -= 1;
        }
        &current.value_data
    }
}

impl<V> core::ops::IndexMut<u32> for EntryType<V> {
    fn index_mut(&mut self, mut index: u32) -> &mut Self::Output {
        let mut current = self;
        while index > 0 {
            if current.next.is_null() {
                panic!("Index out of bounds");
            }
            current = unsafe { &mut *current.next };
            index -= 1;
        }
        &mut current.value_data
    }
}
