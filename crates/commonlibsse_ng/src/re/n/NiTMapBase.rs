use core::ptr;

#[repr(C)]
pub struct NiTMapItem<K, V> {
    pub(crate) next: *mut NiTMapItem<K, V>,
    /// C++ `first`
    pub(crate) key: K,
    /// C++ `second`
    pub(crate) value: V,
}
const _: () = assert!(core::mem::size_of::<NiTMapItem<u32, u64>>() == 0x18);

#[repr(C)]
pub struct NiTMapBase<K, V, A> {
    pub(crate) vtable: *const NiTMapBaseVtbl<K, V, A>,
    pub(crate) capacity: u32,
    pub(crate) pad0c: u32,
    pub(crate) data: *mut *mut NiTMapItem<K, V>,
    pub(crate) allocator: AntiBloatAllocator<A>,
}

impl<K, V, A> NiTMapBase<K, V, A> {
    #[inline]
    pub const fn new(vptr: *const NiTMapBaseVtbl<K, V, A>) -> Self {
        Self {
            vtable: vptr,
            capacity: 37,
            pad0c: 0,
            data: ptr::null_mut(),
            allocator: AntiBloatAllocator::new(),
        }
    }

    #[inline]
    pub const fn with_capacity(capacity: u32) -> Self {
        Self {
            vtable: ptr::null(),
            capacity,
            pad0c: 0,
            data: ptr::null_mut(),
            allocator: AntiBloatAllocator::new(),
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.capacity {
            let current_data_ptr = unsafe { self.data.add(i as usize) };

            while let Some(elem) = unsafe { (*current_data_ptr).as_mut() } {
                unsafe { *current_data_ptr = elem.next };
                self.clear_value(elem);
                self.free_value(elem);
            }
        }

        self.allocator.size = 0;
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.allocator.size as usize
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    fn vtable(&self) -> &NiTMapBaseVtbl<K, V, A> {
        debug_assert!(!self.vtable.is_null());
        unsafe { &*self.vtable }
    }

    #[inline]
    pub(crate) fn hash_function(&self, key: K) -> u32 {
        (self.vtable().hash_function)(self, key)
    }

    #[inline]
    pub(crate) fn key_eq(&self, lhs: K, rhs: K) -> bool {
        (self.vtable().key_eq)(self, lhs, rhs)
    }

    #[inline]
    fn assign_value(&self, value: *mut NiTMapItem<K, V>, key: K, mapped: V) {
        (self.vtable().assign_value)(self, value, key, mapped);
    }

    #[inline]
    fn clear_value(&mut self, value: *mut NiTMapItem<K, V>) {
        (self.vtable().clear_value)(self, value);
    }

    #[inline]
    fn malloc_value(&mut self) -> *mut NiTMapItem<K, V> {
        (self.vtable().malloc_value)(self)
    }

    #[inline]
    fn free_value(&mut self, value: *mut NiTMapItem<K, V>) {
        (self.vtable().free_value)(self, value);
    }
}

impl<K, V, A> NiTMapBase<K, V, A>
where
    K: Copy,
{
    /// C++ `insert_or_assign`
    ///
    /// # Panics
    pub fn insert(&mut self, key: K, value: V) -> bool {
        let index = self.hash_function(key) as usize;

        let mut current_item = unsafe { *self.data.add(index) };
        while let Some(item_) = unsafe { current_item.as_mut() } {
            if self.key_eq(key, item_.key) {
                item_.value = value;
                return false;
            }
            current_item = item_.next;
        }

        current_item = self.malloc_value();

        assert!(!current_item.is_null(), "The current_item after malloc_value must not be null.");
        self.assign_value(current_item, key, value);

        // Shift by one and insert.
        let prev_item_ptr = unsafe { self.data.add(index) };
        unsafe { (*current_item).next = *prev_item_ptr };
        unsafe { *prev_item_ptr = current_item };
        self.allocator.size += 1;

        true
    }

    /// Copy and return the data corresponding to the key, and delete the map.
    ///
    /// - C++ `erase`
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.hash_function(*key);
        let current_item = unsafe { &mut *self.data.add(index as usize) };

        let mut prev: *mut NiTMapItem<K, V> = ptr::null_mut();
        let mut item_ptr = unsafe { current_item.as_mut() };

        while let Some(item) = item_ptr {
            if self.key_eq(*key, item.key) {
                if prev.is_null() {
                    *current_item = item.next;
                } else {
                    unsafe { (*prev).next = item.next };
                }

                // Take ownership of value
                let value = unsafe {
                    // Safety: we're taking out the value, and will drop the rest properly.
                    ptr::read(&item.value)
                };

                self.remove_value(item);
                return Some(value);
            }
            prev = item;
            item_ptr = unsafe { item.next.as_mut() };
        }

        None
    }

    fn remove_value(&mut self, value: *mut NiTMapItem<K, V>) {
        self.clear_value(value);
        self.free_value(value);
        self.allocator.size -= 1;
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let key = *key;
        let hash = self.hash_function(key);
        let index = (hash % self.capacity) as usize;

        let mut current = unsafe { *self.data.add(index) };
        while let Some(node) = unsafe { current.as_ref() } {
            if self.key_eq(node.key, key) {
                return Some(&node.value);
            }
            current = node.next;
        }

        None
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let key = *key;
        let hash = self.hash_function(key);
        let index = (hash % self.capacity) as usize;

        let mut current = unsafe { *self.data.add(index) };
        while let Some(node) = unsafe { current.as_mut() } {
            if self.key_eq(node.key, key) {
                return Some(&mut node.value);
            }
            current = node.next;
        }

        None
    }
}

#[repr(C)]
pub(crate) struct AntiBloatAllocator<A> {
    pub(crate) size: u32,
    alloc_marker: core::marker::PhantomData<A>,
}
impl<A> AntiBloatAllocator<A> {
    #[inline]
    pub const fn new() -> Self {
        Self { size: 0, alloc_marker: core::marker::PhantomData }
    }
}

#[allow(clippy::type_complexity)]
pub struct NiTMapBaseVtbl<K, V, A> {
    pub CxxDrop: fn(this: *mut NiTMapBase<K, V, A>),

    pub hash_function: fn(this: *const NiTMapBase<K, V, A>, key: K) -> u32, // 0x01 - { return a_key % _capacity; }
    pub key_eq: fn(this: *const NiTMapBase<K, V, A>, lhs: K, rhs: K) -> bool, // 0x02 - { return stricmp(a_lhs == a_rhs); }
    pub assign_value:
        fn(this: *const NiTMapBase<K, V, A>, value: *mut NiTMapItem<K, V>, key: K, mapped: V), // 0x03 - { a_value->key = a_key; a_value->mapped = a_mapped; }
    pub clear_value: fn(this: *mut NiTMapBase<K, V, A>, value: *mut NiTMapItem<K, V>), // 0x04 - { return; }
    pub malloc_value: fn(this: *mut NiTMapBase<K, V, A>) -> *mut NiTMapItem<K, V>,     // 0x05
    pub free_value: fn(this: *mut NiTMapBase<K, V, A>, value: *mut NiTMapItem<K, V>),  // 0x06
}
