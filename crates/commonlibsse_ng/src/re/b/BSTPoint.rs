use num_traits::Num;

// NOTE: `BSTPointDefaultOps` is inlined due to the inhibition of Rust struct reuse by Empty base optimization (EBO).

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BSTPoint2<T: Num> {
    pub x: T,
    pub y: T,
}
const _: () = {
    assert!(core::mem::offset_of!(BSTPoint2<f32>, x) == 0);
    assert!(core::mem::offset_of!(BSTPoint2<f32>, y) == 4);

    assert!(core::mem::size_of::<BSTPoint2<f32>>() == 8);
};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BSTPoint3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}
const _: () = {
    assert!(core::mem::offset_of!(BSTPoint3<f32>, x) == 0);
    assert!(core::mem::offset_of!(BSTPoint3<f32>, y) == 4);
    assert!(core::mem::offset_of!(BSTPoint3<f32>, z) == 8);

    assert!(core::mem::size_of::<BSTPoint3<f32>>() == 0xC);
};
