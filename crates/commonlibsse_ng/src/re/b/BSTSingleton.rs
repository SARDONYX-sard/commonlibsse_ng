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

pub struct BSTSingletonSDM<T, A = BSTSingletonSDMOpStaticBuffer<T>> {
    pub __base: BSTSingletonSDMBase<T, A>,
}
