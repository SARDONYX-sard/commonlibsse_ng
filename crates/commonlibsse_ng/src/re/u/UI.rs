use core::ffi::{CStr, c_void};

use crate::re::BSAtomic::BSSpinLock;
use crate::re::BSFixedString::BSFixedString;
use crate::re::BSTArray::BSTArray;
use crate::re::BSTEvent::{BSTEventSink, BSTEventSource};
use crate::re::BSTHashMap::BSTHashMap;
use crate::re::BSTimer::BSTimer;
use crate::re::GFxMovieView;
use crate::re::GPtr::GPtr;
use crate::re::IMenu::IMenu;
use crate::re::MenuModeChangeEvent::MenuModeChangeEvent;
use crate::re::MenuOpenCloseEvent::MenuOpenCloseEvent;

#[rustfmt::skip]
#[repr(C)]
#[derive(Debug)]
pub struct UI {
    pub __base : [u8; 8],                                // 0x000 non Empty base optimization: BSTSingletonSDM
    pub __base1: BSTEventSource<MenuOpenCloseEvent>,     // 0x008
    pub __base2: BSTEventSource<MenuModeChangeEvent>,    // 0x060
    pub __base3: BSTEventSource<*mut c_void>,            // 0x0B8 MenuModeCounterChangedEvent/TutorialEvent
    pub menuStack: BSTArray<GPtr<IMenu>>,                // 0x110
    pub menuMap: BSTHashMap<BSFixedString, UIMenuEntry>, // 0x128
    pub processMessagesLock: BSSpinLock,                 // 0x158
    pub numPausesGame: u32,                              // 0x160 (= 0) += 1 if (i_menu->flags & 0x00001)
    pub numItemMenus: u32,                               // 0x164 (= 0) += 1 if (i_menu->flags & 0x02000)
    pub numDisablePauseMenu: u32,                        // 0x168 (= 0) += 1 if (i_menu->flags & 0x00080)
    pub numAllowSaving: u32,                             // 0x16C (= 0) += 1 if (i_menu->flags & 0x00800)
    pub numDontHideCursorWhenTopmost: u32,               // 0x170 (= 0) += 1 if (i_menu->flags & 0x04000)
    pub numCustomRendering: u32,                         // 0x174 (= 0) += 1 if (i_menu->flags & 0x08000)
    pub numApplicationMenus: u32,                        // 0x178 (= 0) += 1 if (i_menu->flags & 0x20000)
    pub modal: bool,                                     // 0x17C (= 0)  = 1 if (i_menu->flags & 0x00010)
    pub pad17D: u8,                                      // 0x17D
    pub pad17E: u16,                                     // 0x17E
    pub uiTimer: BSTimer,                                // 0x180
    pub menuSystemVisible: bool,                         // 0x1C0
    pub closingAllMenus: bool,                           // 0x1C1
    pub pad1C2: u16,                                     // 0x1C2
    pub pad1C4: u32,                                     // 0x1C4
    pub unk1C8: u32,                                     // 0x1C8 - VR
    pub unk1CA: u32,                                     // 0x1CA - VR
}
const _: () = assert!(core::mem::size_of::<UI>() == 0x1D0);

pub trait HasMenuName {
    const MENU_NAME: &'static CStr;
}

