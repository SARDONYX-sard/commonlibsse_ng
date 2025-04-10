#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SlotTypes {
    LeftHand = 0,
    RightHand,
    Unknown,
    PowerOrShout,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BOOL_BITS {
    None = 0,
    DelayUpdateScenegraph = 1 << 0,
    ProcessMe = 1 << 1,
    MurderAlarm = 1 << 2,
    HasSceneExtra = 1 << 3,
    HeadingFixed = 1 << 4,
    SpeakingDone = 1 << 5,
    IgnoreChangeAnimationCall = 1 << 6,
    SoundFileDone = 1 << 7,
    VoiceFileDone = 1 << 8,
    InTempChangeList = 1 << 9,
    DoNotRunSayToCallback = 1 << 10,
    Dead = 1 << 11,
    ForceGreetingPlayer = 1 << 12,
    ForceUpdateQuestTarget = 1 << 13,
    SearchingInCombat = 1 << 14,
    AttackOnNextTheft = 1 << 15,
    EvpBuffered = 1 << 16,
    ResetAI = 1 << 17,
    InWater = 1 << 18,
    Swimming = 1 << 19,
    VoicePausedByScript = 1 << 20,
    WasInFrustrum = 1 << 21,
    ShouldRotateToTrack = 1 << 22,
    SetOnDeath = 1 << 23,
    DoNotPadVoice = 1 << 24,
    FootIKInRange = 1 << 25,
    PlayerTeammate = 1 << 26,
    GivePlayerXP = 1 << 27,
    SoundCallbackSuccess = 1 << 28,
    UseEmotion = 1 << 29,
    Guard = 1 << 30,
    Paralyzed = 1 << 31,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BOOL_FLAGS {
    None = 0,
    ScenePackage = 1 << 0,
    IsAMount = 1 << 1,
    MountPointClear = 1 << 2,
    GettingOnOffMount = 1 << 3,
    InRandomScene = 1 << 4,
    NoBleedoutRecovery = 1 << 5,
    InBleedoutAnimation = 1 << 6,
    CanDoFavor = 1 << 7,
    ShouldAnimGraphUpdate = 1 << 8,
    CanSpeakToEssentialDown = 1 << 9,
    BribedByPlayer = 1 << 10,
    AngryWithPlayer = 1 << 11,
    IsTrespassing = 1 << 12,
    CanSpeak = 1 << 13,
    IsInKillMove = 1 << 14,
    AttackOnSight = 1 << 15,
    IsCommandedActor = 1 << 16,
    ForceOneAnimgraphUpdate = 1 << 17,
    Essential = 1 << 18,
    Protected = 1 << 19,
    AttackingDisabled = 1 << 20,
    CastingDisabled = 1 << 21,
    SceneHeadTrackRotation = 1 << 22,
    ForceIncMinBoneUpdate = 1 << 23,
    CrimeSearch = 1 << 24,
    MovingIntoLoadedArea = 1 << 25,
    DoNotShowOnStealthMeter = 1 << 26,
    MovementBlocked = 1 << 27,
    AllowInstantFurniturePopInPlayerCell = 1 << 28,
    ForceAnimGraphUpdate = 1 << 29,
    CheckAddEffectDualCast = 1 << 30,
    Underwater = 1 << 31,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum ChangeFlag {
    LifeState = 1 << 10,
    PackageExtraData = 1 << 11,
    MerchantContainer = 1 << 12,
    DismemberedLimbs = 1 << 17,
    LeveledActor = 1 << 18,
    DispModifiers = 1 << 19,
    TempModifiers = 1 << 20,
    DamageModifiers = 1 << 21,
    OverrideModifiers = 1 << 22,
    PermanentModifiers = 1 << 23,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum RecordFlag {
    Deleted = 1 << 5,
    StartsDead = 1 << 9,
    Persistent = 1 << 10,
    InitiallyDisabled = 1 << 11,
    Ignored = 1 << 12,
    NoAIAcquire = 1 << 25,
    DontHavokSettle = 1 << 29,
}
