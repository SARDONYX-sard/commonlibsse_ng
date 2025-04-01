use crate::re::NiSmartPointer::NiPointer;
use crate::re::{BSFadeNode, BSTriShape, NiBillboardNode, NiNode};
use std::ffi::c_void;

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VR_Bow_State {
    #[default]
    None,
    NoAmmo,
    Idle,
    ArrowKnocked,
}

#[derive(Debug)]
pub struct VR_NODE_DATA {
    pub PlayerWorldNode: NiPointer<NiNode>,               // 0x3F0
    pub FollowNode: NiPointer<NiNode>,                    // 0x3F8
    pub FollowOffset: NiPointer<NiNode>,                  // 0x400
    pub HeightOffsetNode: NiPointer<NiNode>,              // 0x408
    pub SnapWalkOffsetNode: NiPointer<NiNode>,            // 0x410
    pub RoomNode: NiPointer<NiNode>,                      // 0x418
    pub BlackSphere: NiPointer<NiNode>,                   // 0x420
    pub uiNode: NiPointer<NiNode>,                        // 0x428
    pub InWorldUIQuadGeo: NiPointer<BSTriShape>,          // 0x430
    pub UIPointerNode: NiPointer<NiNode>,                 // 0x438
    pub UIPointerGeo: NiPointer<BSTriShape>,              // 0x440
    pub DialogueUINode: NiPointer<NiNode>,                // 0x448
    pub TeleportDestinationPreview: NiPointer<NiNode>,    // 0x450
    pub TeleportDestinationFail: NiPointer<NiNode>,       // 0x458
    pub TeleportSprintPreview: NiPointer<NiNode>,         // 0x460
    pub SpellOrigin: NiPointer<NiNode>,                   // 0x468
    pub SpellDestination: NiPointer<NiNode>,              // 0x470
    pub ArrowOrigin: NiPointer<NiNode>,                   // 0x478
    pub ArrowDestination: NiPointer<NiNode>,              // 0x480
    pub QuestMarker: NiPointer<NiNode>,                   // 0x488
    pub LeftWandNode: NiPointer<NiNode>,                  // 0x490
    pub LeftWandShakeNode: NiPointer<NiNode>,             // 0x498
    pub LeftValveIndexControllerNode: NiPointer<NiNode>,  // 0x4A0
    pub unkNode4A8: NiPointer<NiNode>,                    // 0x4A8
    pub LeftWeaponOffsetNode: NiPointer<NiNode>,          // 0x4B0
    pub LeftCrossbowOffsetNode: NiPointer<NiNode>,        // 0x4B8
    pub LeftMeleeWeaponOffsetNode: NiPointer<NiNode>,     // 0x4C0
    pub LeftStaffWeaponOffsetNode: NiPointer<NiNode>,     // 0x4C8
    pub LeftShieldOffsetNode: NiPointer<NiNode>,          // 0x4D0
    pub RightShieldOffsetNode: NiPointer<NiNode>,         // 0x4D8
    pub SecondaryMagicOffsetNode: NiPointer<NiNode>,      // 0x4E0
    pub SecondaryMagicAimNode: NiPointer<NiNode>,         // 0x4E8
    pub SecondaryStaffMagicOffsetNode: NiPointer<NiNode>, // 0x4F0
    pub RightWandNode: NiPointer<NiNode>,                 // 0x4F8
    pub RightWandShakeNode: NiPointer<NiNode>,            // 0x500
    pub RightValveIndexControllerNode: NiPointer<NiNode>, // 0x508
    pub unkNode510: NiPointer<NiNode>,                    // 0x510
    pub RightWeaponOffsetNode: NiPointer<NiNode>,         // 0x518
    pub RightCrossbowOffsetNode: NiPointer<NiNode>,       // 0x520
    pub RightMeleeWeaponOffsetNode: NiPointer<NiNode>,    // 0x528
    pub RightStaffWeaponOffsetNode: NiPointer<NiNode>,    // 0x530
    pub PrimaryMagicOffsetNode: NiPointer<NiNode>,        // 0x538
    pub PrimaryMagicAimNode: NiPointer<NiNode>,           // 0x540
    pub PrimaryStaffMagicOffsetNode: NiPointer<NiNode>,   // 0x548
    pub unk550: u64,                                      // 0x550
    pub CrosshairParent: NiPointer<NiBillboardNode>,      // 0x558
    pub CrosshairSecondaryParent: NiPointer<NiBillboardNode>, // 0x560
    pub TargetLockParent: NiPointer<NiBillboardNode>,     // 0x568
    pub GamepadNode: NiPointer<NiNode>,                   // 0x570
    pub LastSyncPos: NiPointer<NiNode>,                   // 0x578
    pub UprightHmdNode: NiPointer<NiNode>,                // 0x580
    pub MapMarkers3D: NiPointer<NiNode>,                  // 0x588
    pub NPCLHnd: NiPointer<NiNode>,                       // 0x590
    pub NPCRHnd: NiPointer<NiNode>,                       // 0x598
    pub NPCLClv: NiPointer<NiNode>,                       // 0x5A0
    pub NPCRClv: NiPointer<NiNode>,                       // 0x5A8
    pub unk5B0: u32,                                      // 0x5B0
    pub unk5B4: u32,                                      // 0x5B4
    pub unk5B8: u64,                                      // 0x5B8
    pub bowState: VR_Bow_State,                           // 0x5C0
    pub unk5C4: u32,                                      // 0x5C4
    pub BowAimNode: NiPointer<NiNode>,                    // 0x5C8
    pub BowRotationNode: NiPointer<NiNode>,               // 0x5D0
    pub ArrowSnapNode: NiPointer<NiNode>,                 // 0x5D8
    pub ArrowNode: NiPointer<BSFadeNode>,                 // 0x5E0
    pub ArrowFireNode: NiPointer<BSFadeNode>,             // 0x5E8
    pub unk5F0: u64,                                      // 0x5F0
    pub ArrowHoldOffsetNode: NiPointer<NiNode>,           // 0x5F8
    pub ArrowHoldNode: NiPointer<NiNode>,                 // 0x600
    pub unk608: u64,                                      // 0x608
    pub currentArrowSnapDistance: f32,                    // 0x610
    pub unk614: u32,                                      // 0x614
    pub currentBowDrawAmount: f32,                        // 0x618 - 0 to 1
    pub lastRumbleBowDrawAmount: f32,                     // 0x61C - 0 to 1
    pub unk620: u64,                                      // 0x620
    pub unk628: u64,                                      // 0x628
    pub unk630: u64,                                      // 0x630
    pub QuestMarkerBillBoardsNodeArray: *mut c_void, // 0x638    TODO - Make into proper data structure
    pub TeleportNodeArray: *mut c_void, // 0x640    TODO - Make into proper data structure
    pub QuestMarkerBillBoardsNodeArray2: *mut c_void, // 0x648    TODO - Make into proper data structure -> points to same place as QuestMarkerBillBoardsNodeArray
    pub unk650: u64,                                  // 0x650
    pub TeleportNodeArray2: *mut c_void, // 0x658    TODO - Make into proper data structure -> points to same place as TeleportNodeArray
    pub QuestMarkerBillBoardsNodeArray3: *mut c_void, // 0x660    TODO - Make into proper data structure -> points to same place as QuestMarkerBillBoardsNodeArray
    pub unk668: u64,                                  // 0x668
    pub unkf32670: f32,                               // 0x670
    pub unk674: u32,                                  // 0x674
    pub TeleportNodeArray3: *mut c_void, // 0x678    TODO - Make into proper data structure
}
const _: () = assert!(core::mem::size_of::<VR_NODE_DATA>() == 0x290);
