#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum RecordFlag {
    Destructible = 1 << 0,
    Master = 1 << 0,
    Unlocked = 1 << 0,

    Altered = 1 << 1,
    Playable = 1 << 2,
    Initialized = 1 << 3,
    Nonoccluder = 1 << 4,
    Deleted = 1 << 5,

    BorderRegion = 1 << 6,
    GlobalConstant = 1 << 6,
    HasSpokenFlag = 1 << 6,
    Known = 1 << 6,
    InPlaceableWater = 1 << 6,

    FireOff = 1 << 7,
    MustUpdate = 1 << 8,
    OnLocalMap = 1 << 9,
    Persistent = 1 << 10,

    Disabled = 1 << 11,
    UsedAsMovingPlatform = 1 << 11,

    Ignored = 1 << 12,

    Empty = 1 << 13,
    ResetDestruction = 1 << 13,

    Temporary = 1 << 14,
    MustBeVisibleDistant = 1 << 15,
    RandomAnim = 1 << 16,
    Dangerous = 1 << 17,
    HasCurrents = 1 << 19,
    IgnoreFriendlyHits = 1 << 20,
    StillLoading = 1 << 21,
    FormRetainsID = 1 << 22,
    Destroyed = 1 << 23,

    Unk24 = 1 << 24,

    NoAIAcquire = 1 << 25,
    Obstacle = 1 << 25,

    VATSTargetOverride = 1 << 26,
    DisableFade = 1 << 27,

    ReflectedByAutoWater = 1 << 28,
    ShowOnWorldMap = 1 << 28,

    ChildCanUse = 1 << 2,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum InGameFormFlag {
    #[default]
    None = 0,
    WantsDelete = 1 << 0,
    ForcedPersistent = 1 << 1,
    NoFavorAllowed = 1 << 4,
    IsSkyObject = 1 << 5,
    RefOriginalPersistent = 1 << 6,
    RefPermanentlyDeleted = 1 << 7,
}
