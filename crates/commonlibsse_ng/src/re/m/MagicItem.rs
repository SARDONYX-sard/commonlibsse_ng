use crate::re::Actor::Actor;
use crate::re::ActorValues::{ActorValue, ActorValue_CEnum};
use crate::re::BGSKeywordForm::{BGSKeywordForm, BGSKeywordFormVtbl};
use crate::re::BSTArray::BSTArray;
use crate::re::BSTSmartPointer::BSTSmartPointer;
use crate::re::TESBoundObject::{TESBoundObject, TESBoundObjectVtbl};
use crate::re::TESFile::TESFile;
use crate::re::TESFullName::TESFullName;
use crate::re::TESModel::TESModel;
use crate::re::TESObjectWEAP;
use crate::re::offsets_rtti::RTTI_MagicItem;
use crate::re::offsets_vtable::VTABLE_MagicItem;
use crate::re::{EffectItem, EffectSetting};
use crate::re::{MagicSystem, QueuedFile};
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct MagicItem {
    pub __base: TESBoundObject,                   // 0x00
    pub __base1: TESFullName,                     // 0x30
    pub __base2: BGSKeywordForm,                  // 0x40
    pub effects: BSTArray<*mut EffectItem>,       // 0x58
    pub hostileCount: i32,                        // 0x70
    pub pad74: u32,                               // 0x74
    pub avEffect: *mut EffectSetting,             // 0x78
    pub preloadCount: u32,                        // 0x80
    pub pad84: u32,                               // 0x84
    pub preloadItem: BSTSmartPointer<QueuedFile>, // 0x88
}
const _: () = assert!(core::mem::size_of::<MagicItem>() == 0x90);

impl MagicItem {
    pub const RTTI: VariantID = RTTI_MagicItem;
    pub const VTABLE: [VariantID; 3] = VTABLE_MagicItem;
}

#[repr(C)]
pub struct MagicItemVtbl {
    // Base class vtables (TESBoundObject, BGSKeywordForm) should come first
    pub __base: TESBoundObjectVtbl,
    pub __base1: BGSKeywordFormVtbl,

    // MagicItem virtual methods (start offset: 0x53 * 8 = 664)
    pub GetSpellType: extern "C" fn(this: *const MagicItem) -> MagicSystem::SpellType, // 0x53
    pub SetCastingType: extern "C" fn(this: *mut MagicItem, ty: MagicSystem::CastingType), // 0x54 - { return; }
    pub GetCastingType: extern "C" fn(this: *const MagicItem) -> MagicSystem::CastingType_CEnum, // 0x55
    pub SetDelivery: extern "C" fn(this: *mut MagicItem, delivery: MagicSystem::Delivery), // 0x56 - { return; }
    pub GetDelivery: extern "C" fn(this: *const MagicItem) -> MagicSystem::Delivery_CEnum, // 0x57
    pub IsValidDelivery:
        extern "C" fn(this: *const MagicItem, delivery: MagicSystem::Delivery) -> bool, // 0x58 - { return true; }
    pub GetFixedCastDuration: extern "C" fn(this: *const MagicItem) -> f32, // 0x59 - { return 0.0; }
    pub GetRange: extern "C" fn(this: *const MagicItem) -> f32, // 0x5A - { return 0.0; }
    pub IgnoresResistance: extern "C" fn(this: *const MagicItem) -> bool, // 0x5B - { return false; }
    pub IgnoreLOS: extern "C" fn(this: *const MagicItem) -> bool, // 0x5C - { return false; }
    pub IsFood: extern "C" fn(this: *const MagicItem) -> bool,    // 0x5D - { return false; }
    pub GetNoAbsorb: extern "C" fn(this: *const MagicItem) -> bool, // 0x5E - { return false; }
    pub GetNoDualCastModifications: extern "C" fn(this: *const MagicItem) -> bool, // 0x5F - { return false; }
    pub GetSkillUsageData: extern "C" fn(this: *const MagicItem, data: *mut SkillUsageData) -> bool, // 0x60 - { return false; }
    pub IsPoison: extern "C" fn(this: *const MagicItem) -> bool, // 0x61 - { return GetSpellType() == MagicSystem::SpellType::kPoison; }
    pub IsMedicine: extern "C" fn(this: *const MagicItem) -> bool, // 0x62 - { return false; }
    pub AdjustCost: extern "C" fn(this: *const MagicItem, cost: *mut f32, actor: *mut Actor), // 0x63 - { return; }
    pub GetChargeTime: extern "C" fn(this: *const MagicItem) -> f32, // 0x64 - { return 0.0; }
    pub GetMaxEffectCount: extern "C" fn(this: *const MagicItem) -> u32, // 0x65 - { return 0; }
    pub GetAssociatedSkill: extern "C" fn(this: *const MagicItem) -> ActorValue_CEnum, // 0x66 - { return ActorValue::kNone; }
    pub IsTwoHanded: extern "C" fn(this: *const MagicItem) -> bool, // 0x67 - { return false; }
    pub GetChunkID: extern "C" fn(this: *mut MagicItem) -> u32,     // 0x68
    pub CopyMagicItemData: extern "C" fn(this: *mut MagicItem, src: *const MagicItem), // 0x69
    pub LoadMagicItemChunk:
        extern "C" fn(this: *mut MagicItem, mod_file: *mut TESFile, chunk_id: u32), // 0x6A - { return; }
    pub LoadChunkDataPostProcess: extern "C" fn(this: *mut MagicItem, mod_file: *mut TESFile), // 0x6B - { return; }
    pub GetData1: extern "C" fn(this: *const MagicItem) -> *const Data, // 0x6C
    pub GetData2: extern "C" fn(this: *mut MagicItem) -> *mut Data,     // 0x6D
    pub GetDataSize: extern "C" fn(this: *const MagicItem) -> u32,      // 0x6E
    pub InitFromChunk: extern "C" fn(this: *mut MagicItem, mod_file: *mut TESFile), // 0x6F
    pub InitChunk: extern "C" fn(this: *mut MagicItem),                 // 0x70
}
const _: () = {
    const VTABLE_SIZE: usize = core::mem::size_of::<MagicItemVtbl>();
    const EXPECTED_SIZE: usize = 0x71 * core::mem::size_of::<usize>();
    // assert!(VTABLE_SIZE == EXPECTED_SIZE); // FIXME: pass assertion
};

#[repr(C)]
#[derive(Debug)]
pub struct PreloadableVisitor {
    vtable: *const PreloadableVisitorVtbl,
}
const _: () = assert!(core::mem::size_of::<PreloadableVisitor>() == 0x08);

#[repr(C)]
pub struct PreloadableVisitorVtbl {
    pub VisitModel: extern "C" fn(this: *mut PreloadableVisitor, model: *mut TESModel),
    pub VisitWeapon: extern "C" fn(this: *mut PreloadableVisitor, weapon: *mut TESObjectWEAP),
}

#[repr(C)]
#[derive(Debug)]
pub struct SkillUsageData {
    effect: *mut EffectItem, // 0x00
    skill: ActorValue,       // 0x08
    magnitude: f32,          // 0x0C
    custom: bool,            // 0x10
    pad11: u8,               // 0x11
    pad12: u16,              // 0x12
    pad14: u32,              // 0x14
}
const _: () = assert!(core::mem::size_of::<SkillUsageData>() == 0x18);

#[repr(C)]
#[derive(Debug)]
pub struct Data {
    costOverride: i32, // 0x0
    flags: u32,        // 0x4
}
const _: () = assert!(core::mem::size_of::<Data>() == 0x8);
