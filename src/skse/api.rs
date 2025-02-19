use std::sync::{Mutex, OnceLock};

use crate::{re::BSTEventSource, rel::version::Version, skse::impls::stab::INVALID_PLUGIN_HANDLE};

use super::{
    impls::stab::{
        PluginHandle, SKSEDelayFunctorManager, SKSEObjectRegistry, SKSEPersistentObjectStorage,
    },
    interfaces::{
        load::{LoadInterface, LoadInterfaceEnum},
        messaging::MessagingInterface,
        object::ObjectInterface,
        papyrus::PapyrusInterface,
        scaleform::ScaleformInterface,
        serialization::SerializationInterface,
        task::TaskInterface,
        TrampolineInterface,
    },
};

// Placeholder for various SKSE interfaces

// Event source stubs
pub struct ModCallbackEvent;
pub struct CameraEvent;
pub struct CrosshairRefEvent;
pub struct ActionEvent;
pub struct NiNodeUpdateEvent;

struct APIStorage {
    plugin_name: Option<String>,
    plugin_author: Option<String>,
    plugin_version: Option<Version>,

    plugin_handle: PluginHandle,
    release_index: u32,

    scaleform_interface: Option<&'static ScaleformInterface>,
    papyrus_interface: Option<&'static PapyrusInterface>,
    serialization_interface: Option<&'static SerializationInterface>,
    task_interface: Option<&'static TaskInterface>,
    trampoline_interface: Option<&'static TrampolineInterface>,

    messaging_interface: Option<&'static MessagingInterface>,
    mod_callback_event_source: Option<&'static BSTEventSource<ModCallbackEvent>>,
    camera_event_source: Option<&'static BSTEventSource<CameraEvent>>,
    crosshair_ref_event_source: Option<&'static BSTEventSource<CrosshairRefEvent>>,
    action_event_source: Option<&'static BSTEventSource<ActionEvent>>,
    ni_node_update_event_source: Option<&'static BSTEventSource<NiNodeUpdateEvent>>,

    object_interface: Option<&'static ObjectInterface>,
    delay_functor_manager: Option<&'static SKSEDelayFunctorManager>,
    object_registry: Option<&'static SKSEObjectRegistry>,
    persistent_object_storage: Option<&'static SKSEPersistentObjectStorage>,

    api_init: bool,
    api_init_regs: Vec<Box<dyn Fn() + Send + Sync>>,
}

unsafe impl Send for APIStorage {}
unsafe impl Sync for APIStorage {}

impl APIStorage {
    fn get() -> &'static Mutex<APIStorage> {
        static INSTANCE: OnceLock<Mutex<APIStorage>> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            Mutex::new(APIStorage {
                plugin_name: None,
                plugin_author: None,
                plugin_version: None,

                plugin_handle: INVALID_PLUGIN_HANDLE,
                release_index: 0,

                scaleform_interface: None,
                papyrus_interface: None,
                serialization_interface: None,
                task_interface: None,
                trampoline_interface: None,

                messaging_interface: None,
                mod_callback_event_source: None,
                camera_event_source: None,
                crosshair_ref_event_source: None,
                action_event_source: None,
                ni_node_update_event_source: None,

                object_interface: None,
                delay_functor_manager: None,
                object_registry: None,
                persistent_object_storage: None,

                api_init: false,
                api_init_regs: Vec::new(),
            })
        })
    }
}

