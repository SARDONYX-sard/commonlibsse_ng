use core::ptr;
use std::sync::{OnceLock, RwLock};

use crate::re::BSTEventSource;
use crate::rel::version::Version;
use crate::skse::impls::stab::{
    PluginHandle, SKSEDelayFunctorManager, SKSEObjectRegistry, SKSEPersistentObjectStorage,
};
use crate::skse::interfaces::{
    load::LoadInterface,
    messaging::{self, MessagingInterface},
    object::ObjectInterface,
    papyrus::PapyrusInterface,
    scaleform::ScaleformInterface,
    serialization::SerializationInterface,
    task::TaskInterface,
    trampoline::TrampolineInterface,
};
use crate::skse::trampoline::get_trampoline;

use super::interfaces::PluginVersionData;

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

#[derive(Debug)]
pub struct APIStorage {
    plugin_name: &'static str,
    plugin_author: &'static str,
    pub plugin_version: Version,

    plugin_handle: PluginHandle,
    pub release_index: u32,

    pub scaleform_interface: ScaleformInterface,
    pub papyrus_interface: PapyrusInterface,
    pub serialization_interface: SerializationInterface,
    pub task_interface: TaskInterface,
    trampoline_interface: TrampolineInterface,

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

    // api_init: bool,
    api_init_regs: Vec<Box<dyn ApiInitRegFn>>,
}

pub trait ApiInitRegFn: Fn() + Send + Sync + core::fmt::Debug {}

unsafe impl Send for APIStorage {}
unsafe impl Sync for APIStorage {}

impl APIStorage {
    fn get() -> &'static RwLock<Option<Self>> {
        static INSTANCE: OnceLock<RwLock<Option<APIStorage>>> = OnceLock::new();
        INSTANCE.get_or_init(|| RwLock::new(None))
    }

    fn map<F, R>(f: F) -> Option<R>
    where
        F: FnOnce(&Self) -> R,
    {
        Self::get().read().ok().and_then(|guard| guard.as_ref().map(f))
    }
}

/// Stores a global reference to each Interface through the `QueryInterface` of `LoadInterface` (`SKSEInterface`) into a global variable.
///
/// If this is not done first, simple access to each interface will remain uninitialized and unusable.
///
/// # Panics
/// If the thread acquiring the lock panics.
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

    let mut guard = APIStorage::get().write().unwrap();

    if let Some(storage) = &mut *guard {
        for reg in storage.api_init_regs.drain(..) {
            reg();
        }
    }

    let (plugin_name, plugin_author, plugin_version) =
        PluginVersionData::get_singleton().map_or(("", "", 0), |plugin_ver| {
            (
                plugin_ver.get_author_name(),
                plugin_ver.get_author_name(),
                plugin_ver.get_plugin_version(),
            )
        });

    *guard = Some(APIStorage {
        plugin_name,
        plugin_author,
        plugin_version: Version::unpack(plugin_version),

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

        api_init_regs: Vec::new(),
    });
}

/// # Panics
pub fn register_for_api_init_event<F: ApiInitRegFn + 'static>(f: F) {
    let mut guard = APIStorage::get().write().unwrap();

    if let Some(storage) = &mut *guard {
        storage.api_init_regs.push(Box::new(f));
    }
}

#[inline]
pub fn get_plugin_name() -> Option<&'static str> {
    APIStorage::map(|storage| storage.plugin_name)
}

#[inline]
pub fn get_plugin_author() -> Option<&'static str> {
    APIStorage::map(|storage| storage.plugin_author)
}

#[inline]
pub fn get_plugin_version() -> Option<Version> {
    APIStorage::map(|storage| storage.plugin_version.clone())
}

/// # Panics
#[inline]
pub fn get_plugin_handle() -> PluginHandle {
    APIStorage::map(|storage| storage.plugin_handle.clone()).expect("Plugin handle not found")
}

/// # Panics
#[inline]
pub fn get_release_index() -> u32 {
    APIStorage::map(|storage| storage.release_index).unwrap()
}

/// # Panics
#[inline]
pub fn get_scaleform_interface() -> Option<ScaleformInterface> {
    APIStorage::map(|storage| storage.scaleform_interface.clone())
}

/// # Panics
#[inline]
pub fn get_papyrus_interface() -> Option<PapyrusInterface> {
    APIStorage::map(|storage| storage.papyrus_interface.clone())
}

/// # Panics
#[inline]
pub fn get_serialization_interface() -> Option<SerializationInterface> {
    APIStorage::map(|storage| storage.serialization_interface.clone())
}

/// # Panics
#[inline]
pub fn get_task_interface() -> Option<TaskInterface> {
    APIStorage::map(|storage| storage.task_interface.clone())
}

/// # Panics
#[inline]
pub fn get_messaging_interface() -> Option<MessagingInterface> {
    APIStorage::map(|storage| storage.messaging_interface.clone())
}

/// # Errors
pub fn alloc_trampoline(size: usize, try_skse_reserve: bool) -> Result<(), AllocTrampolineError> {
    let trampoline = get_trampoline();

    if try_skse_reserve {
        let ret = APIStorage::map(|storage| {
            let memory = storage.trampoline_interface.allocate_from_branch_pool(size);
            if memory.is_null() {
                let mut guard = trampoline
                    .write()
                    .map_err(|_| AllocTrampolineError::TrampolineLockIsPoisoned)?;
                unsafe { guard.set_trampoline(memory.cast::<u8>(), size, None) };
            }

            Ok(())
        });

        if let Some(r) = ret {
            return r;
        }
        return Ok(());
    }

    {
        let mut guard =
            trampoline.write().map_err(|_| AllocTrampolineError::TrampolineLockIsPoisoned)?;
        guard.create(size, ptr::null_mut())?;
    }

    Ok(())
}

#[derive(Debug, snafu::Snafu)]
pub enum AllocTrampolineError {
    /// The thread that was getting Trampoline's lock panicked.
    TrampolineLockIsPoisoned,

    #[snafu(transparent)]
    TrampolineError { source: super::trampoline::TrampolineError },
}
