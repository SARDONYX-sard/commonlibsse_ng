use crate::re::hkQuaternion::hkQuaternion;
use crate::re::hkVector4::hkVector4;

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct hkSweptTransform {
    pub centerOfMass0: hkVector4,     // 0x00
    pub centerOfMass1: hkVector4,     // 0x10
    pub rotation0: hkQuaternion,      // 0x20
    pub rotation1: hkQuaternion,      // 0x30
    pub centerOfMassLocal: hkVector4, // 0x40
}
const _: () = assert!(std::mem::size_of::<hkSweptTransform>() == 0x50);
