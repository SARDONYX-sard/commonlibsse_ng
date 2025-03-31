use crate::re::hkBaseTypes::{hkHalf, hkUFloat8};
use crate::re::hkSweptTransform::hkSweptTransform;
use crate::re::hkTransform::hkTransform;
use crate::re::hkVector4::hkVector4;

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct hkMotionState {
    pub transform: hkTransform,           // 0x00
    pub sweptTransform: hkSweptTransform, // 0x40
    pub deltaAngle: hkVector4,            // 0x90
    pub objectRadius: f32,                // 0xA0
    pub linearDamping: hkHalf,            // 0xA4
    pub angularDamping: hkHalf,           // 0xA6
    pub timeFactor: hkHalf,               // 0xA8
    pub maxLinearVelocity: hkUFloat8,     // 0xAA
    pub maxAngularVelocity: hkUFloat8,    // 0xAB
    pub deactivationClass: u8,            // 0xAC
    pub padAD: u8,                        // 0xAD
    pub padAE: [u8; 2],                   // 0xAE
}
