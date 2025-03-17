#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FormType {
    None = 0,                   //	00 NONE TESForm
    PluginInfo,                 //	01 TES4
    FormGroup,                  //	02 GRUP
    GameSetting,                //	03 GMST
    Keyword,                    //	04 KYWD	BGSKeyword
    LocationRefType,            //	05 LCRT	BGSLocationRefType
    Action,                     //	06 AACT	BGSAction
    TextureSet,                 //	07 TXST	BGSTextureSet
    MenuIcon,                   //	08 MICN	BGSMenuIcon
    Global,                     //	09 GLOB	TESGlobal
    Class,                      //	0A CLAS	TESClass
    Faction,                    //	0B FACT	TESFaction
    HeadPart,                   //	0C HDPT	BGSHeadPart
    Eyes,                       //	0D EYES	TESEyes
    Race,                       //	0E RACE	TESRace
    Sound,                      //	0F SOUN	TESSound
    AcousticSpace,              //	10 ASPC	BGSAcousticSpace
    Skill,                      //	11 SKIL
    MagicEffect,                //	12 MGEF	EffectSetting
    Script,                     //	13 SCPT	Script
    LandTexture,                //	14 LTEX	TESLandTexture
    Enchantment,                //	15 ENCH	EnchantmentItem
    Spell,                      //	16 SPEL	SpellItem
    Scroll,                     //	17 SCRL	ScrollItem
    Activator,                  //	18 ACTI	TESObjectACTI
    TalkingActivator,           //	19 TACT	BGSTalkingActivator
    Armor,                      //	1A ARMO	TESObjectARMO
    Book,                       //	1B BOOK	TESObjectBOOK
    Container,                  //	1C CONT	TESObjectCONT
    Door,                       //	1D DOOR	TESObjectDOOR
    Ingredient,                 //	1E INGR	IngredientItem
    Light,                      //	1F LIGH	TESObjectLIGH
    Misc,                       //	20 MISC TESObjectMISC
    Apparatus,                  //	21 APPA	BGSApparatus
    Static,                     //	22 STAT	TESObjectSTAT
    StaticCollection,           //	23 SCOL BGSStaticCollection
    MovableStatic,              //	24 MSTT	BGSMovableStatic
    Grass,                      //	25 GRAS	TESGrass
    Tree,                       //	26 TREE	TESObjectTREE
    Flora,                      //	27 FLOR	TESFlora
    Furniture,                  //	28 FURN	TESFurniture
    Weapon,                     //	29 WEAP	TESObjectWEAP
    Ammo,                       //	2A AMMO	TESAmmo
    NPC,                        //	2B NPC_	TESNPC
    LeveledNPC,                 //	2C LVLN	TESLevCharacter
    KeyMaster,                  //	2D KEYM	TESKey
    AlchemyItem,                //	2E ALCH	AlchemyItem
    IdleMarker,                 //	2F IDLM	BGSIdleMarker
    Note,                       //	30 NOTE	BGSNote
    ConstructibleObject,        //	31 COBJ	BGSConstructibleObject
    Projectile,                 //	32 PROJ	BGSProjectile
    Hazard,                     //	33 HAZD	BGSHazard
    SoulGem,                    //	34 SLGM	TESSoulGem
    LeveledItem,                //	35 LVLI	TESLevItem
    Weather,                    //	36 WTHR	TESWeather
    Climate,                    //	37 CLMT	TESClimate
    ShaderParticleGeometryData, //	38 SPGD	BGSShaderParticleGeometryData
    ReferenceEffect,            //	39 RFCT	BGSReferenceEffect
    Region,                     //	3A REGN	TESRegion
    Navigation,                 //	3B NAVI	NavMeshInfoMap
    Cell,                       //	3C CELL	TESObjectCELL
    Reference,                  //	3D REFR	TESObjectREFR
    ActorCharacter,             //	3E ACHR	Actor / Character / PlayerCharacter
    ProjectileMissile,          //	3F PMIS	MissileProjectile
    ProjectileArrow,            //	40 PARW	ArrowProjectile
    ProjectileGrenade,          //	41 PGRE	GrenadeProjectile
    ProjectileBeam,             //	42 PBEA	BeamProjectile
    ProjectileFlame,            //	43 PFLA	FlameProjectile
    ProjectileCone,             //	44 PCON	ConeProjectile
    ProjectileBarrier,          //	45 PBAR	BarrierProjectile
    PlacedHazard,               //	46 PHZD	Hazard
    WorldSpace,                 //	47 WRLD	TESWorldSpace
    Land,                       //	48 LAND	TESObjectLAND
    NavMesh,                    //	49 NAVM	NavMesh
    TLOD,                       //	4A TLOD
    Dialogue,                   //	4B DIAL	TESTopic
    Info,                       //	4C INFO	TESTopicInfo
    Quest,                      //	4D QUST	TESQuest
    Idle,                       //	4E IDLE	TESIdleForm
    Package,                    //	4F PACK	TESPackage / DialoguePackage
    CombatStyle,                //	50 CSTY	TESCombatStyle
    LoadScreen,                 //	51 LSCR	TESLoadScreen
    LeveledSpell,               //	52 LVSP	TESLevSpell
    AnimatedObject,             //	53 ANIO	TESObjectANIO
    Water,                      //	54 WATR	TESWaterForm
    EffectShader,               //	55 EFSH	TESEffectShader
    TOFT,                       //	56 TOFT
    Explosion,                  //	57 EXPL	BGSExplosion
    Debris,                     //	58 DEBR	BGSDebris
    ImageSpace,                 //	59 IMGS	TESImageSpace
    ImageAdapter,               //	5A IMAD	TESImageSpaceModifier
    FormList,                   //	5B FLST	BGSListForm
    Perk,                       //	5C PERK	BGSPerk
    BodyPartData,               //	5D BPTD	BGSBodyPartData
    AddonNode,                  //	5E ADDN	BGSAddonNode
    ActorValueInfo,             //	5F AVIF	ActorValueInfo
    CameraShot,                 //	60 CAMS	BGSCameraShot
    CameraPath,                 //	61 CPTH	BGSCameraPath
    VoiceType,                  //	62 VTYP	BGSVoiceType
    MaterialType,               //	63 MATT	BGSMaterialType
    Impact,                     //	64 IPCT	BGSImpactData
    ImpactDataSet,              //	65 IPDS	BGSImpactDataSet
    Armature,                   //	66 ARMA	TESObjectARMA
    EncounterZone,              //	67 ECZN	BGSEncounterZone
    Location,                   //	68 LCTN	BGSLocation
    Message,                    //	69 MESG	BGSMessage
    Ragdoll,                    //	6A RGDL	BGSRagdoll
    DefaultObject,              //	6B DOBJ BGSDefaultObjectManager
    LightingMaster,             //	6C LGTM	BGSLightingTemplate
    MusicType,                  //	6D MUSC	BGSMusicType
    Footstep,                   //	6E FSTP	BGSFootstep
    FootstepSet,                //	6F FSTS	BGSFootstepSet
    StoryManagerBranchNode,     //	70 SMBN	BGSStoryManagerBranchNode
    StoryManagerQuestNode,      //	71 SMQN	BGSStoryManagerQuestNode
    StoryManagerEventNode,      //	72 SMEN	BGSStoryManagerEventNode
    DialogueBranch,             //	73 DLBR	BGSDialogueBranch
    MusicTrack,                 //	74 MUST	BGSMusicTrackFormWrapper
    DialogueView,               //	75 DLVW
    WordOfPower,                //	76 WOOP	TESWordOfPower
    Shout,                      //	77 SHOU	TESShout
    EquipSlot,                  //	78 EQUP	BGSEquipSlot
    Relationship,               //	79 RELA	BGSRelationship
    Scene,                      //	7A SCEN	BGSScene
    AssociationType,            //	7B ASTP	BGSAssociationType
    Outfit,                     //	7C OTFT	BGSOutfit
    ArtObject,                  //	7D ARTO	BGSArtObject
    MaterialObject,             //	7E MATO	BGSMaterialObject
    MovementType,               //	7F MOVT	BGSMovementType
    SoundRecord,                //	80 SNDR	BGSSoundDescriptorForm
    DualCastData,               //	81 DUAL	BGSDualCastData
    SoundCategory,              //	82 SNCT	BGSSoundCategory
    SoundOutputModel,           //	83 SOPM	BGSSoundOutput
    CollisionLayer,             //	84 COLL	BGSCollisionLayer
    ColorForm,                  //	85 CLFM	BGSColorForm
    ReverbParam,                //	86 REVB	BGSReverbParameters
    LensFlare,                  //	87 LENS BGSLensFlare
    LensSprite,                 //	88 LSPR
    VolumetricLighting,         //	89 VOLI BGSVolumetricLighting

    Max, //	8A
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
