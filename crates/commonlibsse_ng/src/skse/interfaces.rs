// SPDX-FileCopyrightText: (C) 2024 metricexpansion
// SPDX-License-Identifier: MIT OR CC-BY-NC-SA-4.0
//
// See: https://gitlab.com/metricexpansion/SkyrimOutfitSystemSE/-/issues/2#note_2332635556

pub mod load;
pub mod messaging;
pub mod object;
pub mod papyrus;
pub mod query;
pub mod scaleform;
pub mod serialization;
pub mod task;
pub mod trampoline;

use crate::{rel::version::Version, rex::kernel32::get_current_module};

#[repr(C)]
#[derive(Debug)]
pub struct PluginVersionData {
    pub data_version: u32,
    pub plugin_version: u32,
    pub plugin_name: [u8; 256],
    pub author: [u8; 256],
    pub support_email: [u8; 252],
    pub version_independence_ex: u32,
    pub version_independence: u32,
    pub compatible_versions: [u32; 16],
    /// Insert the packed value of the minimum SKSE version required for operation.
    ///
    /// 0 if you are not sure.
    pub xse_minimum: u32,
}

const _: () = {
    use core::mem::offset_of;

    assert!(offset_of!(PluginVersionData, data_version) == 0x000);
    assert!(offset_of!(PluginVersionData, plugin_version) == 0x004);
    assert!(offset_of!(PluginVersionData, plugin_name) == 0x008);
    assert!(offset_of!(PluginVersionData, author) == 0x108);
    assert!(offset_of!(PluginVersionData, support_email) == 0x208);
    assert!(offset_of!(PluginVersionData, version_independence_ex) == 0x304);
    assert!(offset_of!(PluginVersionData, version_independence) == 0x308);
    assert!(offset_of!(PluginVersionData, compatible_versions) == 0x30C);
    assert!(offset_of!(PluginVersionData, xse_minimum) == 0x34C);
    assert!(size_of::<PluginVersionData>() == 0x350);
};

impl PluginVersionData {
    pub const VERSION: u32 = 1;

    pub const VERSION_INDEPENDENT_ADDRESS_LIBRARY_POST_AE: u32 = 1;
    pub const VERSION_INDEPENDENT_SIGNATURES: u32 = 1 << 1;
    pub const VERSION_INDEPENDENT_STRUCTS_POST_629: u32 = 1 << 2;

    pub const VERSION_INDEPENDENT_EX_NO_STRUCT_USE: u32 = 1;

    pub const fn set_plugin_version(&mut self, version: u32) {
        self.plugin_version = version;
    }

    pub const fn get_plugin_version(&self) -> u32 {
        self.plugin_version
    }

    pub fn set_plugin_name(&mut self, name: &str) {
        Self::set_char_buffer(name, &mut self.plugin_name);
    }

    pub fn get_plugin_name(&self) -> &str {
        Self::get_char_buffer(&self.plugin_name)
    }

    pub fn set_author_name(&mut self, name: &str) {
        Self::set_char_buffer(name, &mut self.author);
    }

    pub fn get_author_name(&self) -> &str {
        Self::get_char_buffer(&self.author)
    }

    pub fn set_author_email(&mut self, email: &str) {
        Self::set_char_buffer(email, &mut self.support_email);
    }

    pub fn get_author_email(&self) -> &str {
        Self::get_char_buffer(&self.support_email)
    }

    fn set_char_buffer(input: &str, buffer: &mut [u8]) {
        let bytes = input.as_bytes();
        let len = bytes.len().min(buffer.len() - 1);
        buffer[..len].copy_from_slice(&bytes[..len]);
        buffer[len] = 0;
    }

    fn get_char_buffer(buffer: &[u8]) -> &str {
        let end = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
        core::str::from_utf8(&buffer[..end]).unwrap_or("")
    }

    pub fn get_singleton() -> Option<&'static Self> {
        use windows::Win32::System::LibraryLoader::GetProcAddress;
        use windows::core::s;

