// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/Interfaces.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{rel::version::Version, skse::impls::stab::SKSEInterface};

/// Provides an interface to query various version-related information.
///
/// This trait is designed to be implemented for structures that contain
/// versioning information, such as `SKSEInterface`. It allows querying the
/// editor version, runtime version, and SKSE version, as well as determining
/// whether the interface is running in an editor environment.
pub trait QueryInterface {
    /// Returns the editor version as a `u32`.
    fn editor_version(&self) -> u32;

    /// Returns `true` if the interface is running in the editor, otherwise `false`.
    fn is_editor(&self) -> bool;

    /// Returns the runtime version as a `Version` struct.
    fn runtime_version(&self) -> Version;

    /// Returns the SKSE (Skyrim Script Extender) version as a `u32`.
    fn skse_version(&self) -> u32;
}

impl QueryInterface for SKSEInterface {
    #[inline]
    fn editor_version(&self) -> u32 {
        self.editorVersion
    }

    #[inline]
    fn is_editor(&self) -> bool {
        self.isEditor != 0
    }

    #[inline]
    fn runtime_version(&self) -> Version {
        let packed = self.runtimeVersion;
        let major = ((packed & 0xFF000000) >> 24) as u16;
        let minor = ((packed & 0x00FF0000) >> 16) as u16;
        let revision = ((packed & 0x0000FFF0) >> 4) as u16;
        let build = (packed & 0x0000000F) as u16;
        Version::new(major, minor, revision, build)
    }

    #[inline]
    fn skse_version(&self) -> u32 {
        self.skseVersion
    }
}
