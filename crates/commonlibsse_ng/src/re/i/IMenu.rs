use crate::re::GPtr::GPtr;
use crate::re::UIMessage::UIMessage;
use crate::re::UserEvents::INPUT_CONTEXT_ID;
use crate::re::offsets_rtti::RTTI_IMenu;
use crate::re::offsets_vtable::VTABLE_IMenu;
use crate::re::{CallbackProcessor, FxDelegate, GFxMovieView};
use crate::rel::id::VariantID;
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug)]
pub struct IMenu {
    // NOTE: extract FxDelegateHandler size 0x10
    pub vtable: *const IMenuVtbl,                // 0x00
    pub refCount: core::sync::atomic::AtomicU32, // 0x04
    pad0C: u16,                                  // 0x08

    pub uiMovie: GPtr<GFxMovieView>,    // 0x10
    pub depthPriority: i8,              // 0x18
    pad19: u8,                          // 0x19
    pad20: u16,                         // 0x1A
    pub menuFlags: UI_MENU_FLAGS,       // 0x1C
    pub inputContext: INPUT_CONTEXT_ID, // 0x20
    pad24: u32,                         // 0x24
    pub fxDelegate: GPtr<FxDelegate>,   // 0x28
}
const _: () = assert!(core::mem::size_of::<IMenu>() == 0x30);

impl IMenu {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_IMenu;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_IMenu;

    /// Returns `true` if the menu advances while the pause menu is active.
    #[inline]
    pub const fn advances_under_pause_menu(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::AdvancesUnderPauseMenu)
    }

    /// Returns `true` if the menu allows saving the game.
    #[inline]
    pub const fn allow_saving(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::AllowSaving)
    }

    /// Returns `true` if the menu is always open.
    #[inline]
    pub const fn always_open(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::AlwaysOpen)
    }

    /// Returns `true` if this is an application-level menu.
    #[inline]
    pub const fn application_menu(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::ApplicationMenu)
    }

    /// Returns `true` if the cursor is assigned to the renderer.
    #[inline]
    pub const fn assign_cursor_to_renderer(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::AssignCursorToRenderer)
    }

    /// Returns `true` if the menu uses custom rendering.
    #[inline]
    pub const fn custom_rendering(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::CustomRendering)
    }

    /// Returns `true` if the companion app is allowed to access this menu.
    #[inline]
    pub const fn companion_app_allowed(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::CompanionAppAllowed)
    }

    /// Returns `true` if the pause menu should be disabled.
    #[inline]
    pub const fn disable_pause_menu(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::DisablePauseMenu)
    }

    /// Returns `true` if the cursor should remain visible when this menu is topmost.
    #[inline]
    pub const fn dont_hide_cursor_when_topmost(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::DontHideCursorWhenTopmost)
    }

    /// Returns `true` if the background is frozen while the menu is active.
    #[inline]
    pub const fn freeze_frame_background(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::FreezeFrameBackground)
    }

    /// Returns `true` if the menu causes a freeze frame pause.
    #[inline]
    pub const fn freeze_frame_pause(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::FreezeFramePause)
    }

    /// Returns `true` if the menu includes a button bar.
    #[inline]
    pub const fn has_button_bar(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::HasButtonBar)
    }

    /// Returns `true` if this is an inventory item menu.
    #[inline]
    pub const fn inventory_item_menu(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::InventoryItemMenu)
    }

    /// Returns `true` if this menu is the top button bar.
    #[inline]
    pub const fn is_top_button_bar(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::IsTopButtonBar)
    }

    /// Returns `true` if large-scaleform render cache mode is enabled.
    #[inline]
    pub const fn large_scaleform_render_cache_mode(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::LargeScaleformRenderCacheMode)
    }

    /// Returns `true` if the menu is modal and blocks interaction with other menus.
    #[inline]
    pub const fn modal(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::Modal)
    }

    /// Returns `true` if the menu is currently on the menu stack.
    #[inline]
    pub const fn on_stack(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::OnStack)
    }

    /// Returns `true` if the menu causes the game to pause.
    #[inline]
    pub const fn pauses_game(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::PausesGame)
    }

    /// Returns `true` if the menu renders offscreen targets.
    #[inline]
    pub const fn renders_offscreen_targets(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::RendersOffscreenTargets)
    }

    /// Returns `true` if the menu renders underneath the pause menu.
    #[inline]
    pub const fn renders_under_pause_menu(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::RendersUnderPauseMenu)
    }

    /// Returns `true` if the menu requires updates while active.
    #[inline]
    pub const fn requires_update(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::RequiresUpdate)
    }

    /// Returns `true` if rendering is skipped during freeze frame screenshots.
    #[inline]
    pub const fn skip_render_during_freeze_frame_screenshot(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::SkipRenderDuringFreezeFrameScreenshot)
    }

    /// Returns `true` if this is the topmost rendered menu.
    #[inline]
    pub const fn topmost_rendered_menu(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::TopmostRenderedMenu)
    }

    /// Returns `true` if the menu uses the cursor during update.
    #[inline]
    pub const fn update_uses_cursor(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::UpdateUsesCursor)
    }

    /// Returns `true` if the menu uses a blurred background.
    #[inline]
    pub const fn uses_blurred_background(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::UsesBlurredBackground)
    }

    /// Returns `true` if the menu uses a cursor for interaction.
    #[inline]
    pub const fn uses_cursor(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::UsesCursor)
    }

    /// Returns `true` if the menu uses a custom menu context.
    #[inline]
    pub const fn uses_menu_context(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::UsesMenuContext)
    }

    /// Returns `true` if movement input is translated into direction within the menu.
    #[inline]
    pub const fn uses_movement_to_direction(&self) -> bool {
        self.menuFlags.contains(UI_MENU_FLAGS::UsesMovementToDirection)
    }
}

