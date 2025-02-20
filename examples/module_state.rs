// Untested yet.
use commonlibsse_ng::rel::version::Version;
use commonlibsse_ng::skse;
use commonlibsse_ng::skse::impls::stab::PluginInfo;
use commonlibsse_ng::skse::interfaces::load::LoadInterface;
use commonlibsse_ng::skse::interfaces::query::QueryInterface;
use commonlibsse_ng::skse::interfaces::{
    PluginDeclaration, PluginDeclarationInfo, RuntimeCompatibility, String252, String256,
    StructCompatibility, VersionNumber,
};
use commonlibsse_ng::skse::version::{RUNTIME_SSE_1_5_97, RUNTIME_SSE_LATEST};

const fn to_cstr(s: &str) -> &core::ffi::CStr {
    unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(s.as_bytes()) }
}

const PKG_VERSION: Version = Version::from_str_const(env!("CARGO_PKG_VERSION"));

#[no_mangle]
#[allow(non_snake_case)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn SKSEPlugin_Query(skse: *const QueryInterface, info: *mut PluginInfo) -> bool {
    {
        let info = unsafe { &mut *info };
        *info = PluginInfo {
            version: PKG_VERSION.pack(),
            name: to_cstr(concat!(env!("CARGO_PKG_NAME"), "\0")).as_ptr(),
            info_version: PKG_VERSION.major() as u32,
        };
    };

    if unsafe { &*skse }.is_editor() {
        #[cfg(feature = "tracing")]
        tracing::error!("The use of the SKSE Plugin within Editor is not supported.");
        false
    } else {
        true
    }
}

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static SKSEPlugin_Version: PluginDeclaration = {
    const PKG_NAME: &str = env!("CARGO_PKG_NAME");
    const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

    let mut compatible_versions = [VersionNumber::default_const(); 16];
    compatible_versions[0] = VersionNumber::from_version(RUNTIME_SSE_LATEST);

    PluginDeclaration {
        data_version: PKG_VERSION.major() as u32,
        data: PluginDeclarationInfo {
            version: VersionNumber::from_version(PKG_VERSION),
            name: String256::new(PKG_NAME),
            author: String256::new(PKG_AUTHORS),
            support_email: String252::default_const(),
            struct_compatibility: StructCompatibility::Independent,
            runtime_compatibility: RuntimeCompatibility {
                address_library: true,
                signature_scanning: false,
                structs_post_629: false,
                compatible_versions,
            },
            minimum_skse_version: VersionNumber::from_version(RUNTIME_SSE_1_5_97),
        },
    }
};

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn SKSEPlugin_Load(skse: *const LoadInterface) {
    unsafe { skse::init(skse) };

    let _ = commonlibsse_ng::rel::module::ModuleState::map_or_init(|module| {
        #[cfg(feature = "tracing")]
        tracing::info!("{module:?}");
    });
}
