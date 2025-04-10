use crate::re::NiRefObject::{NiRefObject, NiRefObjectVtbl};
use crate::re::offsets_rtti::RTTI_BSHandleRefObject;
use crate::re::offsets_vtable::VTABLE_BSHandleRefObject;
use crate::rel::id::VariantID;

pub const REF_COUNT_MASK: u32 = 0x3ff;
pub const HANDLE_VALID: u32 = 1 << 10;

#[repr(C)]
#[derive(Debug)]
pub struct BSHandleRefObject {
    pub __base: NiRefObject,
}

impl BSHandleRefObject {
    pub const RTTI: VariantID = RTTI_BSHandleRefObject;
    pub const VTABLE: [VariantID; 1] = VTABLE_BSHandleRefObject;
}

impl crate::re::NiSmartPointer::RefCountable for BSHandleRefObject {
    #[inline]
    fn inc_ref_count(&self) {
        self.__base.inc_ref_count();
    }

    #[inline]
    fn dec_ref_count(&mut self) {
        self.__base.dec_ref_count();
    }
}

#[repr(C)]
pub struct BSHandleRefObjectVtbl {
    pub __base: NiRefObjectVtbl,
}
