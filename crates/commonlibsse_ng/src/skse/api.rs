use crate::re::BSTEventSource;
use crate::rel::version::Version;
use crate::skse::impls::stab::{
    PluginHandle, SKSEDelayFunctorManager, SKSEObjectRegistry, SKSEPersistentObjectStorage,
};
use crate::skse::interfaces::{
    PluginVersionData,
    load::LoadInterface,
    messaging::{self, MessagingInterface},
    object::ObjectInterface,
    papyrus::PapyrusInterface,
    scaleform::ScaleformInterface,
    serialization::SerializationInterface,
    task::TaskInterface,
    trampoline::TrampolineInterface,
};
use snafu::Snafu;
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Snafu)]
pub enum ApiStorageError {
    /// Global API storage has not yet been initialized. We must call `skse::init` first in the `SKSE_PluginLoad` function.
    Uninitialized,

    /// Could not find the definition of the `SKSEPlugin_Version` symbol in this SKSE plugin dll. Therefore, the plugin information could not be retrieved.
    MissingSymbolSKSEPluginVersion,
}

#[derive(Debug)]
pub struct ModCallbackEvent;
#[derive(Debug)]
pub struct CameraEvent;
#[derive(Debug)]
pub struct CrosshairRefEvent;
#[derive(Debug)]
pub struct ActionEvent;
#[derive(Debug)]
pub struct NiNodeUpdateEvent;

#[derive(Debug)]
pub struct APIStorage {
    /// Your SKSE Plugin name
    ///
    /// If the `SKSEPlugin_Version` symbol is not defined, [`Option::None`] is always included.
    pub plugin_name: Option<&'static str>,
    /// Your SKSE Plugin author name
    ///
    /// If the `SKSEPlugin_Version` symbol is not defined, [`Option::None`] is always included.
    pub plugin_author: Option<&'static str>,
    /// Your SKSE Plugin version
    ///
    /// If the `SKSEPlugin_Version` symbol is not defined, [`Option::None`] is always included.
    pub plugin_version: Option<Version>,

    /// The plugin handle (index of how many dlls SKSE has loaded) of this SKSE plugin dll.
    pub plugin_handle: PluginHandle,
    pub release_index: u32,

    pub scaleform_interface: ScaleformInterface,
    pub papyrus_interface: PapyrusInterface,
    pub serialization_interface: SerializationInterface,
    pub task_interface: TaskInterface,
    pub trampoline_interface: TrampolineInterface,

    pub messaging_interface: MessagingInterface,
    pub mod_callback_event_source: *mut BSTEventSource<ModCallbackEvent>,
    pub camera_event_source: *mut BSTEventSource<CameraEvent>,
    pub crosshair_ref_event_source: *mut BSTEventSource<CrosshairRefEvent>,
    pub action_event_source: *mut BSTEventSource<ActionEvent>,
    pub ni_node_update_event_source: *mut BSTEventSource<NiNodeUpdateEvent>,

    pub object_interface: ObjectInterface,
    pub delay_functor_manager: *mut SKSEDelayFunctorManager,
    pub object_registry: *mut SKSEObjectRegistry,
    pub persistent_object_storage: *mut SKSEPersistentObjectStorage,
}

unsafe impl Send for APIStorage {}
unsafe impl Sync for APIStorage {}

static INSTANCE: OnceLock<APIStorage> = OnceLock::new();

impl APIStorage {
    /// Returns a reference to the `APIStorage` instance.
    ///
    /// # Errors
    /// Returns an error if the `APIStorage` is not initialized.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::skse::api::APIStorage;
    /// let storage = APIStorage::get();
    /// ```
    #[inline]
    pub fn get() -> Result<&'static Self, ApiStorageError> {
        INSTANCE.get().ok_or(ApiStorageError::Uninitialized)
    }

    /// Maps over the `APIStorage` instance if it exists and returns a result of `Option<R>`.
    ///
    /// # Errors
    /// Returns an error if the `APIStorage` is not initialized.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::skse::api::APIStorage;
    /// let result = APIStorage::map(|storage| storage.plugin_version.clone());
    /// ```
    pub fn map<F, R>(f: F) -> Result<R, ApiStorageError>
    where
        F: FnOnce(&Self) -> R,
    {
        Self::get().map(f)
    }
}

