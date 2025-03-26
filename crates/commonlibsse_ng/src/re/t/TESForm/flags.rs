bitflags::bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RecordFlag : u32 {
        const DESTRUCTIBLE = 1 << 0;
        const MASTER = 1 << 0;
        const UNLOCKED = 1 << 0;

        const ALTERED = 1 << 1;
        const PLAYABLE = 1 << 2;
        const INITIALIZED = 1 << 3;
        const NONOCCLUDER = 1 << 4;
        const DELETED = 1 << 5;

        const BORDER_REGION = 1 << 6;
        const GLOBAL_CONSTANT = 1 << 6;
        const HAS_SPOKEN_FLAG = 1 << 6;
        const KNOWN = 1 << 6;
        const IN_PLACEABLE_WATER = 1 << 6;

        const FireOff = 1 << 7;
        const MustUpdate = 1 << 8;
        const OnLocalMap = 1 << 9;
        const Persistent = 1 << 10;

        const Disabled = 1 << 11;
        const UsedAsMovingPlatform = 1 << 11;

        const Ignored = 1 << 12;

        const Empty = 1 << 13;
        const ResetDestruction = 1 << 13;

        const Temporary = 1 << 14;
        const MustBeVisibleDistant = 1 << 15;
        const RandomAnim = 1 << 16;
        const Dangerous = 1 << 17;
        const HasCurrents = 1 << 19;
        const IgnoreFriendlyHits = 1 << 20;
        const StillLoading = 1 << 21;
        const FormRetainsID = 1 << 22;
        const Destroyed = 1 << 23;

        const Unk24 = 1 << 24;

        const NoAIAcquire = 1 << 25;
        const Obstacle = 1 << 25;

        const VATSTargetOverride = 1 << 26;
        const DisableFade = 1 << 27;

        const ReflectedByAutoWater = 1 << 28;
        const ShowOnWorldMap = 1 << 28;

        const ChildCanUse = 1 << 2;
    }

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct InGameFormFlag: u16 {
        const NONE = 0;
        const WANTS_DELETE = 1 << 0;
        const FORCED_PERSISTENT = 1 << 1;
        const NO_FAVOR_ALLOWED = 1 << 4;
        const IS_SKY_OBJECT = 1 << 5;
        const REF_ORIGINAL_PERSISTENT = 1 << 6;
        const REF_PERMANENTLY_DELETED = 1 << 7;
    }
}
