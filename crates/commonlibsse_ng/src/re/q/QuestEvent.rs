#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum QuestEvent {
    None = 4294967295,           // NONE - u32::MAX
    KillActor = 0,               // KILL
    AssaultActor = 1,            // ASSU
    ChangeLocation = 2,          // CLOC
    Script = 3,                  // SCPT
    ActorDialogue = 4,           // ADIA
    ActorHello = 5,              // AHEL
    ActivateActor = 6,           // AFAV
    PlayerAddItem = 7,           // AIPL
    PlayerRemoveItem = 8,        // REMP
    CraftItem = 9,               // CRFT
    PickLock = 10,               // LOCK
    Infection = 11,              // INFC
    Cure = 12,                   // CURE
    NewVoicePower = 13,          // NVPE
    DeadBody = 14,               // DEAD
    SkillIncrease = 15,          // SKIL
    IncreaseLevel = 16,          // LEVL
    ChangeRelationshipRank = 17, // CHBR
    IntimidateNPC = 18,          // INTM
    BribeNPC = 19,               // BRIB
    FlatterNPC = 20,             // FLAT
    PlayerGetsFavor = 21,        // PRFV
    PayFine = 22,                // PFIN
    Jail = 23,                   // JAIL
    ServedTime = 24,             // STIJ
    EscapeJail = 25,             // ESIA
    Trespass = 26,               // TRES
    CrimeGold = 27,              // ADCR
    Arrest = 28,                 // ARRT
    CastMagic = 29,              // CAST
}
