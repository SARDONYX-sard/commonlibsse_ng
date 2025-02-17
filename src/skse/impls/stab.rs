// https://github.com/alandtse/CommonLibVR/blob/ng/include/SKSE/Impl/Stubs.h

pub type PluginHandle = u32;
pub const K_INVALID_PLUGIN_HANDLE: PluginHandle = u32::MAX;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginInfo {
    pub info_version: u32,
    /// Use a raw pointer to represent a C-style string
    pub name: *const core::ffi::c_char,
    pub version: u32,
}

impl Default for PluginInfo {
    fn default() -> Self {
        Self {
            info_version: Default::default(),
            name: unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"") }.as_ptr(),
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
    pub query_interface: unsafe fn(u32) -> *mut std::ffi::c_void,
    pub get_plugin_handle: unsafe fn() -> PluginHandle,
    pub get_release_index: unsafe fn() -> u32,
    pub get_plugin_info: unsafe fn(*const core::ffi::c_char) -> *const core::ffi::c_void,
}

#[repr(C)]
pub struct SKSEMessagingInterface {
    pub interface_version: u32,
    pub register_listener: unsafe fn(PluginHandle, *const u8, *mut std::ffi::c_void) -> bool,
    pub dispatch: unsafe fn(PluginHandle, u32, *mut std::ffi::c_void, u32, *const u8) -> bool,
    pub get_event_dispatcher: unsafe fn(u32) -> *mut std::ffi::c_void,
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
    pub register: unsafe fn(*const u8, *mut std::ffi::c_void) -> bool,
    pub register_for_inventory: unsafe fn(*mut std::ffi::c_void),
}

#[repr(C)]
pub struct SKSESerializationInterface {
    pub version: u32,
    pub set_unique_id: unsafe fn(PluginHandle, u32),
    pub set_revert_callback: unsafe fn(PluginHandle, *mut std::ffi::c_void),
    pub set_save_callback: unsafe fn(PluginHandle, *mut std::ffi::c_void),
    pub set_load_callback: unsafe fn(PluginHandle, *mut std::ffi::c_void),
    pub set_form_delete_callback: unsafe fn(PluginHandle, *mut std::ffi::c_void),
    pub write_record: unsafe fn(u32, u32, *const std::ffi::c_void, u32) -> bool,
    pub open_record: unsafe fn(u32, u32) -> bool,
    pub write_record_data: unsafe fn(*const std::ffi::c_void, u32) -> bool,
    pub get_next_record_info: unsafe fn(*mut u32, *mut u32, *mut u32) -> bool,
    pub read_record_data: unsafe fn(*mut std::ffi::c_void, u32) -> u32,
    pub resolve_handle: unsafe fn(u64, *mut u64) -> bool,
    pub resolve_form_id: unsafe fn(u32, *mut u32) -> bool,
}

#[repr(C)]
pub struct SKSETaskInterface {
    pub interface_version: u32,
    pub add_task: unsafe fn(*mut std::ffi::c_void),
    pub add_ui_task: unsafe fn(*mut std::ffi::c_void),
}

#[repr(C)]
pub struct SKSETrampolineInterface {
    pub interface_version: u32,
    pub allocate_from_branch_pool: unsafe fn(PluginHandle, usize) -> *mut std::ffi::c_void,
    pub allocate_from_local_pool: unsafe fn(PluginHandle, usize) -> *mut std::ffi::c_void,
}

pub struct SKSEDelayFunctorManager;
pub struct SKSEObjectRegistry;
pub struct SKSEPersistentObjectStorage;

pub struct TaskDelegate;
pub struct UiDelegateV1;

impl TaskDelegate {
    pub const fn run(&self) {}
    pub const fn dispose(&self) {}
}

impl UiDelegateV1 {
    pub const fn run(&self) {}
    pub const fn dispose(&self) {}
}
