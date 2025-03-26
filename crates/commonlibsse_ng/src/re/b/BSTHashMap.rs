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

const BST_SCATTER_TABLE_SENTINEL: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];

// std::uint64_t     _pad00{ 0 };
// std::uint32_t     _pad08{ 0 };
// size_type         _capacity{ 0 };
// size_type         _free{ 0 };
// size_type         _good{ 0 };
// const entry_type* _sentinel{ reinterpret_cast<const entry_type*>(detail::BSTScatterTableSentinel) };
// allocator_type    _allocator;

use core::hash::Hasher;
use core::marker::PhantomData;

#[repr(C)]
#[derive(Debug)]
pub struct BSTScatterTable<Hash, KeyEq, Traits_, Alloc>
where
    Hash: Hasher,
    KeyEq: PartialEq,
    Traits_: Traits,
    Alloc: FnOnce() -> Box<dyn std::alloc::GlobalAlloc>,
{
    pad00: u64,                                 // 00
    pad08: u32,                                 // 08
    capacity: u32,                              // 0C - total # of slots, always a power of 2
    free: u32,                                  // 10 - # of free slots
    good: u32,                                  // 14 - last free index
    sentinel: *const EntryType<Traits_::Value>, // 18 - signals end of chain
    allocator: Alloc,                           // 20
    marker: PhantomData<(Hash, KeyEq)>,
}

impl<Hash, KeyEqual, Traits_, Allocator> BSTScatterTable<Hash, KeyEqual, Traits_, Allocator>
where
    Hash: Hasher,
    KeyEqual: PartialEq,
    Traits_: Traits,
    Allocator: FnOnce() -> Box<dyn std::alloc::GlobalAlloc>,
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
