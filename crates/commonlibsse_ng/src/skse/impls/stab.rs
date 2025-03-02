// https://github.com/alandtse/CommonLibVR/blob/ng/include/SKSE/Impl/Stubs.h
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_void};

/// Represents a handle for a plugin, which is a 32-bit unsigned integer.
///
/// Internally implemented, this number represents the index of the order in which plugins are loaded.
#[repr(transparent)]
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginHandle(pub u32);

/// The invalid plugin handle value, set to the maximum possible `u32` value.
pub const INVALID_PLUGIN_HANDLE: PluginHandle = PluginHandle(u32::MAX);

/// Structure representing the information about a plugin.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginInfo {
    /// The version of the plugin information structure.
    pub infoVersion: u32,
    /// A pointer to a C-style string containing the plugin's name.
    pub name: *const c_char,
    /// The version of the plugin.
    pub version: u32,
}

impl PluginInfo {
    /// Plugin information version.
    pub const VERSION: u32 = 1;
}

impl Default for PluginInfo {
    /// Creates a new instance of `PluginInfo` with default values.
    fn default() -> Self {
        Self { infoVersion: 0, name: c"".as_ptr(), version: 0 }
    }
}

/// The main SKSE interface structure that provides access to various SKSE functionality.
#[repr(C)]
#[derive(Debug)]
pub struct SKSEInterface {
    /// The version of SKSE being used.
    pub(crate) skseVersion: u32,
    /// The version of the runtime environment.
    pub(crate) runtimeVersion: u32,
    /// The version of the editor environment.
    pub(crate) editorVersion: u32,
    /// A flag indicating whether the environment is the editor or not.
    pub(crate) isEditor: u32,
    /// Function for querying interface.
    pub(crate) QueryInterface: unsafe extern "C" fn(u32) -> *mut c_void,
    /// Function to get the plugin handle.
    pub(crate) GetPluginHandle: unsafe extern "C" fn() -> PluginHandle,
    /// Function to get the release index.
    pub(crate) GetReleaseIndex: unsafe extern "C" fn() -> u32,
    /// Function to get the plugin information.
    pub(crate) GetPluginInfo: unsafe extern "C" fn(*const c_char) -> *const PluginInfo,
}

/// Available for `SKSEInterface::QueryInterface` id.
#[repr(u32)]
#[derive(Debug)]
pub enum InterfaceKind {
    Invalid = 0,
    ScaleForm,
    Papyrus,
    Serialization,
    Task,
    Messaging,
    Object,
    Trampoline,
    /// Max
    Total,
}

/// The messaging interface for SKSE that allows plugin communication.
#[repr(C)]
#[derive(Debug)]
pub struct SKSEMessagingInterface {
    /// The version of the messaging interface.
    pub interfaceVersion: u32,
    /// Registers a listener for messages.
    pub RegisterListener: unsafe extern "C" fn(PluginHandle, *const c_char, *mut c_void) -> bool,
    /// Dispatches a message.
    pub Dispatch: unsafe extern "C" fn(PluginHandle, u32, *mut c_void, u32, *const c_char) -> bool,
    /// Gets the event dispatcher for the messaging interface.
    pub GetEventDispatcher: unsafe extern "C" fn(u32) -> *mut c_void,
}

/// Interface for interacting with SKSE objects.
#[repr(C)]
#[derive(Debug)]
pub struct SKSEObjectInterface {
    /// The version of the interface.
    pub interfaceVersion: u32,
    /// Retrieves the delay functor manager.
    pub GetDelayFunctorManager: unsafe extern "C" fn() -> *mut SKSEDelayFunctorManager,
    /// Retrieves the object registry.
    pub GetObjectRegistry: unsafe extern "C" fn() -> *mut SKSEObjectRegistry,
    /// Retrieves the persistent object storage.
    pub GetPersistentObjectStorage: unsafe extern "C" fn() -> *mut SKSEPersistentObjectStorage,
}

/// Interface for registering papyrus scripts.
#[repr(C)]
#[derive(Debug)]
pub struct SKSEPapyrusInterface {
    /// The version of the interface.
    pub interfaceVersion: u32,
    /// Registers a papyrus script.
    pub Register: unsafe extern "C" fn(*mut c_void) -> bool,
}

/// Interface for interacting with Scaleform movies in SKSE.
#[repr(C)]
#[derive(Debug)]
pub struct SKSEScaleformInterface {
    /// The version of the Scaleform interface.
    pub interfaceVersion: u32,
    /// Registers a Scaleform movie.
    pub Register: unsafe extern "C" fn(*const c_char, *mut c_void) -> bool,
    /// Registers a Scaleform movie for inventory purposes.
    pub RegisterForInventory: unsafe extern "C" fn(*mut c_void),
}

