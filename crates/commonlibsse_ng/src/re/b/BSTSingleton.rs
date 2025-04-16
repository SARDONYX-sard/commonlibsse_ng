pub struct BSTSingletonExplicit<T> {
    _marker: std::marker::PhantomData<T>,
}

pub struct BSTSingletonImplicit<T> {
    _marker: std::marker::PhantomData<T>,
}

pub struct BSTSingletonSDMOpStaticBuffer<T> {
    _marker: std::marker::PhantomData<T>,
}

// A: Allocator
pub struct BSTSDMTraits<Type, Allocator> {
    _marker: core::marker::PhantomData<(Type, Allocator)>,
}

pub struct BSTSingletonSDMBase<T, A> {
    pub traits: BSTSDMTraits<T, A>,
    pub allocator: A,
}

/// This is used when the inherited class does not enable Empty Base Optimization and a u8 address is allocated.
///
/// If Empty Base Optimization works, it should be omitted. (Otherwise, the memory layout will not match.)
pub struct BSTSingletonSDM<T, A = BSTSingletonSDMOpStaticBuffer<T>> {
    pub __base: BSTSingletonSDMBase<T, A>,
    /// C++ Empty class unique address.
    pub address: u8,
}
