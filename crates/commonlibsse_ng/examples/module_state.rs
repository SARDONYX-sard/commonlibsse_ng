use commonlibsse_ng::rel::version::Version;
use commonlibsse_ng::skse;
use commonlibsse_ng::skse::impls::stab::{PluginInfo, SKSEInterface};
use commonlibsse_ng::skse::interfaces::load::LoadInterface;
use commonlibsse_ng::skse::interfaces::query::QueryInterface;
use commonlibsse_ng::skse::interfaces::{PluginVersionData, VersionNumber, to_fixed_str};
use commonlibsse_ng::skse::version::RUNTIME_SSE_LATEST;

const PKG_NAME: &str = "module_state";
const PKG_VERSION: Version = Version::from_str_const(env!("CARGO_PKG_VERSION"));

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn SKSEPlugin_Query(skse: &SKSEInterface, info: &mut PluginInfo) -> bool {
    {
        *info = PluginInfo {
            version: PKG_VERSION.major() as u32,
            name: c"module_state".as_ptr(),
            infoVersion: 1,
        };
    };

    if QueryInterface::is_editor(skse) {
        #[cfg(feature = "tracing")]
        tracing::error!("The use of the SKSE Plugin within Editor is not supported.");
        false
    } else {
        true
    }
}

#[unsafe(no_mangle)]
#[allow(non_upper_case_globals)]
pub static SKSEPlugin_Version: PluginVersionData = {
    const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

    let mut compatible_versions = [0; 16];
    compatible_versions[0] = VersionNumber::from_version(RUNTIME_SSE_LATEST).to_packed();

    PluginVersionData {
        data_version: PKG_VERSION.major() as u32,
        plugin_version: VersionNumber::from_version(PKG_VERSION).to_packed(),
        plugin_name: to_fixed_str(PKG_NAME),
        author: to_fixed_str(PKG_AUTHORS),
        support_email: [0; 252],
        compatible_versions,
        version_independence: 1,
        version_independence_ex: 1,
        xse_minimum: 0,
    }
};

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn SKSEPlugin_Load(skse: &LoadInterface) -> bool {
    #[cfg(feature = "tracing")]
    {
        init_logger();
        tracing::info!("SKSEPlugin_Load has been called.");
    }

    let result = std::panic::catch_unwind(|| {
        #[cfg(feature = "tracing")]
        {
            use commonlibsse_ng::rel::module::ModuleState;
            if let Err(err) = ModuleState::map_or_init(|module| {
                tracing::info!("{module:#?}");
            }) {
                tracing::error!("{err}");
            };
        }

        skse::init(skse);

        if let Some(messaging) = skse::api::get_messaging_interface() {
            let result = messaging.register_listener(|message| {
                #[cfg(feature = "tracing")]
                tracing::info!("SKSE event: {message:?}");

                use commonlibsse_ng::skse::interfaces::messaging::MessageType;
                if message.msg_type == MessageType::DataLoaded {
                    tracing::info!("Data loaded");
                }
            });

            if result {
                #[cfg(feature = "tracing")]
                tracing::info!("Listener has been registered.");
            }
        }
    });

    if let Err(err) = result {
        #[cfg(feature = "tracing")]
        tracing::error!("{err:?}");
    }

    true
}

#[cfg(feature = "tracing")]
fn init_logger() {
    use tracing::level_filters::LevelFilter;

    const LOG_NAME: &str = "module_state.log";
    if let Err(err) = skse::logger::log_directory()
        .map(|log_dir| skse::logger::init(log_dir, LOG_NAME, LevelFilter::TRACE))
    {
        message_box("module_state Error", &err.to_string());
        std::process::exit(1);
    };
    tracing::info!("Logger has been initialized.");
}

fn message_box(title: &str, message: &str) {
    use windows::Win32::UI::WindowsAndMessaging::{MB_OK, MessageBoxW};
    use windows::core::HSTRING;

    let title = HSTRING::from(title.to_string());
    let message = HSTRING::from(message.to_string());
    let _result = unsafe { MessageBoxW(None, &message, &title, MB_OK) };
}
