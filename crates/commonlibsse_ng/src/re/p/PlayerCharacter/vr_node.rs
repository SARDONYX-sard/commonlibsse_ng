use crate::re::NiSmartPointer::NiPointer;
use crate::re::PlayerCharacter::VR_Bow_StateFlags;
use crate::re::{BSFadeNode, BSTriShape, NiBillboardNode, NiNode};
use std::ffi::c_void;

#[repr(C)]
pub struct VRNodeData {
    player_world_node: NiPointer<NiNode>,                   // 3F0
    follow_node: NiPointer<NiNode>,                         // 3F8
    follow_offset: NiPointer<NiNode>,                       // 400
    height_offset_node: NiPointer<NiNode>,                  // 408
    snap_walk_offset_node: NiPointer<NiNode>,               // 410
    room_node: NiPointer<NiNode>,                           // 418
    black_sphere: NiPointer<NiNode>,                        // 420
    ui_node: NiPointer<NiNode>,                             // 428
    in_world_ui_quad_geo: NiPointer<BSTriShape>,            // 430
    ui_pointer_node: NiPointer<NiNode>,                     // 438
    ui_pointer_geo: NiPointer<BSTriShape>,                  // 440
    dialogue_ui_node: NiPointer<NiNode>,                    // 448
    teleport_destination_preview: NiPointer<NiNode>,        // 450
    teleport_destination_fail: NiPointer<NiNode>,           // 458
    teleport_sprint_preview: NiPointer<NiNode>,             // 460
    spell_origin: NiPointer<NiNode>,                        // 468
    spell_destination: NiPointer<NiNode>,                   // 470
    arrow_origin: NiPointer<NiNode>,                        // 478
    arrow_destination: NiPointer<NiNode>,                   // 480
    quest_marker: NiPointer<NiNode>,                        // 488
    left_wand_node: NiPointer<NiNode>,                      // 490
    left_wand_shake_node: NiPointer<NiNode>,                // 498
    left_valve_index_controller_node: NiPointer<NiNode>,    // 4A0
    unk_node4a8: NiPointer<NiNode>,                         // 4A8
    left_weapon_offset_node: NiPointer<NiNode>,             // 4B0
    left_crossbow_offset_node: NiPointer<NiNode>,           // 4B8
    left_melee_weapon_offset_node: NiPointer<NiNode>,       // 4C0
    left_staff_weapon_offset_node: NiPointer<NiNode>,       // 4C8
    left_shield_offset_node: NiPointer<NiNode>,             // 4D0
    right_shield_offset_node: NiPointer<NiNode>,            // 4D8
    secondary_magic_offset_node: NiPointer<NiNode>,         // 4E0
    secondary_magic_aim_node: NiPointer<NiNode>,            // 4E8
    secondary_staff_magic_offset_node: NiPointer<NiNode>,   // 4F0
    right_wand_node: NiPointer<NiNode>,                     // 4F8
    right_wand_shake_node: NiPointer<NiNode>,               // 500
    right_valve_index_controller_node: NiPointer<NiNode>,   // 508
    unk_node510: NiPointer<NiNode>,                         // 510
    right_weapon_offset_node: NiPointer<NiNode>,            // 518
    right_crossbow_offset_node: NiPointer<NiNode>,          // 520
    right_melee_weapon_offset_node: NiPointer<NiNode>,      // 528
    right_staff_weapon_offset_node: NiPointer<NiNode>,      // 530
    primary_magic_offset_node: NiPointer<NiNode>,           // 538
    primary_magic_aim_node: NiPointer<NiNode>,              // 540
    primary_staff_magic_offset_node: NiPointer<NiNode>,     // 548
    unk550: u64,                                            // 550
    crosshair_parent: NiPointer<NiBillboardNode>,           // 558
    crosshair_secondary_parent: NiPointer<NiBillboardNode>, // 560
    target_lock_parent: NiPointer<NiBillboardNode>,         // 568
    gamepad_node: NiPointer<NiNode>,                        // 570
    last_sync_pos: NiPointer<NiNode>,                       // 578
    upright_hmd_node: NiPointer<NiNode>,                    // 580
    map_markers_3d: NiPointer<NiNode>,                      // 588
    npc_lhnd: NiPointer<NiNode>,                            // 590
    npc_rhnd: NiPointer<NiNode>,                            // 598
    npc_lclv: NiPointer<NiNode>,                            // 5A0
    npc_rclv: NiPointer<NiNode>,                            // 5A8
    unk5b0: u32,                                            // 5B0
    unk5b4: u32,                                            // 5B4
    unk5b8: u64,                                            // 5B8
    bow_state: VR_Bow_StateFlags,                           // 5C0
    unk5c4: u32,                                            // 5C4
    bow_aim_node: NiPointer<NiNode>,                        // 5C8
    bow_rotation_node: NiPointer<NiNode>,                   // 5D0
    arrow_snap_node: NiPointer<NiNode>,                     // 5D8
    arrow_node: NiPointer<BSFadeNode>,                      // 5E0
    arrow_fire_node: NiPointer<BSFadeNode>,                 // 5E8
    unk5f0: u64,                                            // 5F0
    arrow_hold_offset_node: NiPointer<NiNode>,              // 5F8
    arrow_hold_node: NiPointer<NiNode>,                     // 600
    unk608: u64,                                            // 608
    current_arrow_snap_distance: f32,                       // 610
    unk614: u32,                                            // 614
    current_bow_draw_amount: f32,                           // 618
    last_rumble_bow_draw_amount: f32,                       // 61C
    unk620: u64,                                            // 620
    unk628: u64,                                            // 628
    unk630: u64,                                            // 630
    quest_marker_billboards_node_array: *mut c_void,        // 638
    teleport_node_array: *mut c_void,                       // 640
    quest_marker_billboards_node_array2: *mut c_void,       // 648
    unk650: u64,                                            // 650
    teleport_node_array2: *mut c_void,                      // 658
    quest_marker_billboards_node_array3: *mut c_void,       // 660
    unk668: u64,                                            // 668
    unk_float670: f32,                                      // 670
    unk674: u32,                                            // 674
    teleport_node_array3: *mut c_void,                      // 678
}
const _: () = assert!(core::mem::size_of::<VRNodeData>() == 0x290);
