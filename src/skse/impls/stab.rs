// https://github.com/alandtse/CommonLibVR/blob/ng/include/SKSE/Impl/Stubs.h

use core::ffi::{c_char, c_void, CStr};

#[repr(transparent)]
pub struct PluginHandle(pub u32);
pub const INVALID_PLUGIN_HANDLE: PluginHandle = PluginHandle(u32::MAX);

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginInfo {
    pub info_version: u32,
    /// Use a raw pointer to represent a C-style string
    pub name: *const c_char,
    pub version: u32,
}

impl Default for PluginInfo {
    fn default() -> Self {
        Self {
            info_version: Default::default(),
            name: unsafe { CStr::from_bytes_with_nul_unchecked(b"") }.as_ptr(),
            version: Default::default(),
        }
    }
}

#[repr(C)]
pub struct SKSEInterface {
    pub skse_version: u32,
    pub runtime_version: u32,
    pub editor_version: u32,
    pub is_editor: u32,
    pub query_interface: unsafe fn(u32) -> *mut c_void,
    pub get_plugin_handle: unsafe fn() -> PluginHandle,
    pub get_release_index: unsafe fn() -> u32,
    pub get_plugin_info: unsafe fn(*const c_char) -> *const c_void,
}

#[repr(C)]
pub struct SKSEMessagingInterface {
    pub interface_version: u32,
    pub register_listener: unsafe fn(PluginHandle, *const c_char, *mut c_void) -> bool,
    pub dispatch: unsafe fn(PluginHandle, u32, *mut c_void, u32, *const c_char) -> bool,
    pub get_event_dispatcher: unsafe fn(u32) -> *mut c_void,
}

#[repr(C)]
pub struct SKSEObjectInterface {
    pub interface_version: u32,
    pub get_delay_functor_manager: unsafe fn() -> *mut SKSEDelayFunctorManager,
    pub get_object_registry: unsafe fn() -> *mut SKSEObjectRegistry,
    pub get_persistent_object_storage: unsafe fn() -> *mut SKSEPersistentObjectStorage,
}

#[repr(C)]
pub struct SKSEPapyrusInterface {
    pub interface_version: u32,
    pub register: unsafe fn(*mut std::ffi::c_void) -> bool,
}

#[repr(C)]
pub struct SKSEScaleformInterface {
    pub interface_version: u32,
    pub register: unsafe fn(*const c_char, *mut c_void) -> bool,
    pub register_for_inventory: unsafe fn(*mut c_void),
}

#[repr(C)]
pub struct SKSESerializationInterface {
    pub version: u32,
    pub set_unique_id: unsafe fn(PluginHandle, u32),
    pub set_revert_callback: unsafe fn(PluginHandle, *mut c_void),
    pub set_save_callback: unsafe fn(PluginHandle, *mut c_void),
    pub set_load_callback: unsafe fn(PluginHandle, *mut c_void),
    pub set_form_delete_callback: unsafe fn(PluginHandle, *mut c_void),
    pub write_record: unsafe fn(u32, u32, *const c_void, u32) -> bool,
    pub open_record: unsafe fn(u32, u32) -> bool,
    pub write_record_data: unsafe fn(*const c_void, u32) -> bool,
    pub get_next_record_info: unsafe fn(*mut u32, *mut u32, *mut u32) -> bool,
    pub read_record_data: unsafe fn(*mut c_void, u32) -> u32,
    pub resolve_handle: unsafe fn(u64, *mut u64) -> bool,
    pub resolve_form_id: unsafe fn(u32, *mut u32) -> bool,
}

#[repr(C)]
pub struct SKSETaskInterface {
    pub interface_version: u32,
    pub add_task: unsafe fn(*mut c_void),
    pub add_ui_task: unsafe fn(*mut c_void),
}

#[repr(C)]
pub struct SKSETrampolineInterface {
    pub interface_version: u32,
    pub allocate_from_branch_pool: unsafe fn(PluginHandle, usize) -> *mut c_void,
    pub allocate_from_local_pool: unsafe fn(PluginHandle, usize) -> *mut c_void,
}

#[repr(C)]
pub struct SKSEDelayFunctorManager;

#[repr(C)]
pub struct SKSEObjectRegistry;

#[repr(C)]
pub struct SKSEPersistentObjectStorage;

#[repr(C)]
pub struct TaskDelegate {
    pub vtbl: *const TaskDelegateVirtualTable,
}

#[repr(C)]
pub struct UiDelegateV1 {
    pub vtbl: *const UiDelegateV1VirtualTable,
}

#[repr(C)]
pub struct TaskDelegateVirtualTable {
    pub run: unsafe fn(this: *const c_void),
    pub dispose: fn(this: *const c_void),
}

#[repr(C)]
pub struct UiDelegateV1VirtualTable {
    pub run: fn(this: *const c_void),
    pub dispose: fn(this: *const c_void),
}
