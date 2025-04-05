use crate::re::BGSEncounterZone::BGSEncounterZone;
use crate::re::BSAtomic::BSSpinLock;
use crate::re::BSBitField::BSBitField;
use crate::re::BSPointerHandle::ObjectRefHandle;
use crate::re::BSTArray::BSTArray;
use crate::re::BSTList::BSSimpleList;
use crate::re::BSTSmartPointer::BSTSmartPointer;
use crate::re::ExtraDataList::ExtraDataList;
use crate::re::FormTypes::FormType;
use crate::re::InteriorData::INTERIOR_DATA;
use crate::re::NiSmartPointer::NiPointer;
use crate::re::TESForm::TESForm;
use crate::re::TESFullName::TESFullName;
use crate::re::TESObjectREFR::TESObjectREFR;
use crate::re::TESWorldSpace::TESWorldSpace;
use crate::re::offsets_rtti::RTTI_TESObjectCELL;
use crate::re::offsets_vtable::VTABLE_TESObjectCELL;
use crate::re::{
    BGSLightingTemplate, BGSWaterUpdateI, BSMultiBoundNode, BSPortalGraph, BSTMap, BSTSet, NavMesh,
    NiNode, TESObjectLAND,
};
use crate::rel::id::VariantID;
use core::sync::atomic::AtomicI32;

#[repr(C)]
#[derive(Debug)]
pub struct BGSTerrainVisibilityData {
    pub visData: *mut BSBitField, // 0x0
}
const _: () = assert!(core::mem::size_of::<BGSTerrainVisibilityData>() == 0x8);

// EXTERIOR_DATA (XCLC)
#[repr(C)]
#[derive(Debug)]
pub struct EXTERIOR_DATA {
    pub cellX: i32,                                // 0x00
    pub cellY: i32,                                // 0x04
    pub maxHeightData: *mut i8,                    // 0x08
    pub lodVisData: *mut BGSTerrainVisibilityData, // 0x10
    pub worldX: f32,                               // 0x18
    pub worldY: f32,                               // 0x1C
    pub landHideFlags: u8,                         // 0x20 - Using raw u8 for enum
    pub pad21: u8,                                 // 0x21
    pub pad22: u16,                                // 0x22
    pub pad24: u32,                                // 0x24
}
const _: () = assert!(std::mem::size_of::<EXTERIOR_DATA>() == 0x28);

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LandHideFlag {
    None = 0,
    Quad1 = 1 << 0,
    Quad2 = 1 << 1,
    Quad3 = 1 << 2,
    Quad4 = 1 << 3,
}

// NavMeshArray
#[repr(C)]
pub struct NavMeshArray {
    pub navMeshes: BSTArray<BSTSmartPointer<NavMesh>>, // 0x00
}
const _: () = assert!(std::mem::size_of::<NavMeshArray>() == 0x18);

// LOADED_CELL_DATA
#[repr(C)]
pub struct LOADED_CELL_DATA {
    pub portalGraph: NiPointer<BSPortalGraph>,       // 0x000
    pub cell3D: NiPointer<NiNode>,                   // 0x008
    pub lightMarkerNode: NiPointer<NiNode>,          // 0x010
    pub soundMarkerNode: NiPointer<NiNode>,          // 0x018
    pub multiBoundNode: NiPointer<NiNode>,           // 0x020
    pub unk028: u64,                                 // 0x028
    pub unk030: u64,                                 // 0x030
    pub unk038: u64,                                 // 0x038
    pub unk040: BSTArray<ObjectRefHandle>,           // 0x040
    pub flickeringLights: BSTArray<ObjectRefHandle>, // 0x058
    pub emittanceSourceRefMap: BSTMap<*mut TESForm, ObjectRefHandle>, // 0x070
    pub emittanceLightRefMap: BSTMap<ObjectRefHandle, *mut NiNode>, // 0x090
    pub multiboundRefMap: BSTMap<ObjectRefHandle, NiPointer<BSMultiBoundNode>>, // 0x0B0
    pub refMultiboundMap: BSTMap<*mut BSMultiBoundNode, ObjectRefHandle>, // 0x0D0
    pub activatingRefs: BSSimpleList<ObjectRefHandle>, // 0x0F0
    pub unk100: BSSimpleList<ObjectRefHandle>,       // 0x100
    pub unk110: u64,                                 // 0x110
    pub unk118: BSTArray<*mut ()>,                   // 0x118
    pub decalRefs: BSTArray<ObjectRefHandle>,        // 0x130
    pub skyActors: BSTArray<ObjectRefHandle>,        // 0x148
    pub encounterZone: *mut BGSEncounterZone,        // 0x160
    pub decalsQueued: bool,                          // 0x168
    pub criticalQueuedRefCount: AtomicI32,           // 0x16C
    pub queuedRefCount: AtomicI32,                   // 0x170
    pub queuedDistantRefCount: AtomicI32,            // 0x174
    pub unk178: i32,                                 // 0x178
    pub refsFullyLoaded: bool,                       // 0x17C
}
const _: () = assert!(std::mem::size_of::<LOADED_CELL_DATA>() == 0x180);