/// Initializes global API interfaces through the `QueryInterface` method of `LoadInterface`.
/// Ensures all interfaces are initialized before use.
///
/// # Example
/// ```
/// #[unsafe(no_mangle)]
/// pub extern "C" fn SKSEPlugin_Load(skse: &commonlibsse_ng::skse::interfaces::load::LoadInterface) -> bool {
///     commonlibsse_ng::skse::init(skse);
///     true
/// }
/// ```
pub fn init(load_interface: &LoadInterface) {
    let plugin_handle = load_interface.get_plugin_handle();
    let release_index = load_interface.get_release_index();

    let scaleform_interface = ScaleformInterface::new(load_interface.query_interface());
    let papyrus_interface = PapyrusInterface::new(load_interface.query_interface());
    let serialization_interface = SerializationInterface::new(load_interface.query_interface());
    let task_interface = TaskInterface::new(load_interface.query_interface());
    let trampoline_interface = TrampolineInterface::new(load_interface.query_interface());
    let messaging_interface = MessagingInterface::new(load_interface.query_interface());

    let mod_callback_event_source =
        messaging_interface.get_event_dispatcher(messaging::Dispatcher::ModEvent);

    let camera_event_source =
        messaging_interface.get_event_dispatcher(messaging::Dispatcher::CameraEvent);

    let crosshair_ref_event_source =
        messaging_interface.get_event_dispatcher(messaging::Dispatcher::CrosshairEvent);

    let action_event_source =
        messaging_interface.get_event_dispatcher(messaging::Dispatcher::ActionEvent);

    let ni_node_update_event_source =
        messaging_interface.get_event_dispatcher(messaging::Dispatcher::NiNodeUpdateEvent);

    let object_interface = ObjectInterface::new(load_interface.query_interface());
    let delay_functor_manager = object_interface.get_delay_functor_manager();
    let object_registry = object_interface.get_object_registry();
    let persistent_object_storage = object_interface.get_persistent_object_storage();

    let (plugin_name, plugin_author, plugin_version) =
        PluginVersionData::get_singleton().map_or((None, None, None), |plugin_ver| {
            (
                Some(plugin_ver.get_author_name()),
                Some(plugin_ver.get_author_name()),
                Some(Version::unpack(plugin_ver.get_plugin_version())),
            )
        });

    // ignore double insert
    let _ = INSTANCE.set(APIStorage {
        plugin_name,
        plugin_author,
        plugin_version,

        plugin_handle,
        release_index,

        scaleform_interface,
        papyrus_interface,
        serialization_interface,
        task_interface,
        trampoline_interface,

        messaging_interface,
        mod_callback_event_source: mod_callback_event_source.cast(),
        camera_event_source: camera_event_source.cast(),
        crosshair_ref_event_source: crosshair_ref_event_source.cast(),
        action_event_source: action_event_source.cast(),
        ni_node_update_event_source: ni_node_update_event_source.cast(),

        object_interface,
        delay_functor_manager,
        object_registry,
        persistent_object_storage,
    });
}

/// Get the plugin's name.
///
/// # Errors
/// - If the internal global API storage is uninitialized because forgot to call `skse::init`
/// - Returns an error if forgot to define the `SKSEPlugin_Version` symbol in this SKSE plugin dll
///
/// # Example
/// ```
/// use commonlibsse_ng::skse::api::{get_plugin_name};
/// match get_plugin_name() {
///     Ok(name) => println!("Plugin name: {}", name),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
#[inline]
pub fn get_plugin_name() -> Result<&'static str, ApiStorageError> {
    APIStorage::get()?.plugin_name.ok_or(ApiStorageError::MissingSymbolSKSEPluginVersion)
}

/// Get the plugin's author.
///
/// # Errors
/// - If the internal global API storage is uninitialized because forgot to call `skse::init`
/// - Returns an error if forgot to define the `SKSEPlugin_Version` symbol in this SKSE plugin dll
///
/// # Example
/// ```
/// use commonlibsse_ng::skse::api::{get_plugin_author, ApiStorageError};
///
/// assert_eq!(get_plugin_author(), Err(ApiStorageError::Uninitialized));
/// ```
#[inline]
pub fn get_plugin_author() -> Result<&'static str, ApiStorageError> {
    APIStorage::get()?.plugin_author.ok_or(ApiStorageError::MissingSymbolSKSEPluginVersion)
}

