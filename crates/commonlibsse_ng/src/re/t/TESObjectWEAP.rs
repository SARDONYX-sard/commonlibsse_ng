// NOTE: The name `Flag` is not Reexported due to batting.
pub mod critical_data;
pub mod data;

use self::critical_data::CriticalData;
use self::data::Data;
use crate::re::BGSEquipType::BGSEquipType;
use crate::re::BGSKeywordForm::BGSKeywordForm;
use crate::re::BGSSoundDescriptorForm::BGSSoundDescriptorForm;
use crate::re::BSFixedString::BSFixedString;
use crate::re::SoundLevel::SOUND_LEVEL;
use crate::re::TESAttackDamageForm::TESAttackDamageForm;
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::TESDescription::TESDescription;
use crate::re::TESEnchantableForm::TESEnchantableForm;
use crate::re::TESFullName::TESFullName;
use crate::re::TESIcon::TESIcon;
use crate::re::TESModel::TESModel;
use crate::re::TESModelTextSwap::TESModelTextureSwap;
use crate::re::TESValueForm::TESValueForm;
use crate::re::TESWeightForm::TESWeightForm;

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum WEAPON_RUMBLE_PATTERN {
    Constant = 0,
    PeriodicSquare = 1,
    PeriodicTriangle = 2,
    PeriodicSawtooth = 3,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum RecordFlag {
    NonPlayable = 1 << 2,
    Deleted = 1 << 5,
    Ignored = 1 << 12,
}

#[repr(C)]
#[derive(Debug)]
pub struct RangedData {
    pub sightFOV: f32,                        // 0x00
    pub unk04: f32,                           // 0x04
    pub firingRumbleLeftMotorStrength: f32,   // 0x08
    pub firingRumbleRightMotorStrength: f32,  // 0x0C
    pub firingRumbleDuration: f32,            // 0x10
    pub rumblePattern: WEAPON_RUMBLE_PATTERN, // 0x14
    pub numProjectiles: i8,                   // 0x18
    pad19: u8,                                // 0x19
    pad1A: u16,                               // 0x1A
}
const _: () = assert!(core::mem::size_of::<RangedData>() == 0x1C);

#[repr(C)]
#[derive(Debug)]
pub struct Unk1B8 {
    pub unk00: TESModel,             // 00
    pub unk28: *mut TESEffectShader, // 28
}
const _: () = assert!(core::mem::size_of::<Unk1B8>() == 0x30);

#[repr(C)]
#[derive(Debug)]
pub struct TESObjectWEAP {
    pub __base0: TESBoundObject,                      // 0x000
    pub __base1: TESFullName,                         // 0x030
    pub __base2: TESModelTextureSwap,                 // 0x040
    pub __base3: TESIcon,                             // 0x078
    pub __base4: TESEnchantableForm,                  // 0x088
    pub __base5: TESValueForm,                        // 0x0A0
    pub __base6: TESWeightForm,                       // 0x0B0
    pub __base7: TESAttackDamageForm,                 // 0x0C0
    pub __base8: BGSDestructibleObjectForm,           // 0x0D0
    pub __base9: BGSEquipType,                        // 0x0E0
    pub __base10: BGSPreloadable,                     // 0x0F0
    pub __base11: BGSMessageIcon,                     // 0x0F8
    pub __base12: BGSPickupPutdownSounds,             // 0x110
    pub __base13: BGSBlockBashData,                   // 0x128
    pub __base14: BGSKeywordForm,                     // 0x140
    pub __base15: TESDescription,                     // 0x158
    pub weaponData: Data,                             // 0x168 - DNAM
    pub criticalData: CriticalData,                   // 0x1A0 - CRDT
    pub unk1B8: *mut Unk1B8,                          // 0x1B8
    pub attackSound: *mut BGSSoundDescriptorForm,     // 0x1C0 - SNAM
    pub attackSound2D: *mut BGSSoundDescriptorForm,   // 0x1C8 - XNAM
    pub attackLoopSound: *mut BGSSoundDescriptorForm, // 0x1D0 - NAM7
    pub attackFailSound: *mut BGSSoundDescriptorForm, // 0x1D8 - TNAM
    pub idleSound: *mut BGSSoundDescriptorForm,       // 0x1E0 - UNAM
    pub equipSound: *mut BGSSoundDescriptorForm,      // 0x1E8 - NAM9
    pub unequipSound: *mut BGSSoundDescriptorForm,    // 0x1F0 - NAM8
    pub impactDataSet: *mut BGSImpactDataSet,         // 0x1F8
    pub firstPersonModelObject: *mut TESObjectSTAT,   // 0x200 - WNAM
    pub templateWeapon: *mut TESObjectWEAP,           // 0x208 - CNAM
    pub embeddedNode: BSFixedString,                  // 0x210
    pub soundLevel: SOUND_LEVEL,                      // 0x218 - VNAM
    pub pad21C: u32,                                  // 0x21C
}
const _: () = assert!(core::mem::size_of::<TESObjectWEAP>() == 0x220);

#[repr(C)]
#[derive(Debug)]
pub struct BGSDestructibleObjectForm {
    _data: [u8; 0x10], // 0x0D0 - 0x0C0 = 0x10
}
const _: () = assert!(core::mem::size_of::<BGSDestructibleObjectForm>() == 0x10);

#[repr(C)]
#[derive(Debug)]
pub struct BGSPreloadable {
    _data: [u8; 0x08], // 0x0F0 - 0x0E8 = 0x08
}
const _: () = assert!(core::mem::size_of::<BGSPreloadable>() == 0x08);

#[repr(C)]
#[derive(Debug)]
pub struct BGSMessageIcon {
    _data: [u8; 0x18], // 0x110 - 0x0F8 = 0x18
}
const _: () = assert!(core::mem::size_of::<BGSMessageIcon>() == 0x18);

#[repr(C)]
#[derive(Debug)]
pub struct BGSPickupPutdownSounds {
    _data: [u8; 0x18], // 0x128 - 0x110 = 0x18
}
const _: () = assert!(core::mem::size_of::<BGSPickupPutdownSounds>() == 0x18);

#[repr(C)]
#[derive(Debug)]
pub struct BGSBlockBashData {
    _data: [u8; 0x18], // 0x140 - 0x128 = 0x18
}
const _: () = assert!(core::mem::size_of::<BGSBlockBashData>() == 0x18);

#[repr(C)]
#[derive(Debug)]
pub struct BGSImpactDataSet {
    _data: [u8; 0x08],
}

#[repr(C)]
#[derive(Debug)]
pub struct TESObjectSTAT {
    _data: [u8; 0x08],
}

#[repr(C)]
#[derive(Debug)]
pub struct TESEffectShader {
    _data: [u8; 0x08],
}
