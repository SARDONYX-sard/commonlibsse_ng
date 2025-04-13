mod input_context_id;

pub use self::input_context_id::{
    INPUT_CONTEXT_ID, INPUT_CONTEXT_ID_AE, INPUT_CONTEXT_ID_AE_CEnum, INPUT_CONTEXT_ID_SE,
    INPUT_CONTEXT_ID_SE_CEnum, INPUT_CONTEXT_ID_VR, INPUT_CONTEXT_ID_VR_CEnum,
};

use crate::re::BSFixedString::BSFixedString;

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum USER_EVENT_FLAG {
    None = 0,
    Movement = 1 << 0,
    Looking = 1 << 1,
    Activate = 1 << 2,
    Menu = 1 << 3,
    Console = 1 << 4,
    POVSwitch = 1 << 5,
    Fighting = 1 << 6,
    Sneaking = 1 << 7,
    MainFour = 1 << 8,
    WheelZoom = 1 << 9,
    Jumping = 1 << 10,
    VATS = 1 << 11,
    Invalid = 1 << 31,
}

#[repr(C)]
pub struct UserEvents {
    pad001: u8,                       // 0x001
    pad002: u16,                      // 0x002
    pad004: u32,                      // 0x004
    forward: BSFixedString,           // 0x008 - "Forward"
    back: BSFixedString,              // 0x010 - "Back"
    strafeLeft: BSFixedString,        // 0x018 - "Strafe Left"
    strafeRight: BSFixedString,       // 0x020 - "Strafe Right"
    move_: BSFixedString,             // 0x028 - "Move"
    look: BSFixedString,              // 0x030 - "Look"
    activate: BSFixedString,          // 0x038 - "Activate"
    leftAttack: BSFixedString,        // 0x040 - "Left Attack/Block"
    rightAttack: BSFixedString,       // 0x048 - "Right Attack/Block"
    dualAttack: BSFixedString,        // 0x050 - "Dual Attack"
    forceRelease: BSFixedString,      // 0x058 - "ForceRelease"
    pause: BSFixedString,             // 0x060 - "Pause"
    readyWeapon: BSFixedString,       // 0x068 - "Ready Weapon"
    togglePOV: BSFixedString,         // 0x070 - "Toggle POV"
    jump: BSFixedString,              // 0x078 - "Jump"
    journal: BSFixedString,           // 0x080 - "Journal"
    sprint: BSFixedString,            // 0x088 - "Sprint"
    sneak: BSFixedString,             // 0x090 - "Sneak"
    shout: BSFixedString,             // 0x098 - "Shout"
    kinectShout: BSFixedString,       // 0x0A0 - "KinectShout"
    grab: BSFixedString,              // 0x0A8 - "Grab"
    run: BSFixedString,               // 0x0B0 - "Run"
    toggleRun: BSFixedString,         // 0x0B8 - "Toggle Always Run"
    autoMove: BSFixedString,          // 0x0C0 - "Auto-Move"
    quicksave: BSFixedString,         // 0x0C8 - "Quicksave"
    quickload: BSFixedString,         // 0x0D0 - "Quickload"
    newSave: BSFixedString,           // 0x0D8 - "NewSave"
    inventory: BSFixedString,         // 0x0E0 - "Inventory"
    stats: BSFixedString,             // 0x0E8 - "Stats"
    map: BSFixedString,               // 0x0F0 - "Map"
    screenshot: BSFixedString,        // 0x0F8 - "Screenshot"
    multiScreenshot: BSFixedString,   // 0x100 - "Multi-Screenshot"
    console: BSFixedString,           // 0x108 - "Console"
    cameraPath: BSFixedString,        // 0x110 - "CameraPath"
    tweenMenu: BSFixedString,         // 0x118 - "Tween Menu"
    takeAll: BSFixedString,           // 0x120 - "Take All"
    accept: BSFixedString,            // 0x128 - "Accept"
    cancel: BSFixedString,            // 0x130 - "Cancel"
    up: BSFixedString,                // 0x138 - "Up"
    down: BSFixedString,              // 0x140 - "Down"
    left: BSFixedString,              // 0x148 - "Left"
    right: BSFixedString,             // 0x150 - "Right"
    pageUp: BSFixedString,            // 0x158 - "PageUp"
    pageDown: BSFixedString,          // 0x160 - "PageDown"
    pick: BSFixedString,              // 0x168 - "Pick"
    pickNext: BSFixedString,          // 0x170 - "PickNext"
    pickPrevious: BSFixedString,      // 0x178 - "PickPrevious"
    cursor: BSFixedString,            // 0x180 - "Cursor"
    kinect: BSFixedString,            // 0x188 - "Kinect"
    sprintStart: BSFixedString,       // 0x190 - "SprintStart"
    sprintStop: BSFixedString,        // 0x198 - "SprintStop"
    sneakStart: BSFixedString,        // 0x1A0 - "sneakStart"
    sneakStop: BSFixedString,         // 0x1A8 - "sneakStop"
    blockStart: BSFixedString,        // 0x1B0 - "blockStart"
    blockStop: BSFixedString,         // 0x1B8 - "blockStop"
    blockBash: BSFixedString,         // 0x1C0 - "blockBash"
    attackStart: BSFixedString,       // 0x1C8 - "attackStart"
    attackPowerStart: BSFixedString,  // 0x1D0 - "attackPowerStart"
    reverseDirection: BSFixedString,  // 0x1D8 - "reverseDirection"
    unequip: BSFixedString,           // 0x1E0 - "Unequip"
    zoomIn: BSFixedString,            // 0x1E8 - "Zoom In"
    zoomOut: BSFixedString,           // 0x1F0 - "Zoom Out"
    rotateItem: BSFixedString,        // 0x1F8 - "RotateItem"
    leftStick: BSFixedString,         // 0x200 - "Left Stick"
    prevPage: BSFixedString,          // 0x208 - "PrevPage"
    nextPage: BSFixedString,          // 0x210 - "NextPage"
    prevSubPage: BSFixedString,       // 0x218 - "PrevSubPage"
    nextSubPage: BSFixedString,       // 0x220 - "NextSubPage"
    leftEquip: BSFixedString,         // 0x228 - "LeftEquip"
    rightEquip: BSFixedString,        // 0x230 - "RightEquip"
    toggleFavorite: BSFixedString,    // 0x238 - "ToggleFavorite"
    favorites: BSFixedString,         // 0x240 - "Favorites"
    hotkey1: BSFixedString,           // 0x248 - "Hotkey1"
    hotkey2: BSFixedString,           // 0x250 - "Hotkey2"
    hotkey3: BSFixedString,           // 0x258 - "Hotkey3"
    hotkey4: BSFixedString,           // 0x260 - "Hotkey4"
    hotkey5: BSFixedString,           // 0x268 - "Hotkey5"
    hotkey6: BSFixedString,           // 0x270 - "Hotkey6"
    hotkey7: BSFixedString,           // 0x278 - "Hotkey7"
    hotkey8: BSFixedString,           // 0x280 - "Hotkey8"
    quickInventory: BSFixedString,    // 0x288 - "Quick Inventory"
    quickMagic: BSFixedString,        // 0x290 - "Quick Magic"
    quickStats: BSFixedString,        // 0x298 - "Quick Stats"
    quickMap: BSFixedString,          // 0x2A0 - "Quick Map"
    toggleCursor: BSFixedString,      // 0x2A8 - "ToggleCursor"
    wait: BSFixedString,              // 0x2B0 - "Wait"
    click: BSFixedString,             // 0x2B8 - "Click"
    mapLookMode: BSFixedString,       // 0x2C0 - "MapLookMode"
    equip: BSFixedString,             // 0x2C8 - "Equip"
    dropItem: BSFixedString,          // 0x2D0 - "DropItem"
    rotate: BSFixedString,            // 0x2D8 - "Rotate"
    nextFocus: BSFixedString,         // 0x2E0 - "NextFocus"
    prevFocus: BSFixedString,         // 0x2E8 - "PreviousFocus"
    setActiveQuest: BSFixedString,    // 0x2F0 - "SetActiveQuest"
    placePlayerMarker: BSFixedString, // 0x2F8 - "PlacePlayerMarker"
    xButton: BSFixedString,           // 0x300 - "XButton"
    yButton: BSFixedString,           // 0x308 - "YButton"
    chargeItem: BSFixedString,        // 0x310 - "ChargeItem"
    unk318: BSFixedString,            // 0x318 - ""
    playerPosition: BSFixedString,    // 0x320 - "PlayerPosition"
    localMap: BSFixedString,          // 0x328 - "LocalMap"
    localMapMoveMode: BSFixedString,  // 0x330 - "LocalMapMoveMode"
    itemZoom: BSFixedString,          // 0x338 - "Item Zoom"
}
const _: () = assert!(core::mem::size_of::<UserEvents>() == 0x340);

impl UserEvents {
    /// Gets the singleton instance of `UserEvents`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut UserEvents",
        default = "None",
        deref_once,
        id(se = 516458, ae = 402638)
    )]
    pub fn get_singleton() -> Option<&'static UserEvents> {
        |deref_type: DerefType| unsafe { deref_type.as_ref() }
    }
}
