#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DIALOGUE_TYPE {
    PlayerDialogue = 0,
    CommandDialogue = 1,

    SceneDialogue = 2,
    Combat = 3,
    Favors = 4,
    Detection = 5,
    Service = 6,
    Miscellaneous = 7,
}

impl DIALOGUE_TYPE {
    pub const BRANCHED_TOTAL: usize = 2;
}
