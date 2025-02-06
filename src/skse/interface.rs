// SPDX-FileCopyrightText: (C) 2024 metricexpansion
// SPDX-License-Identifier: MIT OR CC-BY-NC-SA-4.0
//
// See: https://gitlab.com/metricexpansion/SkyrimOutfitSystemSE/-/issues/2#note_2332635556
use crate::sys::root::{__BindgenBitfieldUnit, SKSE};

impl SKSE::PluginVersionData {
    /// Create a new `SKSEPluginVersionData` at compile time.
    const fn const_default() -> Self {
        Self {
            dataVersion: SKSE::PluginVersionData_kVersion as u32,
            pluginVersion: 0,
            pluginName: [0; 256],
            author: [0; 256],
            supportEmail: [0; 252],
            _bitfield_align_1: [0; 0],
            _bitfield_1: __BindgenBitfieldUnit::<[u8; 1]>::new([0]),
            padding2: 0,
            padding3: 0,
            _bitfield_align_2: [0; 0],
            _bitfield_2: __BindgenBitfieldUnit::<[u8; 1]>::new([0]),
            padding5: 0,
            padding6: 0,
            compatibleVersions: [0; 16],
            xseMinimum: 0,
        }
    }
}

impl Default for SKSE::PluginVersionData {
    fn default() -> Self {
        Self::const_default()
    }
}
