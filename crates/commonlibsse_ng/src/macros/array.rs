/// A macro to construct an `hkArrayBase` similar to the `vec![]` macro.
///
/// # Examples
///
/// Create an array with values:
/// ```no_run
/// # use commonlibsse_ng::hk_array;
/// let arr = hk_array![1, 2, 3];
/// ```
///
/// Create an empty array:
/// ```no_run
/// # use commonlibsse_ng::hk_array;
/// use commonlibsse_ng::re::hkArray::hkArray;
/// let empty: hkArray<i8> = hk_array![];
/// ```
///
/// Create an array with a custom allocator:
/// ```
/// # use commonlibsse_ng::hk_array;
/// use commonlibsse_ng::re::hkArray::RustAllocator;
/// let arr = hk_array![with_allocator: RustAllocator => "a", "b", "c"];
/// ```
#[macro_export]
macro_rules! hk_array {
    // Empty hkArray with default allocator
    () => {
        $crate::re::hkArray::hkArray::new()
    };

    ($value:expr; $count:expr) => {{
        let mut arr: $crate::re::hkArray::hkArray<_> = $crate::re::hkArray::hkArray::with_capacity($count);
        for _ in 0..$count {
            arr.push($value);
        }
        arr
    }};

    // hkArray with elements, using default allocator
    ($($elem:expr),+ $(,)?) => {{
        let mut arr: $crate::re::hkArray::hkArray<_> = $crate::re::hkArray::hkArray::with_capacity(0);
        $(
            arr.push($elem);
        )+
        arr
    }};

    (with_allocator: $alloc:ty => $value:expr; $count:expr) => {{
        let mut arr = $crate::re::hkArray::hkArray::<_, $alloc>::with_capacity($count);
        for _ in 0..$count {
            arr.push($value);
        }
        arr
    }};

    // hkArray with elements and custom allocator
    (with_allocator: $alloc:ty => $($elem:expr),+ $(,)?) => {{
        let mut arr = $crate::re::hkArray::hkArray::<_, $alloc>::with_capacity(0);
        $(
            arr.push($elem);
        )+
        arr
    }};
}

/// Creates a `BSTArray` with syntax similar to the standard `vec!` macro.
///
/// # Examples
///
/// Create an array from a list of values:
/// ```no_run
/// # use commonlibsse_ng::bst_array;
/// let arr = bst_array![1_u32, 2, 3];
/// assert_eq!(arr.len(), 3);
/// ```
///
/// Create an array with repeated values:
/// ```no_run
/// # use commonlibsse_ng::bst_array;
/// let arr = bst_array![42_u32; 5];
/// assert_eq!(arr.len(), 5);
/// ```
///
/// Create an empty array:
/// ```no_run
/// # use commonlibsse_ng::bst_array;
/// use commonlibsse_ng::re::BSTArray::BSTArray;
/// let arr: BSTArray<i32> = bst_array![];
/// assert!(arr.is_empty());
/// ```
///
/// Create an array with a custom allocator:
/// ```
/// # use commonlibsse_ng::bst_array;
/// use commonlibsse_ng::re::BSTArray::RustAllocator;
/// let arr = bst_array![with_allocator: RustAllocator => 10, 20];
/// ```
///
/// Create a repeated array with a custom allocator:
/// ```
/// # use commonlibsse_ng::bst_array;
/// use commonlibsse_ng::re::BSTArray::RustAllocator;
/// let arr = bst_array![with_allocator: RustAllocator => 7; 3];
/// ```
///
/// # Note
///
/// This macro uses `.with_capacity()` and `.push()` under the hood.
/// It does **not** call `resize()` or pre-initialize memory.
///
/// The repeated variant (`[value; count]`) requires that `value` is `Clone`,
/// but cloning is done by the caller expression (i.e., `push(value)`).
#[macro_export]
macro_rules! bst_array {
    () => {
        $crate::re::BSTArray::BSTArray::new()
    };

    ($value:expr; $count:expr) => {{
        let mut arr: $crate::re::BSTArray::BSTArray<_> = $crate::re::BSTArray::BSTArray::with_capacity($count);
        for _ in 0..$count {
            arr.push($value);
        }
        arr
    }};

    ($($elem:expr),+ $(,)?) => {{
        let _count = <[()]>::len(&[$($crate::bst_array![@count $elem]),+]);
        let mut arr: $crate::re::BSTArray::BSTArray<_> = $crate::re::BSTArray::BSTArray::with_capacity(_count);
        $(
            arr.push($elem);
        )+
        arr
    }};

    (with_allocator: $alloc:ty => $value:expr; $count:expr) => {{
        let mut arr = $crate::re::BSTArray::BSTArray::<_, $alloc>::with_capacity($count);
        for _ in 0..$count {
            arr.push($value);
        }
        arr
    }};

    (with_allocator: $alloc:ty => $($elem:expr),+ $(,)?) => {{
        let _count = <[()]>::len(&[$($crate::bst_array![@count $elem]),+]);
        let mut arr = $crate::re::BSTArray::BSTArray::<_, $alloc>::with_capacity(_count);
        $(
            arr.push($elem);
        )+
        arr
    }};

    (@count $x:expr) => { () };
}
