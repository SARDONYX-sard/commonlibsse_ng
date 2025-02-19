// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/Interfaces.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{rel::version::Version, skse::impls::stab::SKSEInterface};

#[repr(C)]
pub struct QueryInterface {
    proxy: *const u8,
}

impl QueryInterface {
    pub fn editor_version(&self) -> u32 {
        unsafe { (*self.get_proxy()).editor_version }
    }

    pub fn is_editor(&self) -> bool {
        unsafe { (*self.get_proxy()).is_editor != 0 }
    }

    pub fn runtime_version(&self) -> Version {
        let packed = unsafe { (*self.get_proxy()).runtime_version };
        let major = ((packed & 0xFF000000) >> 24) as u16;
        let minor = ((packed & 0x00FF0000) >> 16) as u16;
        let revision = ((packed & 0x0000FFF0) >> 4) as u16;
        let build = (packed & 0x0000000F) as u16;
        Version::new(major, minor, revision, build)
    }

    pub fn skse_version(&self) -> u32 {
        unsafe { (*self.get_proxy()).skse_version }
    }

    pub(crate) fn get_proxy(&self) -> *const SKSEInterface {
        assert!(!self.proxy.is_null());
        self.proxy.cast()
    }
}