/// Interface for managing serialization in SKSE.
#[repr(C)]
#[derive(Debug)]
pub struct SKSESerializationInterface {
    /// The version of the serialization interface.
    pub version: u32,
    /// Sets a unique identifier for a plugin.
    pub SetUniqueId: unsafe extern "C" fn(PluginHandle, u32),
    /// Sets the callback for reverting changes.
    pub SetRevertCallback: unsafe extern "C" fn(PluginHandle, *mut c_void),
    /// Sets the callback for saving data.
    pub SetSaveCallback: unsafe extern "C" fn(PluginHandle, *mut c_void),
    /// Sets the callback for loading data.
    pub SetLoadCallback: unsafe extern "C" fn(PluginHandle, *mut c_void),
    /// Sets the callback for handling form deletions.
    pub SetFormDeleteCallback: unsafe extern "C" fn(PluginHandle, *mut c_void),
    /// Writes a record to the serialization system.
    pub WriteRecord: unsafe extern "C" fn(u32, u32, *const c_void, u32) -> bool,
    /// Opens a record for reading or writing.
    pub OpenRecord: unsafe extern "C" fn(u32, u32) -> bool,
    /// Writes data to a record.
    pub WriteRecordData: unsafe extern "C" fn(*const c_void, u32) -> bool,
    /// Retrieves information for the next record.
    pub GetNextRecordInfo: unsafe extern "C" fn(*mut u32, *mut u32, *mut u32) -> bool,
    /// Reads data from a record.
    pub ReadRecordData: unsafe extern "C" fn(*mut c_void, u32) -> u32,
    /// Resolves a handle to a new value.
    pub ResolveHandle: unsafe extern "C" fn(u64, *mut u64) -> bool,
    /// Resolves a form ID to a new value.
    pub ResolveFormId: unsafe extern "C" fn(u32, *mut u32) -> bool,
}

/// Interface for managing tasks in SKSE.
#[repr(C)]
#[derive(Debug)]
pub struct SKSETaskInterface {
    /// The version of the task interface.
    pub interfaceVersion: u32,
    /// Adds a task to the task queue.
    pub AddTask: unsafe extern "C" fn(*mut c_void),
    /// Adds a UI task to the UI task queue.
    pub AddUiTask: unsafe extern "C" fn(*mut c_void),
}

/// Interface for managing trampoline functions in SKSE.
#[repr(C)]
#[derive(Debug)]
pub struct SKSETrampolineInterface {
    /// The version of the trampoline interface.
    pub interfaceVersion: u32,
    /// Allocates memory from the branch pool.
    pub AllocateFromBranchPool: unsafe extern "C" fn(PluginHandle, usize) -> *mut c_void,
    /// Allocates memory from the local pool.
    pub AllocateFromLocalPool: unsafe extern "C" fn(PluginHandle, usize) -> *mut c_void,
}

/// Dummy structure representing the delay functor manager.
#[repr(C)]
#[derive(Debug)]
pub struct SKSEDelayFunctorManager;

/// Dummy structure representing the object registry.
#[repr(C)]
#[derive(Debug)]
pub struct SKSEObjectRegistry;

/// Dummy structure representing persistent object storage.
#[repr(C)]
#[derive(Debug)]
pub struct SKSEPersistentObjectStorage;

/// A delegate structure for task execution in SKSE.
#[repr(C)]
#[derive(Debug)]
pub struct TaskDelegate {
    pub vtbl: *const TaskDelegateVirtualTable,
}

/// A delegate structure for UI task execution in SKSE.
#[repr(C)]
#[derive(Debug)]
pub struct UiDelegateV1 {
    pub vtbl: *const UiDelegateV1VirtualTable,
}

/// Virtual table for task delegate functions.
#[repr(C)]
pub struct TaskDelegateVirtualTable {
    /// Executes the task.
    pub Run: unsafe extern "C" fn(this: *const c_void),
    /// Disposes of the task delegate.
    pub Dispose: fn(this: *const c_void),
}

/// Virtual table for UI task delegate functions.
#[repr(C)]
pub struct UiDelegateV1VirtualTable {
    /// Executes the UI task.
    pub Run: fn(this: *const c_void),
    /// Disposes of the UI task delegate.
    pub Dispose: fn(this: *const c_void),
}