#[repr(C)]
#[derive(Debug)]
pub struct TESObjectCELL {
    // Base classes
    pub tes_form: TESForm,          // 0x000
    pub tes_full_name: TESFullName, // 0x020

    // Members
    pub grassCreateLock: BSSpinLock, // 0x030
    pub grassTaskLock: BSSpinLock,   // 0x038
    pub cellFlags: u16,              // 0x040 - Using raw u16 for enum
    pub cellGameFlags: u16,          // 0x042
    pub cellState: u8,               // 0x044 - Using raw u8 for enum
    pub autoWaterLoaded: bool,       // 0x045
    pub cellDetached: bool,          // 0x046
    pub pad047: u8,                  // 0x047
    pub extraList: ExtraDataList,    // 0x048

    // Runtime Data
    pub cellData: CellData,                           // 0x060
    pub cellLand: *mut TESObjectLAND,                 // 0x068
    pub waterHeight: f32,                             // 0x070
    pub navMeshes: *mut NavMeshArray,                 // 0x078
    pub references: BSTSet<NiPointer<TESObjectREFR>>, // 0x080
    pub unk0B0: *mut TESForm,                         // 0x0B0
    pub objectList: BSTArray<*mut TESObjectREFR>,     // 0x0B8
    pub unk0D0: BSTArray<*mut ()>,                    // 0x0D0
    pub waterObjects: BSTArray<*mut BGSWaterUpdateI>, // 0x0E8
    pub unk100: BSTArray<*mut ()>,                    // 0x100
    pub spinLock: BSSpinLock,                         // 0x118
    pub worldSpace: *mut TESWorldSpace,               // 0x120
    pub loadedData: *mut LOADED_CELL_DATA,            // 0x128
    pub lightingTemplate: *mut BGSLightingTemplate,   // 0x130
    pub unk138: u64,                                  // 0x138
}

#[repr(C)]
pub union CellData {
    pub exterior: *mut EXTERIOR_DATA,
    pub interior: *mut INTERIOR_DATA,
}
const _: () = assert!(std::mem::size_of::<CellData>() == 0x8);
impl core::fmt::Debug for CellData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unsafe {
            f.debug_struct("CellData")
                .field("exterior", &self.exterior)
                .field("interior", &self.interior)
                .finish()
        }
    }
}

impl TESObjectCELL {
    pub const RTTI: VariantID = RTTI_TESObjectCELL;
    pub const VTABLE: [VariantID; 2] = VTABLE_TESObjectCELL;
    pub const FORM_TYPE: FormType = FormType::Cell;
}

pub enum Flag {
    None = 0,
    IsInteriorCell = 1 << 0,
    HasWater = 1 << 1,
    CanTravelFromHere = 1 << 2,
    NoLODWater = 1 << 3,
    HasTempData = 1 << 4,
    PublicArea = 1 << 5,
    HandChanged = 1 << 6,
    ShowSky = 1 << 7,
    UseSkyLighting = 1 << 8,
    WarnToLeave = 1 << 9,
}

pub enum CellState {
    Attached = 7,
}
