/// Represents the `BSTimer` class from C++.
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BSTimer {
    pub unk00: u64,
    pub lastPerformanceCount: u32,
    pub clamp: f32,
    pub clampRemainder: f32,
    pub delta: f32,
    pub realTimeDelta: f32,
    pub unk1C: u32,
    pub unk20: u32,
    pub unk24: f32,
    pub unk28: u32,
    pub unk2C: u32,
    pub unk30: u32,
    pub unk34: u8,
    pub unk35: u8,
    pub useGlobalTimeMultiplierTarget: bool,
    pub pad37: u8,
    pub pad38: u32,

    pub pad3C: u32,
}

// Ensure the memory layout matches the C++ struct
const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(BSTimer, unk00) == 0x00);
    assert!(offset_of!(BSTimer, lastPerformanceCount) == 0x08);
    assert!(offset_of!(BSTimer, clamp) == 0xc);
    assert!(offset_of!(BSTimer, clampRemainder) == 0x10);
    assert!(offset_of!(BSTimer, delta) == 0x14);
    assert!(offset_of!(BSTimer, realTimeDelta) == 0x18);
    assert!(offset_of!(BSTimer, unk1C) == 0x1c);
    assert!(offset_of!(BSTimer, unk20) == 0x20);
    assert!(offset_of!(BSTimer, unk24) == 0x24);
    assert!(offset_of!(BSTimer, unk28) == 0x28);
    assert!(offset_of!(BSTimer, unk2C) == 0x2c);
    assert!(offset_of!(BSTimer, unk30) == 0x30);
    assert!(offset_of!(BSTimer, unk34) == 0x34);
    assert!(offset_of!(BSTimer, unk35) == 0x35);
    assert!(offset_of!(BSTimer, useGlobalTimeMultiplierTarget) == 0x36);
    assert!(offset_of!(BSTimer, pad37) == 0x37);
    assert!(offset_of!(BSTimer, pad38) == 0x38);
    assert!(offset_of!(BSTimer, pad3C) == 0x3c);

    assert!(core::mem::size_of::<BSTimer>() == 0x40);
};

impl BSTimer {
    /// Gets the singleton instance of `BSTimer`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut BSTimer",
        default = "None",
        id(se = 523657, ae = 410196)
    )]
    #[inline]
    pub fn get_singleton() -> Option<&'static BSTimer> {
        |as_type: AsType| unsafe { as_type.as_ref() }
    }

    /// Gets the global time multiplier.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut f32",
        default = "None",
        id(se = 511882, ae = 388442)
    )]
    #[inline]
    pub fn q_global_time_multiplier() -> Option<f32> {
        |as_type: AsType| unsafe { as_type.as_ref().copied() }
    }

    /// Gets the target global time multiplier.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut f32",
        default = "None",
        id(se = 511883, ae = 388443)
    )]
    #[inline]
    pub fn q_global_time_multiplier_target() -> Option<f32> {
        |as_type: AsType| unsafe { as_type.as_ref().copied() }
    }

    /// Sets the global time multiplier.
    #[inline]
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 66988, ae_id = 68245)]
    pub fn set_global_time_multiplier(&mut self, multiplier: f32, arg2: bool) {}
}
