// Untested yet.
use commonlibsse_ng::rel::module::ModuleState;
use commonlibsse_ng::rel::version::Version;
use commonlibsse_ng::skse::impls::stab::PluginInfo;
use commonlibsse_ng::skse::interfaces::load::LoadInterface;
use commonlibsse_ng::skse::interfaces::query::QueryInterface;
use commonlibsse_ng::skse::interfaces::PluginVersionData;
use commonlibsse_ng::skse::version::{RUNTIME_SSE_1_5_97, RUNTIME_SSE_LATEST};

const fn to_fixed_str<const N: usize>(s: &str) -> [u8; N] {
    let bytes = s.as_bytes();
    let bytes_len = bytes.len();

    assert!(
        bytes_len <= N,
        "The bytes_len in &str is too larger than the specified bytes."
    );

    let mut buf = [0_u8; N];
    let mut i = 0;
    while i < bytes_len && i < N - 1 {
        buf[i] = bytes[i];
        i += 1;
    }

    buf
}

const fn to_cstr(s: &str) -> &core::ffi::CStr {
    unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(s.as_bytes()) }
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn SKSEPlugin_Query(skse: *const QueryInterface, info: *mut PluginInfo) -> bool {
    {
        let info = unsafe { &mut *info };
        let ver = Version::from_str_const(env!("CARGO_PKG_VERSION"));
        *info = PluginInfo {
            version: ver.pack(),
            name: to_cstr(env!("CARGO_PKG_NAME")).as_ptr(),
            info_version: ver.major() as u32,
        };
    };

    if unsafe { &*skse }.is_editor() {
        tracing::error!("The use of the SKSE Plugin within Editor is not supported.");
        false
    } else {
        true
    }
}

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static SKSEPlugin_Version: PluginVersionData = {
    let mut compatible_versions = [0; 16];
    compatible_versions[0] = RUNTIME_SSE_LATEST.pack();

    PluginVersionData {
        data_version: PluginVersionData::VERSION,
        plugin_version: Version::from_str_const(env!("CARGO_PKG_VERSION")).pack(),
        plugin_name: to_fixed_str(env!("CARGO_PKG_NAME")),
        author: to_fixed_str(env!("CARGO_PKG_AUTHORS")),
        support_email: to_fixed_str(env!("CARGO_PKG_VERSION")),
        version_independence_ex: 0,
        version_independence: 0,
        compatible_versions,
        xse_minimum: RUNTIME_SSE_1_5_97.pack(),
    }
};

#[no_mangle]
pub extern "C" fn SKSEPlugin_Load(_skse: *const LoadInterface) {
    let _ = ModuleState::map_or_init(|module| tracing::info!("{module:?}"));
}
