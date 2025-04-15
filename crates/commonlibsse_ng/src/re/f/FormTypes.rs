#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FormType {
    None = 0,                   // 0x00 NONE TESForm
    PluginInfo,                 // 0x01 TES4
    FormGroup,                  // 0x02 GRUP
    GameSetting,                // 0x03 GMST
    Keyword,                    // 0x04 KYWD	BGSKeyword
    LocationRefType,            // 0x05 LCRT	BGSLocationRefType
    Action,                     // 0x06 AACT	BGSAction
    TextureSet,                 // 0x07 TXST	BGSTextureSet
    MenuIcon,                   // 0x08 MICN	BGSMenuIcon
    Global,                     // 0x09 GLOB	TESGlobal
    Class,                      // 0x0A CLAS	TESClass
    Faction,                    // 0x0B FACT	TESFaction
    HeadPart,                   // 0x0C HDPT	BGSHeadPart
    Eyes,                       // 0x0D EYES	TESEyes
    Race,                       // 0x0E RACE	TESRace
    Sound,                      // 0x0F SOUN	TESSound
    AcousticSpace,              // 0x10 ASPC	BGSAcousticSpace
    Skill,                      // 0x11 SKIL
    MagicEffect,                // 0x12 MGEF	EffectSetting
    Script,                     // 0x13 SCPT	Script
    LandTexture,                // 0x14 LTEX	TESLandTexture
    Enchantment,                // 0x15 ENCH	EnchantmentItem
    Spell,                      // 0x16 SPEL	SpellItem
    Scroll,                     // 0x17 SCRL	ScrollItem
    Activator,                  // 0x18 ACTI	TESObjectACTI
    TalkingActivator,           // 0x19 TACT	BGSTalkingActivator
    Armor,                      // 0x1A ARMO	TESObjectARMO
    Book,                       // 0x1B BOOK	TESObjectBOOK
    Container,                  // 0x1C CONT	TESObjectCONT
    Door,                       // 0x1D DOOR	TESObjectDOOR
    Ingredient,                 // 0x1E INGR	IngredientItem
    Light,                      // 0x1F LIGH	TESObjectLIGH
    Misc,                       // 0x20 MISC TESObjectMISC
    Apparatus,                  // 0x21 APPA	BGSApparatus
    Static,                     // 0x22 STAT	TESObjectSTAT
    StaticCollection,           // 0x23 SCOL BGSStaticCollection
    MovableStatic,              // 0x24 MSTT	BGSMovableStatic
    Grass,                      // 0x25 GRAS	TESGrass
    Tree,                       // 0x26 TREE	TESObjectTREE
    Flora,                      // 0x27 FLOR	TESFlora
    Furniture,                  // 0x28 FURN	TESFurniture
    Weapon,                     // 0x29 WEAP	TESObjectWEAP
    Ammo,                       // 0x2A AMMO	TESAmmo
    NPC,                        // 0x2B NPC_	TESNPC
    LeveledNPC,                 // 0x2C LVLN	TESLevCharacter
    KeyMaster,                  // 0x2D KEYM	TESKey
    AlchemyItem,                // 0x2E ALCH	AlchemyItem
    IdleMarker,                 // 0x2F IDLM	BGSIdleMarker
    Note,                       // 0x30 NOTE	BGSNote
    ConstructibleObject,        // 0x31 COBJ	BGSConstructibleObject
    Projectile,                 // 0x32 PROJ	BGSProjectile
    Hazard,                     // 0x33 HAZD	BGSHazard
    SoulGem,                    // 0x34 SLGM	TESSoulGem
    LeveledItem,                // 0x35 LVLI	TESLevItem
    Weather,                    // 0x36 WTHR	TESWeather
    Climate,                    // 0x37 CLMT	TESClimate
    ShaderParticleGeometryData, // 0x38 SPGD	BGSShaderParticleGeometryData
    ReferenceEffect,            // 0x39 RFCT	BGSReferenceEffect
    Region,                     // 0x3A REGN	TESRegion
    Navigation,                 // 0x3B NAVI	NavMeshInfoMap
    Cell,                       // 0x3C CELL	TESObjectCELL
    Reference,                  // 0x3D REFR	TESObjectREFR
    ActorCharacter,             // 0x3E ACHR	Actor / Character / PlayerCharacter
    ProjectileMissile,          // 0x3F PMIS	MissileProjectile
    ProjectileArrow,            // 0x40 PARW	ArrowProjectile
    ProjectileGrenade,          // 0x41 PGRE	GrenadeProjectile
    ProjectileBeam,             // 0x42 PBEA	BeamProjectile
    ProjectileFlame,            // 0x43 PFLA	FlameProjectile
    ProjectileCone,             // 0x44 PCON	ConeProjectile
    ProjectileBarrier,          // 0x45 PBAR	BarrierProjectile
    PlacedHazard,               // 0x46 PHZD	Hazard
    WorldSpace,                 // 0x47 WRLD	TESWorldSpace
    Land,                       // 0x48 LAND	TESObjectLAND
    NavMesh,                    // 0x49 NAVM	NavMesh
    TLOD,                       // 0x4A TLOD
    Dialogue,                   // 0x4B DIAL	TESTopic
    Info,                       // 0x4C INFO	TESTopicInfo
    Quest,                      // 0x4D QUST	TESQuest
    Idle,                       // 0x4E IDLE	TESIdleForm
    Package,                    // 0x4F PACK	TESPackage / DialoguePackage
    CombatStyle,                // 0x50 CSTY	TESCombatStyle
    LoadScreen,                 // 0x51 LSCR	TESLoadScreen
    LeveledSpell,               // 0x52 LVSP	TESLevSpell
    AnimatedObject,             // 0x53 ANIO	TESObjectANIO
    Water,                      // 0x54 WATR	TESWaterForm
    EffectShader,               // 0x55 EFSH	TESEffectShader
    TOFT,                       // 0x56 TOFT
    Explosion,                  // 0x57 EXPL	BGSExplosion
    Debris,                     // 0x58 DEBR	BGSDebris
    ImageSpace,                 // 0x59 IMGS	TESImageSpace
    ImageAdapter,               // 0x5A IMAD	TESImageSpaceModifier
    FormList,                   // 0x5B FLST	BGSListForm
    Perk,                       // 0x5C PERK	BGSPerk
    BodyPartData,               // 0x5D BPTD	BGSBodyPartData
    AddonNode,                  // 0x5E ADDN	BGSAddonNode
    ActorValueInfo,             // 0x5F AVIF	ActorValueInfo
    CameraShot,                 // 0x60 CAMS	BGSCameraShot
    CameraPath,                 // 0x61 CPTH	BGSCameraPath
    VoiceType,                  // 0x62 VTYP	BGSVoiceType
    MaterialType,               // 0x63 MATT	BGSMaterialType
    Impact,                     // 0x64 IPCT	BGSImpactData
    ImpactDataSet,              // 0x65 IPDS	BGSImpactDataSet
    Armature,                   // 0x66 ARMA	TESObjectARMA
    EncounterZone,              // 0x67 ECZN	BGSEncounterZone
    Location,                   // 0x68 LCTN	BGSLocation
    Message,                    // 0x69 MESG	BGSMessage
    Ragdoll,                    // 0x6A RGDL	BGSRagdoll
    DefaultObject,              // 0x6B DOBJ BGSDefaultObjectManager
    LightingMaster,             // 0x6C LGTM	BGSLightingTemplate
    MusicType,                  // 0x6D MUSC	BGSMusicType
    Footstep,                   // 0x6E FSTP	BGSFootstep
    FootstepSet,                // 0x6F FSTS	BGSFootstepSet
    StoryManagerBranchNode,     // 0x70 SMBN	BGSStoryManagerBranchNode
    StoryManagerQuestNode,      // 0x71 SMQN	BGSStoryManagerQuestNode
    StoryManagerEventNode,      // 0x72 SMEN	BGSStoryManagerEventNode
    DialogueBranch,             // 0x73 DLBR	BGSDialogueBranch
    MusicTrack,                 // 0x74 MUST	BGSMusicTrackFormWrapper
    DialogueView,               // 0x75 DLVW
    WordOfPower,                // 0x76 WOOP	TESWordOfPower
    Shout,                      // 0x77 SHOU	TESShout
    EquipSlot,                  // 0x78 EQUP	BGSEquipSlot
    Relationship,               // 0x79 RELA	BGSRelationship
    Scene,                      // 0x7A SCEN	BGSScene
    AssociationType,            // 0x7B ASTP	BGSAssociationType
    Outfit,                     // 0x7C OTFT	BGSOutfit
    ArtObject,                  // 0x7D ARTO	BGSArtObject
    MaterialObject,             // 0x7E MATO	BGSMaterialObject
    MovementType,               // 0x7F MOVT	BGSMovementType
    SoundRecord,                // 0x80 SNDR	BGSSoundDescriptorForm
    DualCastData,               // 0x81 DUAL	BGSDualCastData
    SoundCategory,              // 0x82 SNCT	BGSSoundCategory
    SoundOutputModel,           // 0x83 SOPM	BGSSoundOutput
    CollisionLayer,             // 0x84 COLL	BGSCollisionLayer
    ColorForm,                  // 0x85 CLFM	BGSColorForm
    ReverbParam,                // 0x86 REVB	BGSReverbParameters
    LensFlare,                  // 0x87 LENS BGSLensFlare
    LensSprite,                 // 0x88 LSPR
    VolumetricLighting,         // 0x89 VOLI BGSVolumetricLighting

    Max, //	0x8A
}

