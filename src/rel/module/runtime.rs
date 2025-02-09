// C++ Original code
// - https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/Module.h
// - load_segments, clear: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/Module.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MI

//! Defines Runtime(e.g. `Runtime::Ae`) types.

use crate::rel::version::Version;

/// Defines Skyrim runtime versions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Runtime {
    /// Unknown runtime
    #[default]
    Unknown = 0,
    /// The Skyrim runtime is a post-Anniversary Edition Skyrim SE release (version 1.6.x and later).
    Ae = 1,
    /// The Skyrim runtime is a pre-Anniversary Edition Skyrim SE release (version 1.5.97 and prior).
    Se = 1 << 1,
    /// The Skyrim runtime is Skyrim VR.
    Vr = 1 << 2,
}

impl Runtime {
    /// Get the runtime from version.
    ///
    /// This function takes a `Version` object and returns the corresponding `Runtime` variant.
    ///
    /// The runtime is determined based on the version's `minor` numbers:
    /// - `minor` 4 -> `Runtime::Vr` (Skyrim VR)
    /// - `minor` 6 -> `Runtime::Ae` (Skyrim Anniversary Edition)
    /// - Any other version is considered `Runtime::Se` (Skyrim Special Edition).
    ///
    /// If you want strictness, use `Runtime::from_version_strict`.
    ///
    /// # Example
    ///
    /// ```
    /// use commonlibsse_ng::rel::module::Runtime;
    /// use commonlibsse_ng::rel::version::Version;
    ///
    /// let version = Version::new(1, 5, 50, 0); // SE version
    /// let runtime = Runtime::from_version(&version);
    /// assert_eq!(runtime, Runtime::Se);
    ///
    /// let version = Version::new(1, 6, 317, 0); // AE version
    /// let runtime = Runtime::from_version(&version);
    /// assert_eq!(runtime, Runtime::Ae);
    /// ```
    ///
    /// # Laxity of judgment.
    /// This judgment is incorrectly determined to be Vr if the SE is 1.4.2.
    ///
    /// This method is useful under the following assumptions
    /// - SE users are using the latest (1.5.97).
    /// - The version update of this library has not caught up with the version of Skyrim, even though the version of Skyrim has been upgraded.
    #[inline]
    pub const fn from_version(version: &Version) -> Self {
        match version.minor() {
            4 => Self::Vr,
            6 => Self::Ae,
            _ => Self::Se,
        }
    }

    /// Get the runtime from version, strictly matching predefined database versions.
    ///
    /// This function will only return a runtime if the version matches exactly one of the
    /// predefined versions in the database.
    ///
    /// # Example
    ///
    /// ```
    /// use commonlibsse_ng::rel::module::Runtime;
    /// use commonlibsse_ng::rel::version::Version;
    /// use commonlibsse_ng::skse::version::{RUNTIME_SSE_1_5_50, RUNTIME_SSE_1_6_317, RUNTIME_VR_1_4_15};
    ///
    /// // SE version within range
    /// let runtime = Runtime::from_version_strict(&RUNTIME_SSE_1_5_50);
    /// assert_eq!(runtime, Some(Runtime::Se));
    ///
    /// // AE version within range
    /// let runtime = Runtime::from_version_strict(&RUNTIME_SSE_1_6_317);
    /// assert_eq!(runtime, Some(Runtime::Ae));
    ///
    /// // VR version
    /// let runtime = Runtime::from_version_strict(&RUNTIME_VR_1_4_15);
    /// assert_eq!(runtime, Some(Runtime::Vr));
    /// ```
    pub const fn from_version_strict(version: &Version) -> Option<Self> {
        use crate::skse::version::*;

        // Match specific predefined version constants for SE, AE, and VR

        Some(match *version {
            // SE versions (1.1.47 to 1.5.97)
            RUNTIME_SSE_1_1_47 | RUNTIME_SSE_1_1_51 | RUNTIME_SSE_1_2_36 | RUNTIME_SSE_1_2_39
            | RUNTIME_SSE_1_3_5 | RUNTIME_SSE_1_3_9 | RUNTIME_SSE_1_4_2 | RUNTIME_SSE_1_5_3
            | RUNTIME_SSE_1_5_16 | RUNTIME_SSE_1_5_23 | RUNTIME_SSE_1_5_39 | RUNTIME_SSE_1_5_50
            | RUNTIME_SSE_1_5_53 | RUNTIME_SSE_1_5_62 | RUNTIME_SSE_1_5_73 | RUNTIME_SSE_1_5_80
            | RUNTIME_SSE_1_5_97 => Self::Se,

            // AE versions (1.6.0 to 1.6.1170)
            RUNTIME_SSE_1_6_317 | RUNTIME_SSE_1_6_318 | RUNTIME_SSE_1_6_323
            | RUNTIME_SSE_1_6_342 | RUNTIME_SSE_1_6_353 | RUNTIME_SSE_1_6_629
            | RUNTIME_SSE_1_6_640 | RUNTIME_SSE_1_6_659 | RUNTIME_SSE_1_6_678
            | RUNTIME_SSE_1_6_1130 | RUNTIME_SSE_1_6_1170 => Self::Ae,

            // VR version (1.4.15)
            RUNTIME_VR_1_4_15 => Self::Vr,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skse::version::{RUNTIME_SSE_1_5_50, RUNTIME_SSE_1_6_317, RUNTIME_VR_1_4_15};

    /// Tests the correct conversion from `Version` to `Runtime`.
    /// This simulates real-world scenarios based on Skyrim's version numbers.
    #[test]
    fn test_runtime_enum() {
        // Checking the enum values by their numeric representation.
        assert_eq!(Runtime::Unknown as u8, 0);
        assert_eq!(Runtime::Ae as u8, 1);
        assert_eq!(Runtime::Se as u8, 2);
        assert_eq!(Runtime::Vr as u8, 4);

        assert_eq!(Runtime::from_version(&RUNTIME_SSE_1_5_50), Runtime::Se);
        assert_eq!(Runtime::from_version(&RUNTIME_SSE_1_6_317), Runtime::Ae);
        assert_eq!(Runtime::from_version(&RUNTIME_VR_1_4_15), Runtime::Vr);

        let version_1_4_5 = Version::new(1, 4, 5, 0); // Unknown version (not recognized by rules)
        assert_eq!(Runtime::from_version(&version_1_4_5), Runtime::Vr);
        assert_eq!(Runtime::from_version_strict(&version_1_4_5), None);
    }
}