/// Get the plugin's version.
///
/// # Errors
/// - If the internal global API storage is uninitialized because forgot to call `skse::init`
/// - Returns an error if forgot to define the `SKSEPlugin_Version` symbol in this SKSE plugin dll
///
/// # Example
/// ```
/// use commonlibsse_ng::skse::api::get_plugin_version;
///
/// match get_plugin_version() {
///     Ok(version) => println!("Plugin version: {:?}", version),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
#[inline]
pub fn get_plugin_version() -> Result<Version, ApiStorageError> {
    APIStorage::get()?.plugin_version.clone().ok_or(ApiStorageError::MissingSymbolSKSEPluginVersion)
}

/// Get the plugin handle (index of how many dlls SKSE has loaded) of this SKSE plugin dll.
///
/// # Errors
/// If the internal global API storage is uninitialized because forgot to call `skse::init`
///
/// # Example
/// ```
/// use commonlibsse_ng::skse::api::get_plugin_handle;
///
/// match get_plugin_handle() {
///     Ok(handle) => println!("Plugin handle(dll index): {:?}", handle),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
#[inline]
pub fn get_plugin_handle() -> Result<PluginHandle, ApiStorageError> {
    APIStorage::map(|storage| storage.plugin_handle.clone())
}

/// Retrieves the release index.
///
/// # Errors
/// If the internal global API storage is uninitialized because forgot to call `skse::init`
///
/// # Example
/// ```
/// use commonlibsse_ng::skse::api::get_release_index;
///
/// match get_release_index() {
///     Ok(index) => println!("Release index: {}", index),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
#[inline]
pub fn get_release_index() -> Result<u32, ApiStorageError> {
    APIStorage::map(|storage| storage.release_index)
}

/// Retrieves the `ScaleformInterface` instance.
///
/// # Errors
/// If the internal global API storage is uninitialized because forgot to call `skse::init`
///
/// # Example
/// ```
/// use commonlibsse_ng::skse::api::get_scaleform_interface;
///
/// match get_scaleform_interface() {
///     Ok(interface) => println!("Scaleform Interface: {:?}", interface),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
#[inline]
pub fn get_scaleform_interface() -> Result<ScaleformInterface, ApiStorageError> {
    APIStorage::map(|storage| storage.scaleform_interface.clone())
}

/// Retrieves the `PapyrusInterface` instance.
///
/// # Errors
/// If the internal global API storage is uninitialized because forgot to call `skse::init`
///
/// # Example
/// ```
/// use commonlibsse_ng::skse::api::get_papyrus_interface;
///
/// match get_papyrus_interface() {
///     Ok(interface) => println!("Papyrus Interface: {:?}", interface),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
#[inline]
pub fn get_papyrus_interface() -> Result<PapyrusInterface, ApiStorageError> {
    APIStorage::map(|storage| storage.papyrus_interface.clone())
}

/// Retrieves the `SerializationInterface` instance.
///
/// # Errors
/// If the internal global API storage is uninitialized because forgot to call `skse::init`
///
/// # Example
/// ```
/// use commonlibsse_ng::skse::api::get_serialization_interface;
///
/// match get_serialization_interface() {
///     Ok(interface) => println!("Serialization Interface: {:?}", interface),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
#[inline]
pub fn get_serialization_interface() -> Result<SerializationInterface, ApiStorageError> {
    APIStorage::map(|storage| storage.serialization_interface.clone())
}

/// Retrieves the `TaskInterface` instance.
///
/// # Errors
/// If the internal global API storage is uninitialized because forgot to call `skse::init`
///
/// # Example
/// ```
/// use commonlibsse_ng::skse::api::get_task_interface;
///
/// match get_task_interface() {
///     Ok(interface) => println!("Task Interface: {:?}", interface),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
#[inline]
pub fn get_task_interface() -> Result<TaskInterface, ApiStorageError> {
    APIStorage::map(|storage| storage.task_interface.clone())
}

/// Retrieves the `MessagingInterface` instance.
///
/// # Errors
/// If the internal global API storage is uninitialized because forgot to call `skse::init`
///
/// # Example
/// ```
/// use commonlibsse_ng::skse::api::get_messaging_interface;
///
/// match get_messaging_interface() {
///     Ok(interface) => println!("Messaging Interface: {:?}", interface),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
#[inline]
pub fn get_messaging_interface() -> Result<MessagingInterface, ApiStorageError> {
    APIStorage::map(|storage| storage.messaging_interface.clone())
}
