use core::sync::atomic::{AtomicU32, Ordering};

#[repr(C)]
#[derive(Debug, Default)]
pub struct BSIntrusiveRefCounted {
    pub refCount: AtomicU32,
}
const _: () = assert!(core::mem::size_of::<BSIntrusiveRefCounted>() == 0x4);

pub trait BSIntrusiveRefCountedTrait {
    fn inc_ref(&self) -> u32;
    fn dec_ref(&self) -> u32;
}

impl BSIntrusiveRefCountedTrait for BSIntrusiveRefCounted {
    #[inline]
    fn inc_ref(&self) -> u32 {
        self.refCount.fetch_add(1, Ordering::AcqRel)
    }

    #[inline]
    fn dec_ref(&self) -> u32 {
        self.refCount.fetch_sub(1, Ordering::AcqRel)
    }
}
