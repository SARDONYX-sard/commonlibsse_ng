use crate::re::hkRefPtr::hkRefPtr;
use crate::re::hkReferencedObject::hkReferencedObject;

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct hkRefVariant {
    pub __base: hkRefPtr<hkReferencedObject>,
}

impl crate::re::hkRefPtr::hkRefPtrCounted for hkRefVariant {}