        let f = unsafe { GetProcAddress(get_current_module(), s!("SKSEPlugin_Version")) };
        #[allow(clippy::fn_to_numeric_cast_any)]
        f.map(|f| unsafe { &*(f as *const Self) })
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructCompatibility {
    Dependent = 0,
    Independent = 1,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct VersionNumber {
    packed: u32,
}

impl VersionNumber {
    #[inline]
    pub const fn new(major: u16, minor: u16, patch: u16, build: u16) -> Self {
        Self {
            packed: ((major as u32) << 24)
                | ((minor as u32) << 16)
                | ((patch as u32) << 8)
                | (build as u32),
        }
    }

    #[inline]
    pub const fn default_const() -> Self {
        Self::new(0, 0, 0, 0)
    }

    #[inline]
    pub const fn from_version(version: Version) -> Self {
        Self { packed: version.pack() }
    }

    #[inline]
    pub const fn from_packed(packed: u32) -> Self {
        Self { packed }
    }

    #[inline]
    pub const fn to_packed(self) -> u32 {
        self.packed
    }
}

pub const fn to_fixed_str<const N: usize>(s: &str) -> [u8; N] {
    let bytes = s.as_bytes();
    let bytes_len = bytes.len();

    assert!(bytes_len < N, "The length of the input string is too large for the specified size.");

    let mut buf = [0_u8; N];

    let mut i = 0;
    while i < bytes_len {
        let b = bytes[i];
        assert!(b != 0, "The input string contains a null byte.");
        assert!(b.is_ascii(), "The input string contains non-ASCII characters.");
        buf[i] = b;
        i += 1;
    }

    buf
}

// SPDX-FileCopyrightText: (C) 2023 peelz
// SPDX-License-Identifier: MIT
// https://github.com/notpeelz/cstr-literal/blob/master/src/lib.rs#L11
//
/// Create str to Cstr at compile time.
///
/// # Example
///
/// ```rust
/// const HELLO: &std::ffi::CStr = commonlibsse_ng::skse::interfaces::new_cstr(concat!("hello", "\0"));
/// ```
///
/// # Panics
///
/// This function panics if:
/// - The input string contains null bytes before the null terminator.
/// - The input string is not null-terminated.
pub const fn new_cstr(s: &'static str) -> &'static std::ffi::CStr {
    let mut bytes = s.as_bytes();
    loop {
        match bytes {
            [0, _, ..] => panic!("C strings cannot contain null bytes"),
            [] => panic!("C strings must be null-terminated"),
            [0] => break,
            [_, remaining @ ..] => bytes = remaining,
        }
    }

    // SAFETY: The input string is validated to be null-terminated and without interior null bytes.
    unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(s.as_bytes()) }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct String256([u8; 256]);

impl String256 {
    /// Creates a new String256
    /// # Panics
    /// - Non ascii characters
    /// - Null byte
    pub const fn new(s: &str) -> Self {
        Self(to_fixed_str(s))
    }

    /// Creates a new String256 with a default value of 0
    pub const fn default_const() -> Self {
        Self([0; 256])
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct String252([u8; 252]);

impl String252 {
    /// Creates a new String252
    /// # Panics
    /// - Non ascii characters
    /// - Null byte
    pub const fn new(s: &str) -> Self {
        Self(to_fixed_str(s))
    }

    /// Creates a new String252 with a default value of 0
    pub const fn default_const() -> Self {
        Self([0; 252])
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RuntimeCompatibility {
    // The bool order of this bit flag is valid as long as ByteOrder is little-endian.
    pub address_library: bool,
    pub signature_scanning: bool,
    pub structs_post_629: bool,

    /// Initialization of pad is necessary because an error will occur if the memory is uninitialized.
    /// (Otherwise, UB will occur when [`core::mem::transmute`] is done on `PluginVersionData`.
    pub _pad0: u8,
    // _pad1: u8,
    // _pad2: u16,
    pub compatible_versions: [VersionNumber; 16],
}

const _: () = {
    assert!(core::mem::size_of::<RuntimeCompatibility>() == 0x44);
};

impl RuntimeCompatibility {
    pub const fn new() -> Self {
        Self {
            address_library: true,
            signature_scanning: false,
            structs_post_629: false,
            _pad0: 0,
            // _pad1: 0,
            // _pad2: 0,
            compatible_versions: [VersionNumber::new(0, 0, 0, 0); 16],
        }
    }
}

impl Default for RuntimeCompatibility {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PluginDeclarationInfo {
    /// The version number of the plugin.
    pub version: VersionNumber,
    /// The plugin's name (maximum of 256 characters).
    pub name: String256,
    /// The name of the plugin's author (maximum of 256 characters).
    pub author: String256,
    /// A support email address for the plugin (maximum of 256 characters).
    pub support_email: String252,
    /// Defines the compatibility with structure layout of the plugin.
    ///
    /// For most of modern CommonLibSSE-era plugin development structs in Skyrim have remained
    /// unchanged. In AE 1.6.629, however, the layout of some structs changed. If this is flagged
    /// as independent, then SKSE will let your plugin work with runtimes before and after this
    /// change. CommonLibSSE NG defaults to flagging a plugin independent because it supports
    /// both struct layouts in a single plugin. If your plugin has any RE'd structs that have
    /// changed you should override this.
    pub struct_compatibility: StructCompatibility,
    /// A definition of the runtime compatibility for the plugin.
    ///
    /// This can be either an indicator of how version-independence is achieved (either through using Address Library
    /// or signature scanning, indicated with a value from `skse::VersionIndependence`, or a list of up to
    /// 16 version numbers of Skyrim runtimes that are supported by this plugin.
    pub runtime_compatibility: RuntimeCompatibility,
    /// The minimum SKSE version required for the plugin; this should almost always be left 0.
    pub minimum_skse_version: VersionNumber,
}

const _: () = {
    use std::mem::offset_of;

    assert!(0x000 == offset_of!(PluginDeclarationInfo, version));
    assert!(0x004 == offset_of!(PluginDeclarationInfo, name));
    assert!(0x104 == offset_of!(PluginDeclarationInfo, author));
    assert!(0x204 == offset_of!(PluginDeclarationInfo, support_email));
    assert!(0x300 == offset_of!(PluginDeclarationInfo, struct_compatibility));
    assert!(0x304 == offset_of!(PluginDeclarationInfo, runtime_compatibility));
    assert!(0x348 == offset_of!(PluginDeclarationInfo, minimum_skse_version));
};

/// The same memory layout as `PluginVersionData`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PluginDeclaration {
    pub data_version: u32,
    pub data: PluginDeclarationInfo,
}
const _: () = assert!(0x350 == core::mem::size_of::<PluginDeclaration>());

impl PluginDeclaration {
    pub fn get_singleton() -> Option<&'static mut Self> {
        use windows::Win32::System::LibraryLoader::GetProcAddress;
        use windows::core::s;

        let f = unsafe { GetProcAddress(get_current_module(), s!("SKSEPlugin_Version")) };

        #[allow(clippy::fn_to_numeric_cast_any)]
        f.map(|f| unsafe { &mut *(f as *mut Self) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem;

    #[test]
    fn test_plugin_version_data_matches_plugin_declaration_info() {
        const PLUGIN_VERSION_DATA: PluginVersionData = PluginVersionData {
            data_version: 1,
            plugin_version: 2,
            plugin_name: [0; 256],
            author: [0; 256],
            support_email: [0; 252],
            version_independence_ex: 1,
            version_independence: 1,
            compatible_versions: [0; 16],
            xse_minimum: 0,
        };

        const PLUGIN_DECLARATION: PluginDeclaration = PluginDeclaration {
            data_version: 1,
            data: PluginDeclarationInfo {
                version: VersionNumber::new(0, 0, 0, 2),
                name: String256::default_const(),
                author: String256::default_const(),
                support_email: String252::default_const(),
                struct_compatibility: StructCompatibility::Independent,
                runtime_compatibility: RuntimeCompatibility {
                    address_library: true,
                    signature_scanning: false,
                    structs_post_629: false,
                    _pad0: 0,
                    compatible_versions: [VersionNumber::default_const(); 16],
                },
                minimum_skse_version: VersionNumber::default_const(),
            },
        };

        // NOTE: By doing this at compile time, it also serves as a UB check when type conversion is performed.
        const PLUGIN_VERSION_BYTES: [u8; 848] = unsafe { mem::transmute(PLUGIN_VERSION_DATA) };
        const PLUGIN_DECLARATION_BYTES: [u8; 848] = unsafe { mem::transmute(PLUGIN_DECLARATION) };

        pretty_assertions::assert_eq!(PLUGIN_VERSION_BYTES, PLUGIN_DECLARATION_BYTES); // Array can't evaluate at compile time
    }
}
