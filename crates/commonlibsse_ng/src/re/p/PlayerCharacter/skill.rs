#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerSkills {
    data: *mut Data,
}
const _: () = assert!(core::mem::size_of::<PlayerSkills>() == 0x8);

/// Skill data
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Data {
    pub xp: f32,
    pub levelThreshold: f32,
    pub skills: [SkillData; Skill::TOTAL],
    pub legendaryLevels: [u32; Skill::TOTAL],
}
const _: () = assert!(core::mem::size_of::<Data>() == 0x128);

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SkillData {
    pub level: f32,          // 0x00
    pub xp: f32,             // 0x04
    pub levelThreshold: f32, // 0x08
}
const _: () = {
    assert!(core::mem::offset_of!(SkillData, level) == 0x0);
    assert!(core::mem::offset_of!(SkillData, xp) == 0x4);
    assert!(core::mem::offset_of!(SkillData, levelThreshold) == 0x8);
    assert!(core::mem::size_of::<SkillData>() == 0xC);
};

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Skill {
    #[default]
    OneHanded = 0,
    TwoHanded = 1,
    Archery = 2,
    Block = 3,
    Smithing = 4,
    HeavyArmor = 5,
    LightArmor = 6,
    Pickpocket = 7,
    Lockpicking = 8,
    Sneak = 9,
    Alchemy = 10,
    Speech = 11,
    Alteration = 12,
    Conjuration = 13,
    Destruction = 14,
    Illusion = 15,
    Restoration = 16,
    Enchanting = 17,
}

impl Skill {
    pub const TOTAL: usize = 18;

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 40560, ae_id = 41567)]
    pub fn advance_level(&mut self, add_threshold: bool) {}
}