impl UI {
    /// Returns a reference to the singleton instance of `UI`, if available.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut UI",
        default = "None",
        deref_once,
        id(se = 514178, ae = 400327)
    )]
    #[inline]
    pub fn get_singleton() -> Option<&'static UI> {
        |deref_type: DerefType| unsafe { deref_type.as_ref() }
    }

    /// Returns a mutable reference to the singleton instance of `UI`, if available.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut UI",
        default = "None",
        deref_once,
        id(se = 514178, ae = 400327)
    )]
    #[inline]
    pub fn get_singleton_mut() -> Option<&'static mut UI> {
        |deref_type: DerefType| unsafe { deref_type.as_mut() }
    }

    /// Returns true if the game is currently paused.
    #[inline]
    pub const fn game_is_paused(&self) -> bool {
        self.numPausesGame > 0
    }

    /// Gets a pointer to the specified menu if it is currently registered.
    #[inline]
    pub fn get_menu(&self, menu_name: &CStr) -> Option<GPtr<IMenu>> {
        self.menuMap
            .get(&BSFixedString::new(menu_name))
            .map(|menu_entry| GPtr::clone(&menu_entry.menu))
    }

    /// Gets the `GFxMovieView` associated with the specified menu.
    #[inline]
    pub fn get_movie_view(&self, menu_name: &CStr) -> Option<GPtr<GFxMovieView>> {
        self.get_menu(menu_name).map(|menu_ptr| GPtr::clone(&menu_ptr.uiMovie))
    }

    /// Returns true if the cursor should be hidden when this UI is the topmost.
    #[inline]
    pub const fn is_cursor_hidden_when_topmost(&self) -> bool {
        self.numDontHideCursorWhenTopmost == 0
    }

    /// Returns true if an inventory/item menu is currently open.
    #[inline]
    pub const fn is_item_menu_open(&self) -> bool {
        self.numItemMenus > 0
    }

    /// Checks whether a given menu is currently open and on the stack.
    #[inline]
    pub fn is_menu_open(&self, menu_name: &CStr) -> bool {
        self.get_menu(menu_name).is_some_and(|menu_ptr| menu_ptr.on_stack())
    }

    /// Returns true if any modal menu is currently open.
    #[inline]
    pub const fn is_modal_menu_open(&self) -> bool {
        self.modal
    }

    /// Returns true if the pause menu is currently disabled.
    #[inline]
    pub const fn is_pause_menu_disabled(&self) -> bool {
        self.numDisablePauseMenu > 0
    }

    /// Returns true if saving the game is currently allowed.
    #[inline]
    pub const fn is_saving_allowed(&self) -> bool {
        self.numAllowSaving > 0
    }

    /// Returns true if the menu system is currently being displayed.
    #[inline]
    pub const fn is_showing_menus(&self) -> bool {
        self.menuSystemVisible
    }

    /// Returns true if custom rendering is being used for menus.
    #[inline]
    pub const fn is_using_custom_rendering(&self) -> bool {
        self.numCustomRendering > 0
    }

    /// Registers a new menu with the UI system.
    #[inline]
    pub fn register(&mut self, menu_name: &CStr, creator: fn() -> *mut IMenu) {
        self.menuMap.insert(BSFixedString::new(menu_name), UIMenuEntry::new(GPtr::null(), creator));
    }

    /// Sets whether the UI system should be visible.
    #[inline]
    pub const fn show_menus(&mut self, show: bool) {
        self.menuSystemVisible = show;
    }

    #[inline]
    pub fn get_menu_by_name<T>(&self, name: &CStr) -> Option<GPtr<T>>
    where
        T: crate::re::GPtr::RefCounted,
    {
        self.get_menu(name).map(|menu_ptr| menu_ptr.cast::<T>())
    }

    #[inline]
    pub fn get_menu_as<T>(&self) -> Option<GPtr<T>>
    where
        T: HasMenuName + crate::re::GPtr::RefCounted,
    {
        self.get_menu_by_name::<T>(T::MENU_NAME)
    }
}

macro_rules! impl_ui_event_sink {
    (
        $(
            ($add_fn:ident, $remove_fn:ident, $field:ident, $event_ty:ty)
        ),*
        $(,)?
    ) => {
        impl UI {
            $(
                pub fn $add_fn(&mut self, sink: *mut BSTEventSink<$event_ty>) {
                    self.$field.add_event_sink(sink);
                }

                pub fn $remove_fn(&mut self, sink: *mut BSTEventSink<$event_ty>) {
                    self.$field.remove_event_sink(sink);
                }
            )*
        }
    };
}
impl_ui_event_sink! {
    (add_menu_open_close_sink, remove_menu_open_close_sink, __base1, MenuOpenCloseEvent),
    (add_menu_mode_change_sink, remove_menu_mode_change_sink, __base2, MenuModeChangeEvent),
    (add_void_sink, remove_void_sink, __base3, *mut c_void),
}

#[repr(C)]
#[derive(Debug)]
pub struct UIMenuEntry {
    pub menu: GPtr<IMenu>,               // 0x0
    pub menuCreator: fn() -> *mut IMenu, // 0x8
}
const _: () = assert!(core::mem::size_of::<UIMenuEntry>() == 0x10);

impl UIMenuEntry {
    #[inline]
    pub const fn new(menu: GPtr<IMenu>, menu_creator: fn() -> *mut IMenu) -> Self {
        Self { menu, menuCreator: menu_creator }
    }
}
