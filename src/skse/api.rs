use std::sync::{Mutex, OnceLock};

use crate::{re::BSTEventSource, rel::version::Version, skse::impls::stab::INVALID_PLUGIN_HANDLE};

use super::{
    impls::stab::{
        PluginHandle, SKSEDelayFunctorManager, SKSEObjectRegistry, SKSEPersistentObjectStorage,
    },
    interfaces::{
        load::{LoadInterface, LoadInterfaceEnum},
        messaging::{self, MessagingInterface},
        object::ObjectInterface,
        papyrus::PapyrusInterface,
        scaleform::ScaleformInterface,
        serialization::SerializationInterface,
        task::TaskInterface,
        trampoline::TrampolineInterface,
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
    fn get() -> &'static Mutex<Self> {
        static INSTANCE: OnceLock<Mutex<APIStorage>> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            Mutex::new(Self {
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

unsafe fn init(load_interface: &LoadInterface) {
    use tracing::error;

    let mut storage = APIStorage::get().lock().unwrap();
    if storage.api_init {
        return;
    }

    storage.plugin_handle = load_interface.get_plugin_handle();
    storage.release_index = load_interface.get_release_index();

    let ptr = load_interface.query_interface(LoadInterfaceEnum::ScaleForm as u32);
    storage.scaleform_interface = if ptr.is_null() {
        error!("Failed to get ScaleformInterface");
        None
    } else {
        Some(&*ptr.cast::<ScaleformInterface>())
    };

    storage.papyrus_interface = {
        let ptr = load_interface.query_interface(LoadInterfaceEnum::Papyrus as u32);
        if ptr.is_null() {
            error!("Failed to get PapyrusInterface");
            None
        } else {
            Some(&*ptr.cast::<PapyrusInterface>())
        }
    };

    storage.serialization_interface = {
        let ptr = load_interface.query_interface(LoadInterfaceEnum::Serialization as u32);
        if ptr.is_null() {
            error!("Failed to get SerializationInterface");
            None
        } else {
            Some(&*ptr.cast::<SerializationInterface>())
        }
    };

    storage.task_interface = {
        let ptr = load_interface.query_interface(LoadInterfaceEnum::Task as u32);
        if ptr.is_null() {
            error!("Failed to get TaskInterface");
            None
        } else {
            Some(&*ptr.cast::<TaskInterface>())
        }
    };

    storage.trampoline_interface = {
        let ptr = load_interface.query_interface(LoadInterfaceEnum::Trampoline as u32);
        if ptr.is_null() {
            error!("Failed to get TrampolineInterface");
            None
        } else {
            Some(&*ptr.cast::<TrampolineInterface>())
        }
    };

    let messaging_ptr = load_interface.query_interface(LoadInterfaceEnum::Messaging as u32);
    if messaging_ptr.is_null() {
        error!("Failed to get MessagingInterface");
    } else {
        let messaging_interface = &*messaging_ptr.cast::<MessagingInterface>();
        storage.messaging_interface = Some(messaging_interface);

        let ptr = messaging_interface.get_event_dispatcher(messaging::Dispatcher::ModEvent);
        storage.mod_callback_event_source = if ptr.is_null() {
            error!("Failed to get BSTEventSource<ModCallbackEvent>");
            None
        } else {
            Some(&*(ptr as *const _))
        };

        let ptr = messaging_interface.get_event_dispatcher(messaging::Dispatcher::CameraEvent);
        storage.camera_event_source = if ptr.is_null() {
            error!("Failed to get BSTEventSource<CameraEvent>");
            None
        } else {
            Some(&*(ptr as *const _))
        };

        let ptr = messaging_interface.get_event_dispatcher(messaging::Dispatcher::CameraEvent);
        storage.camera_event_source = if ptr.is_null() {
            error!("Failed to get BSTEventSource<CameraEvent>");
            None
        } else {
            Some(&*(ptr as *const _))
        };

        let ptr = messaging_interface.get_event_dispatcher(messaging::Dispatcher::CrosshairEvent);
        storage.crosshair_ref_event_source = if ptr.is_null() {
            error!("Failed to get BSTEventSource<CrosshairEvent>");
            None
        } else {
            Some(&*(ptr as *const _))
        };

        let ptr = messaging_interface.get_event_dispatcher(messaging::Dispatcher::ActionEvent);
        storage.action_event_source = if ptr.is_null() {
            error!("Failed to get BSTEventSource<ActionEvent>");
            None
        } else {
            Some(&*(ptr as *const _))
        };

        let ptr =
            messaging_interface.get_event_dispatcher(messaging::Dispatcher::NiNodeUpdateEvent);
        storage.ni_node_update_event_source = if ptr.is_null() {
            error!("Failed to get BSTEventSource<NiNodeUpdateEvent>");
            None
        } else {
            Some(&*(ptr as *const _))
        };
    }

    let object_ptr = load_interface.query_interface(LoadInterfaceEnum::Object as u32);
    if object_ptr.is_null() {
        error!("Failed to get ObjectInterface");
    } else {
        let object_interface = &*object_ptr.cast::<ObjectInterface>();
        storage.object_interface = Some(object_interface);
        storage.delay_functor_manager = {
            let ptr = object_interface.get_delay_functor_manager();
            if ptr.is_null() {
                error!("Failed to get SKSEDelayFunctorManager");
                None
            } else {
                Some(&*ptr)
            }
        };
        storage.object_registry = {
            let ptr = object_interface.get_object_registry();
            if ptr.is_null() {
                error!("Failed to get SKSEDelayFunctorManager");
                None
            } else {
                Some(&*ptr)
            }
        };

        storage.persistent_object_storage = {
            let ptr = object_interface.get_persistent_object_storage();
            if ptr.is_null() {
                error!("Failed to get SKSEDelayFunctorManager");
                None
            } else {
                Some(&*ptr)
            }
        };
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
    APIStorage::get()
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