unsafe fn init(load_interface: &LoadInterface, a_log: bool) {
    let mut storage = APIStorage::get().lock().unwrap();
    if storage.api_init {
        return;
    }

    storage.plugin_handle = load_interface.get_plugin_handle();
    storage.release_index = load_interface.get_release_index();

    storage.scaleform_interface =
        load_interface.query_interface(LoadInterfaceEnum::ScaleForm as u32);
    storage.papyrus_interface = load_interface.query_interface(LoadInterfaceEnum::Papyrus as u32);
    storage.serialization_interface =
        load_interface.query_interface(LoadInterfaceEnum::Serialization as u32);
    storage.task_interface = load_interface.query_interface(LoadInterfaceEnum::Task as u32);
    storage.trampoline_interface =
        load_interface.query_interface(LoadInterfaceEnum::Trampoline as u32);

    if let Some(messaging_interface) =
        load_interface.query_interface(LoadInterfaceEnum::Messaging as u32)
    {
        storage.messaging_interface = Some(messaging_interface);
        storage.mod_callback_event_source =
            messaging_interface.get_event_dispatcher(MessagingInterface::MOD_EVENT);
        storage.camera_event_source =
            messaging_interface.get_event_dispatcher(MessagingInterface::CAMERA_EVENT);
        storage.crosshair_ref_event_source =
            messaging_interface.get_event_dispatcher(MessagingInterface::CROSSHAIR_EVENT);
        storage.action_event_source =
            messaging_interface.get_event_dispatcher(MessagingInterface::ACTION_EVENT);
        storage.ni_node_update_event_source =
            messaging_interface.get_event_dispatcher(MessagingInterface::NI_NODE_UPDATE_EVENT);
    }

    if let Some(object_interface) = load_interface.query_interface(LoadInterfaceEnum::Object as u32)
    {
        storage.object_interface = Some(object_interface);
        storage.delay_functor_manager = Some(object_interface.get_delay_functor_manager());
        storage.object_registry = Some(object_interface.get_object_registry());
        storage.persistent_object_storage = Some(object_interface.get_persistent_object_storage());
    }

    storage.api_init = true;
    for reg in storage.api_init_regs.drain(..) {
        reg();
    }
}

static INIT_MUTEX: Mutex<()> = Mutex::new(());

pub fn register_for_api_init_event<F: Fn() + 'static>(_callback: F) {
    // Register callback logic
}

#[cfg(feature = "skyrim_ae")]
pub fn get_plugin_name() -> &'static str {
    "MyPlugin"
}

#[cfg(feature = "skyrim_ae")]
pub fn get_plugin_author() -> &'static str {
    "Author Name"
}

#[cfg(feature = "skyrim_ae")]
pub fn get_plugin_version() -> (u32, u32, u32) {
    (1, 0, 0)
}

pub fn get_plugin_handle() -> PluginHandle {
    PluginHandle(0) // Example handle
}

pub fn get_release_index() -> u32 {
    0 // Example release index
}

pub fn get_scaleform_interface() -> Option<&'static ScaleformInterface> {
    None
}

pub fn get_papyrus_interface() -> Option<&'static PapyrusInterface> {
    None
}

pub fn get_serialization_interface() -> Option<&'static SerializationInterface> {
    None
}

pub fn get_task_interface() -> Option<&'static TaskInterface> {
    None
}

pub fn get_trampoline_interface() -> Option<&'static TrampolineInterface> {
    None
}

pub fn get_messaging_interface() -> Option<&'static MessagingInterface> {
    None
}

pub fn get_mod_callback_event_source() -> Option<&'static BSTEventSource<ModCallbackEvent>> {
    None
}

pub fn get_camera_event_source() -> Option<&'static BSTEventSource<CameraEvent>> {
    None
}

pub fn get_crosshair_ref_event_source() -> Option<&'static BSTEventSource<CrosshairRefEvent>> {
    None
}

pub fn get_action_event_source() -> Option<&'static BSTEventSource<ActionEvent>> {
    None
}

pub fn get_ni_node_update_event_source() -> Option<&'static BSTEventSource<NiNodeUpdateEvent>> {
    None
}

pub fn get_object_interface() -> Option<&'static ObjectInterface> {
    None
}

pub fn get_delay_functor_manager() -> Option<&'static SKSEDelayFunctorManager> {
    None
}

pub fn get_object_registry() -> Option<&'static SKSEObjectRegistry> {
    None
}

pub fn get_persistent_object_storage() -> Option<&'static SKSEPersistentObjectStorage> {
    None
}

pub fn alloc_trampoline(_size: usize, _try_skse_reserve: bool) {
    // Allocate trampoline logic
}