pub struct IMenuVtbl {
    /// Destructor for `IMenu` (represented as a virtual method in C++).
    pub CxxDrop: unsafe extern "C" fn(this: *mut IMenu),
    pub Accept: fn(this: *mut IMenu, processor: *mut CallbackProcessor), // 01 - override { return; }

    pub PostCreate: fn(this: *mut IMenu),     // 0x02
    pub Unk_03: fn(this: *mut IMenu, c_void), // 0x03
    pub ProcessMessage: fn(this: *mut IMenu, message: &UIMessage) -> UI_MESSAGE_RESULTS, // 0x04
    pub AdvanceMovie: fn(this: *mut IMenu, interval: f32, currentTime: u32), // 0x05
    pub PostDisplay: fn(this: *mut IMenu),    // 0x06

    /// Only available if kRendersOffscreenTargets is set
    pub PreDisplay: fn(this: *mut IMenu), // 0x07
    pub RefreshPlatform: fn(this: *mut IMenu), // 0x08

    /// - VR Only method: `{ unk30 = a_unk; }`
    pub Unk_09: fn(this: *mut IMenu, unk: UI_MENU_Unk09), // 0x09
    /// Does something with _root.ResetOnShow swf function
    /// - VR Only method.
    pub Unk_0A: fn(this: *mut IMenu), // 0x0A
}

impl crate::re::GPtr::RefCounted for IMenu {
    fn add_ref(&mut self) {
        self.refCount.fetch_add(1, core::sync::atomic::Ordering::Acquire);
    }
    fn release(&mut self) {
        self.refCount.fetch_sub(1, core::sync::atomic::Ordering::Release);
    }
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum UI_MENU_FLAGS {
    None = 0,
    PausesGame = 1 << 0,
    AlwaysOpen = 1 << 1,
    UsesCursor = 1 << 2,
    UsesMenuContext = 1 << 3,
    Modal = 1 << 4, // prevents lower movies with this flag from advancing
    FreezeFrameBackground = 1 << 5,
    OnStack = 1 << 6,
    DisablePauseMenu = 1 << 7,
    RequiresUpdate = 1 << 8,
    TopmostRenderedMenu = 1 << 9,
    UpdateUsesCursor = 1 << 10,
    AllowSaving = 1 << 11,
    RendersOffscreenTargets = 1 << 12,
    InventoryItemMenu = 1 << 13,
    DontHideCursorWhenTopmost = 1 << 14,
    CustomRendering = 1 << 15,
    AssignCursorToRenderer = 1 << 16,
    ApplicationMenu = 1 << 17,
    HasButtonBar = 1 << 18,
    IsTopButtonBar = 1 << 19,
    AdvancesUnderPauseMenu = 1 << 20,
    RendersUnderPauseMenu = 1 << 21,
    UsesBlurredBackground = 1 << 22,
    CompanionAppAllowed = 1 << 23,
    FreezeFramePause = 1 << 24,
    SkipRenderDuringFreezeFrameScreenshot = 1 << 25,
    LargeScaleformRenderCacheMode = 1 << 26,
    UsesMovementToDirection = 1 << 27,
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum UI_MESSAGE_RESULTS {
    Handled = 0,
    Ignore = 1,
    PassOn = 2,
}

/// NOTE: Entire enum needs more REing
#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum UI_MENU_Unk09 {
    None = u32::MAX,
}
