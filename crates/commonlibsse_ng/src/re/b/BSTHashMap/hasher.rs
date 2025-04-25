use crate::re::CRC::Crc32Hasher;
use core::marker::PhantomData;

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
