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
pub mod types;

use crate::rel::version::Version;

pub struct TrampolineInterface;
impl TrampolineInterface {
    pub const VERSION: u32 = 1;

    pub fn version(&self) -> u32 {
        Self::VERSION
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PluginInfo {
    pub info_version: u32,
    pub name: *const i8,
    pub version: u32,
}

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
    pub xse_minimum: u32,
}

const _: () = {
    assert!(core::mem::offset_of!(PluginVersionData, data_version) == 0x000);
    assert!(core::mem::offset_of!(PluginVersionData, plugin_version) == 0x004);
    assert!(core::mem::offset_of!(PluginVersionData, plugin_name) == 0x008);
    assert!(core::mem::offset_of!(PluginVersionData, author) == 0x108);
    assert!(core::mem::offset_of!(PluginVersionData, support_email) == 0x208);
    assert!(core::mem::offset_of!(PluginVersionData, version_independence_ex) == 0x304);
    assert!(core::mem::offset_of!(PluginVersionData, version_independence) == 0x308);
    assert!(core::mem::offset_of!(PluginVersionData, compatible_versions) == 0x30C);
    assert!(core::mem::offset_of!(PluginVersionData, xse_minimum) == 0x34C);
    assert!(core::mem::size_of::<PluginVersionData>() == 0x350);
};

impl PluginVersionData {
    pub const VERSION: u32 = 1;
    pub const VERSION_INDEPENDENT_ADDRESS_LIBRARY_POST_AE: u32 = 1 << 0;
    pub const VERSION_INDEPENDENT_SIGNATURES: u32 = 1 << 1;
    pub const VERSION_INDEPENDENT_STRUCTS_POST_629: u32 = 1 << 2;
    pub const VERSION_INDEPENDENT_EX_NO_STRUCT_USE: u32 = 1 << 0;

    pub fn set_plugin_version(&mut self, version: u32) {
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

    pub fn uses_address_library(&mut self) {
        self.version_independence |= Self::VERSION_INDEPENDENT_ADDRESS_LIBRARY_POST_AE;
    }

    pub fn uses_signature_scanning(&mut self) {
        self.version_independence |= Self::VERSION_INDEPENDENT_SIGNATURES;
    }

    pub fn uses_updated_structs(&mut self) {
        self.version_independence |= Self::VERSION_INDEPENDENT_STRUCTS_POST_629;
    }

    pub fn uses_no_structs(&mut self) {
        self.version_independence_ex |= Self::VERSION_INDEPENDENT_EX_NO_STRUCT_USE;
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
    pub const fn new(major: u16, minor: u16, patch: u16, build: u16) -> Self {
        Self {
            packed: ((major as u32) << 24)
                | ((minor as u32) << 16)
                | ((patch as u32) << 8)
                | (build as u32),
        }
    }

    pub const fn from_version(version: Version) -> Self {
        Self {
            packed: version.pack(),
        }
    }

    pub const fn from_packed(packed: u32) -> Self {
        Self { packed }
    }

    pub const fn to_packed(self) -> u32 {
        self.packed
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct String256([u8; 256]);

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct String252([u8; 252]);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RuntimeCompatibility {
    address_library: bool,
    signature_scanning: bool,
    structs_post_629: bool,
    // _pad0: u8,
    // _pad1: u8,
    // _pad2: u16,
    compatible_versions: [VersionNumber; 16],
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
            // _pad0: 0,
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
#[derive(Debug, Clone, Copy)]
pub struct PluginDeclarationInfo {
    /// The version number of the plugin.
    version: VersionNumber,
    /// The plugin's name (maximum of 256 characters).
    name: String256,
    /// The name of the plugin's author (maximum of 256 characters).
    author: String256,
    /// A support email address for the plugin (maximum of 256 characters).
    support_email: String252,
    /// Defines the compatibility with structure layout of the plugin.
    ///
    /// For most of modern CommonLibSSE-era plugin development structs in Skyrim have remained
    /// unchanged. In AE 1.6.629, however, the layout of some structs changed. If this is flagged
    /// as independent, then SKSE will let your plugin work with runtimes before and after this
    /// change. CommonLibSSE NG defaults to flagging a plugin independent because it supports
    /// both struct layouts in a single plugin. If your plugin has any RE'd structs that have
    /// changed you should override this.
    struct_compatibility: StructCompatibility,
    /// A definition of the runtime compatibility for the plugin.
    ///
    /// This can be either an indicator of how version-independence is achieved (either through using Address Library
    /// or signature scanning, indicated with a value from `skse::VersionIndependence`, or a list of up to
    /// 16 version numbers of Skyrim runtimes that are supported by this plugin.
    runtime_compatibility: RuntimeCompatibility,
    /// The minimum SKSE version required for the plugin; this should almost always be left 0.
    minimum_skse_version: VersionNumber,
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

#[repr(C)]
pub struct PluginDeclaration {
    pub data_version: u32,
    pub data: PluginDeclarationInfo,
}

const _: () = assert!(0x350 == core::mem::size_of::<PluginDeclaration>());
