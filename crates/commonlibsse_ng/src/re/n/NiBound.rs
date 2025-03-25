use crate::re::NiPoint3::NiPoint3;

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct NiBound {
    pub center: NiPoint3,
    pub radius: f32,
}

const _: () = {
    assert!(core::mem::offset_of!(NiBound, center) == 0x0);
    assert!(core::mem::offset_of!(NiBound, radius) == 0xc);
    assert!(core::mem::size_of::<NiBound>() == 0x10);
};