impl FormType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::PluginInfo => "TES4",
            Self::FormGroup => "GRUP",
            Self::GameSetting => "GMST",
            Self::Keyword => "KYWD",
            Self::LocationRefType => "LCRT",
            Self::Action => "AACT",
            Self::TextureSet => "TXST",
            Self::MenuIcon => "MICN",
            Self::Global => "GLOB",
            Self::Class => "CLAS",
            Self::Faction => "FACT",
            Self::HeadPart => "HDPT",
            Self::Eyes => "EYES",
            Self::Race => "RACE",
            Self::Sound => "SOUN",
            Self::AcousticSpace => "ASPC",
            Self::Skill => "SKIL",
            Self::MagicEffect => "MGEF",
            Self::Script => "SCPT",
            Self::LandTexture => "LTEX",
            Self::Enchantment => "ENCH",
            Self::Spell => "SPEL",
            Self::Scroll => "SCRL",
            Self::Activator => "ACTI",
            Self::TalkingActivator => "TACT",
            Self::Armor => "ARMO",
            Self::Book => "BOOK",
            Self::Container => "CONT",
            Self::Door => "DOOR",
            Self::Ingredient => "INGR",
            Self::Light => "LIGH",
            Self::Misc => "MISC",
            Self::Apparatus => "APPA",
            Self::Static => "STAT",
            Self::StaticCollection => "SCOL",
            Self::MovableStatic => "MSTT",
            Self::Grass => "GRAS",
            Self::Tree => "TREE",
            Self::Flora => "FLOR",
            Self::Furniture => "FURN",
            Self::Weapon => "WEAP",
            Self::Ammo => "AMMO",
            Self::NPC => "NPC_",
            Self::LeveledNPC => "LVLN",
            Self::KeyMaster => "KEYM",
            Self::AlchemyItem => "ALCH",
            Self::IdleMarker => "IDLM",
            Self::Note => "NOTE",
            Self::ConstructibleObject => "COBJ",
            Self::Projectile => "PROJ",
            Self::Hazard => "HAZD",
            Self::SoulGem => "SLGM",
            Self::LeveledItem => "LVLI",
            Self::Weather => "WTHR",
            Self::Climate => "CLMT",
            Self::ShaderParticleGeometryData => "SPGD",
            Self::ReferenceEffect => "RFCT",
            Self::Region => "REGN",
            Self::Navigation => "NAVI",
            Self::Cell => "CELL",
            Self::Reference => "REFR",
            Self::ActorCharacter => "ACHR",
            Self::ProjectileMissile => "PMIS",
            Self::ProjectileArrow => "PARW",
            Self::ProjectileGrenade => "PGRE",
            Self::ProjectileBeam => "PBEA",
            Self::ProjectileFlame => "PFLA",
            Self::ProjectileCone => "PCON",
            Self::ProjectileBarrier => "PBAR",
            Self::PlacedHazard => "PHZD",
            Self::WorldSpace => "WRLD",
            Self::Land => "LAND",
            Self::NavMesh => "NAVM",
            Self::TLOD => "TLOD",
            Self::Dialogue => "DIAL",
            Self::Info => "INFO",
            Self::Quest => "QUST",
            Self::Idle => "IDLE",
            Self::Package => "PACK",
            Self::CombatStyle => "CSTY",
            Self::LoadScreen => "LSCR",
            Self::LeveledSpell => "LVSP",
            Self::AnimatedObject => "ANIO",
            Self::Water => "WATR",
            Self::EffectShader => "EFSH",
            Self::TOFT => "TOFT",
            Self::Explosion => "EXPL",
            Self::Debris => "DEBR",
            Self::ImageSpace => "IMGS",
            Self::ImageAdapter => "IMAD",
            Self::FormList => "FLST",
            Self::Perk => "PERK",
            Self::BodyPartData => "BPTD",
            Self::AddonNode => "ADDN",
            Self::ActorValueInfo => "AVIF",
            Self::CameraShot => "CAMS",
            Self::CameraPath => "CPTH",
            Self::VoiceType => "VTYP",
            Self::MaterialType => "MATT",
            Self::Impact => "IPCT",
            Self::ImpactDataSet => "IPDS",
            Self::Armature => "ARMA",
            Self::EncounterZone => "ECZN",
            Self::Location => "LCTN",
            Self::Message => "MESG",
            Self::Ragdoll => "RGDL",
            Self::DefaultObject => "DOBJ",
            Self::LightingMaster => "LGTM",
            Self::MusicType => "MUSC",
            Self::Footstep => "FSTP",
            Self::FootstepSet => "FSTS",
            Self::StoryManagerBranchNode => "SMBN",
            Self::StoryManagerQuestNode => "SMQN",
            Self::StoryManagerEventNode => "SMEN",
            Self::DialogueBranch => "DLBR",
            Self::MusicTrack => "MUST",
            Self::DialogueView => "DLVW",
            Self::WordOfPower => "WOOP",
            Self::Shout => "SHOU",
            Self::EquipSlot => "EQUP",
            Self::Relationship => "RELA",
            Self::Scene => "SCEN",
            Self::AssociationType => "ASTP",
            Self::Outfit => "OTFT",
            Self::ArtObject => "ARTO",
            Self::MaterialObject => "MATO",
            Self::MovementType => "MOVT",
            Self::SoundRecord => "SNDR",
            Self::DualCastData => "DUAL",
            Self::SoundCategory => "SNCT",
            Self::SoundOutputModel => "SOPM",
            Self::CollisionLayer => "COLL",
            Self::ColorForm => "CLFM",
            Self::ReverbParam => "REVB",
            Self::LensFlare => "LENS",
            Self::LensSprite => "LSPR",
            Self::VolumetricLighting => "VOLI",
            _ => "NONE",
        }
    }
}

