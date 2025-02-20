use core::ptr;
use std::sync::{OnceLock, RwLock};

use crate::re::BSTEventSource;
use crate::rel::version::Version;
use crate::skse::impls::stab::{
    PluginHandle, SKSEDelayFunctorManager, SKSEObjectRegistry, SKSEPersistentObjectStorage,
    INVALID_PLUGIN_HANDLE,
};
use crate::skse::interfaces::{
    load::{LoadInterface, LoadInterfaceEnum},
    messaging::{self, MessagingInterface},
    object::ObjectInterface,
    papyrus::PapyrusInterface,
    scaleform::ScaleformInterface,
    serialization::SerializationInterface,
    task::TaskInterface,
    trampoline::TrampolineInterface,
};
use crate::skse::trampoline::get_trampoline;

// Placeholder for various SKSE interfaces

// Event source stubs
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

#[derive(Debug, Default)]
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
    api_init_regs: Vec<Box<dyn ApiInitRegFns>>,
}

trait ApiInitRegFns: Fn() + Send + Sync + core::fmt::Debug {}

unsafe impl Send for APIStorage {}
unsafe impl Sync for APIStorage {}

impl APIStorage {
    fn get() -> &'static RwLock<Self> {
        static INSTANCE: OnceLock<RwLock<APIStorage>> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            RwLock::new(Self {
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

/// # Safety
/// # Panics
#[allow(clippy::cognitive_complexity)]
pub unsafe fn init(load_interface: &LoadInterface) {
    use tracing::error;

    let mut storage = APIStorage::get().write().unwrap();
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

pub fn register_for_api_init_event<F: Fn() + 'static>(_callback: F) {
    // Register callback logic
}

/// # Panics
pub fn get_plugin_name() -> Option<String> {
    APIStorage::get().read().unwrap().plugin_name.clone()
}

/// # Panics
pub fn get_plugin_author() -> Option<String> {
    APIStorage::get().read().unwrap().plugin_author.clone()
}

/// # Panics
pub fn get_plugin_version() -> Option<Version> {
    APIStorage::get().read().unwrap().plugin_version.clone()
}

/// # Panics
pub fn get_plugin_handle() -> PluginHandle {
    APIStorage::get().read().unwrap().plugin_handle.clone()
}

/// # Panics
pub fn get_release_index() -> u32 {
    APIStorage::get().read().unwrap().release_index
}

/// # Panics
pub fn get_scaleform_interface() -> Option<&'static ScaleformInterface> {
    APIStorage::get().read().unwrap().scaleform_interface
}

/// # Panics
pub fn get_papyrus_interface() -> Option<&'static PapyrusInterface> {
    APIStorage::get().read().unwrap().papyrus_interface
}

/// # Panics
pub fn get_serialization_interface() -> Option<&'static SerializationInterface> {
    APIStorage::get().read().unwrap().serialization_interface
}

/// # Panics
pub fn get_task_interface() -> Option<&'static TaskInterface> {
    APIStorage::get().read().unwrap().task_interface
}

/// # Panics
pub fn get_trampoline_interface() -> Option<&'static TrampolineInterface> {
    APIStorage::get().read().unwrap().trampoline_interface
}

/// # Panics
pub fn get_messaging_interface() -> Option<&'static MessagingInterface> {
    APIStorage::get().read().unwrap().messaging_interface
}

/// # Panics
pub fn get_mod_callback_event_source() -> Option<&'static BSTEventSource<ModCallbackEvent>> {
    APIStorage::get().read().unwrap().mod_callback_event_source
}

/// # Panics
pub fn get_camera_event_source() -> Option<&'static BSTEventSource<CameraEvent>> {
    APIStorage::get().read().unwrap().camera_event_source
}

/// # Panics
pub fn get_crosshair_ref_event_source() -> Option<&'static BSTEventSource<CrosshairRefEvent>> {
    APIStorage::get().read().unwrap().crosshair_ref_event_source
}

/// # Panics
pub fn get_action_event_source() -> Option<&'static BSTEventSource<ActionEvent>> {
    APIStorage::get().read().unwrap().action_event_source
}

/// # Panics
pub fn get_ni_node_update_event_source() -> Option<&'static BSTEventSource<NiNodeUpdateEvent>> {
    APIStorage::get()
        .read()
        .unwrap()
        .ni_node_update_event_source
}

/// # Panics
pub fn get_object_interface() -> Option<&'static ObjectInterface> {
    APIStorage::get().read().unwrap().object_interface
}

/// # Panics
pub fn get_delay_functor_manager() -> Option<&'static SKSEDelayFunctorManager> {
    APIStorage::get().read().unwrap().delay_functor_manager
}

/// # Panics
pub fn get_object_registry() -> Option<&'static SKSEObjectRegistry> {
    APIStorage::get().read().unwrap().object_registry
}

/// # Panics
pub fn get_persistent_object_storage() -> Option<&'static SKSEPersistentObjectStorage> {
    APIStorage::get().read().unwrap().persistent_object_storage
}

/// # Panics
pub fn alloc_trampoline(size: usize, try_skse_reserve: bool) {
    let trampoline = get_trampoline();

    if try_skse_reserve {
        let interface = get_trampoline_interface();
        if let Some(interface) = interface {
            let memory = interface.allocate_from_branch_pool(size);
            if memory.is_null() {
                unsafe {
                    trampoline
                        .write()
                        .unwrap()
                        .set_trampoline(memory.cast::<u8>(), size, None);
                }
            }
        }

        return;
    }

    trampoline.write().unwrap().create(size, ptr::null_mut());
}
