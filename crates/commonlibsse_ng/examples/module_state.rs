use commonlibsse_ng::rel::version::Version;
use commonlibsse_ng::skse;
use commonlibsse_ng::skse::impls::stab::{PluginInfo, SKSEInterface};
use commonlibsse_ng::skse::interfaces::load::LoadInterface;
use commonlibsse_ng::skse::interfaces::query::QueryInterface;
use commonlibsse_ng::skse::interfaces::{PluginVersionData, to_fixed_str};
use commonlibsse_ng::skse::version::RUNTIME_SSE_LATEST;

const PKG_NAME: &str = "module_state";
const PKG_VERSION: Version = Version::from_str_const(env!("CARGO_PKG_VERSION"));

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn SKSEPlugin_Query(skse: &SKSEInterface, info: &mut PluginInfo) -> bool {
    *info = PluginInfo {
        infoVersion: PluginInfo::VERSION,
        name: c"module_state".as_ptr(),
        version: PKG_VERSION.pack(),
    };

    if QueryInterface::is_editor(skse) {
        #[cfg(feature = "tracing")]
        tracing::error!("The use of the SKSE Plugin within Editor is not supported.");
        return false;
    }

    true
}

#[unsafe(no_mangle)]
#[allow(non_upper_case_globals)]
pub static SKSEPlugin_Version: PluginVersionData = {
    let mut compatible_versions = [0; 16];
    compatible_versions[0] = RUNTIME_SSE_LATEST.pack();

    PluginVersionData {
        data_version: PKG_VERSION.major() as u32,
        plugin_version: PKG_VERSION.pack(),
        plugin_name: to_fixed_str(PKG_NAME),
        author: to_fixed_str(env!("CARGO_PKG_AUTHORS")),
        support_email: [0; 252],
        version_independence_ex: PluginVersionData::VERSION_INDEPENDENT_ADDRESS_LIBRARY_POST_AE,
        version_independence: PluginVersionData::VERSION_INDEPENDENT_EX_NO_STRUCT_USE,
        compatible_versions,
        xse_minimum: 0,
    }
};

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn SKSEPlugin_Load(skse: &LoadInterface) -> bool {
    #[cfg(feature = "tracing")]
    init_logger();

    if let Err(err) = std::panic::catch_unwind(|| {
        skse::init(skse);

        if let Some(messaging) = skse::api::get_messaging_interface() {
            messaging.register_listener(|message| {
                #[cfg(feature = "tracing")]
                tracing::info!("SKSE event: {message:?}");
            });
        }
    }) {
        #[cfg(feature = "tracing")]
        tracing::error!("{err:?}");
    }

    true
}

#[cfg(feature = "tracing")]
fn init_logger() {
    use skse::logger;
    use tracing::level_filters::LevelFilter;

    if let Err(err) = logger::init_with_log_dir("module_state.log", LevelFilter::TRACE) {
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