impl core::str::FromStr for FormType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "TES4" => Self::PluginInfo,
            "GRUP" => Self::FormGroup,
            "GMST" => Self::GameSetting,
            "KYWD" => Self::Keyword,
            "LCRT" => Self::LocationRefType,
            "AACT" => Self::Action,
            "TXST" => Self::TextureSet,
            "MICN" => Self::MenuIcon,
            "GLOB" => Self::Global,
            "CLAS" => Self::Class,
            "FACT" => Self::Faction,
            "HDPT" => Self::HeadPart,
            "EYES" => Self::Eyes,
            "RACE" => Self::Race,
            "SOUN" => Self::Sound,
            "ASPC" => Self::AcousticSpace,
            "SKIL" => Self::Skill,
            "MGEF" => Self::MagicEffect,
            "SCPT" => Self::Script,
            "LTEX" => Self::LandTexture,
            "ENCH" => Self::Enchantment,
            "SPEL" => Self::Spell,
            "SCRL" => Self::Scroll,
            "ACTI" => Self::Activator,
            "TACT" => Self::TalkingActivator,
            "ARMO" => Self::Armor,
            "BOOK" => Self::Book,
            "CONT" => Self::Container,
            "DOOR" => Self::Door,
            "INGR" => Self::Ingredient,
            "LIGH" => Self::Light,
            "MISC" => Self::Misc,
            "APPA" => Self::Apparatus,
            "STAT" => Self::Static,
            "SCOL" => Self::StaticCollection,
            "MSTT" => Self::MovableStatic,
            "GRAS" => Self::Grass,
            "TREE" => Self::Tree,
            "FLOR" => Self::Flora,
            "FURN" => Self::Furniture,
            "WEAP" => Self::Weapon,
            "AMMO" => Self::Ammo,
            "NPC_" => Self::NPC,
            "LVLN" => Self::LeveledNPC,
            "KEYM" => Self::KeyMaster,
            "ALCH" => Self::AlchemyItem,
            "IDLM" => Self::IdleMarker,
            "NOTE" => Self::Note,
            "COBJ" => Self::ConstructibleObject,
            "PROJ" => Self::Projectile,
            "HAZD" => Self::Hazard,
            "SLGM" => Self::SoulGem,
            "LVLI" => Self::LeveledItem,
            "WTHR" => Self::Weather,
            "CLMT" => Self::Climate,
            "SPGD" => Self::ShaderParticleGeometryData,
            "RFCT" => Self::ReferenceEffect,
            "REGN" => Self::Region,
            "NAVI" => Self::Navigation,
            "CELL" => Self::Cell,
            "REFR" => Self::Reference,
            "ACHR" => Self::ActorCharacter,
            "NONE" => Self::None,
            invalid => return Err(format!("Invalid FormType: {invalid}")),
        })
    }
}
