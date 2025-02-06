// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/Version.h
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(feature = "win_api")]
mod win_api;

#[cfg(feature = "win_api")]
pub use win_api::{get_file_version, FileVersionError};

#[cfg(not(feature = "no_sys"))]
pub use crate::sys::REL::Version;

#[cfg(feature = "no_sys")]
/// Represents a 4-part version number.
///
/// # Example
/// ```
/// use commonlibsse_ng::rel::version::Version;
///
/// let ver = Version::new(1, 6, 1170, 0);
/// assert_eq!(ver.major(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    /// Internal representation of the version as a 4-element array.
    ///
    /// - Index 0: Major version
    /// - Index 1: Minor version
    /// - Index 2: Patch version
    /// - Index 3: Build number
    ///
    /// This field is private, and access should be done through provided methods.
    _impl: [u16; 4],
}

impl Version {
    /// Create a empty version.
    ///
    /// # Examples
    /// ```
    /// use commonlibsse_ng::rel::version::Version;
    ///
    /// assert_eq!(Version::const_default(), Version::new(0, 0, 0, 0));
    /// ```
    #[inline]
    pub const fn const_default() -> Self {
        Self::new(0, 0, 0, 0)
    }

    /// Creates a new `Version` from four components.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::rel::version::Version;
    ///
    /// let ver = Version::new(1, 2, 3, 4);
    /// assert_eq!(ver.major(), 1);
    /// ```
    #[inline]
    pub const fn new(major: u16, minor: u16, patch: u16, build: u16) -> Self {
        Self {
            _impl: [major, minor, patch, build],
        }
    }

    /// Parses a version string at compile time.
    ///
    /// # Errors
    /// Errors are made under the following conditions.
    ///
    /// - When there is no number after a point.
    /// - If there are more than 4 numbers.
    /// - When there is a non-numeric character (other than a dot).
    ///
    /// # Examples
    /// ```rust
    /// use commonlibsse_ng::rel::version::{Version, VersionParseError};
    ///
    /// assert_eq!(
    ///     Version::const_from_str("1.2.3.4"),
    ///     Ok(Version::new(1, 2, 3, 4))
    /// );
    /// assert_eq!(
    ///     Version::const_from_str("1.2.3"),
    ///     Ok(Version::new(1, 2, 3, 0))
    /// );
    ///
    /// assert_eq!(
    ///     Version::const_from_str("1.2.3.4.5"),
    ///     Err(VersionParseError::TooManyParts { parts: 4 })
    /// );
    /// assert_eq!(
    ///     Version::const_from_str("1.2.f.4.5"),
    ///     Err(VersionParseError::InvalidCharacter { character: 'f' })
    /// );
    /// assert_eq!(
    ///     Version::const_from_str("1.2."),
    ///     Err(VersionParseError::MissingNumber { part: 2 })
    /// );
    /// ```
    #[inline]
    pub const fn const_from_str(version: &str) -> Result<Self, VersionParseError> {
        let mut parts = [0u16; 4];
        let mut idx = 0;
        let mut num = 0;
        let mut has_digit = false;

        let bytes = version.as_bytes();
        let len = bytes.len();
        let mut i = 0;
        while i < len {
            let b = bytes[i];
            if b == b'.' {
                if idx >= 4 {
                    return Err(VersionParseError::TooManyParts { parts: idx });
                }
                parts[idx] = num;

                num = 0;
                idx += 1;
                has_digit = false;
            } else if b.is_ascii_digit() {
                num = num * 10 + (b - b'0') as u16;
                has_digit = true;
            } else {
                return Err(VersionParseError::InvalidCharacter {
                    character: b as char,
                });
            }
            i += 1;
        }

        if has_digit {
            if idx >= 4 {
                return Err(VersionParseError::TooManyParts { parts: idx });
            }
            parts[idx] = num;
        } else {
            return Err(VersionParseError::MissingNumber { part: idx });
        }

        Ok(Self { _impl: parts })
    }

    /// Returns the major version component.
    ///
    /// # Examples
    /// ```
    /// use commonlibsse_ng::rel::version::Version;
    ///
    /// let v = Version::new(1, 2, 3, 4);
    /// assert_eq!(v.major(), 1);
    /// ```
    #[inline]
    pub const fn major(&self) -> u16 {
        self._impl[0]
    }

    /// Returns the minor version component.
    ///
    /// # Examples
    /// ```
    /// use commonlibsse_ng::rel::version::Version;
    ///
    /// let v = Version::new(1, 2, 3, 4);
    /// assert_eq!(v.minor(), 2);
    /// ```
    #[inline]
    pub const fn minor(&self) -> u16 {
        self._impl[1]
    }

    /// Returns the patch version component.
    ///
    /// # Examples
    /// ```
    /// use commonlibsse_ng::rel::version::Version;
    ///
    /// let v = Version::new(1, 2, 3, 4);
    /// assert_eq!(v.patch(), 3);
    /// ```
    #[inline]
    pub const fn patch(&self) -> u16 {
        self._impl[2]
    }

    /// Returns the build version component.
    ///
    /// # Examples
    /// ```
    /// use commonlibsse_ng::rel::version::Version;
    ///
    /// let v = Version::new(1, 2, 3, 4);
    /// assert_eq!(v.build(), 4);
    /// ```
    #[inline]
    pub const fn build(&self) -> u16 {
        self._impl[3]
    }

    /// Packs the version into a 32-bit integer.
    /// # Examples
    /// ```
    /// use commonlibsse_ng::rel::version::Version;
    ///
    /// let v = Version::new(1, 2, 3, 4);
    /// assert_eq!(v.pack(), 16908340);
    /// ```
    #[inline]
    pub const fn pack(&self) -> u32 {
        ((self._impl[0] as u32 & 0xFF) << 24)
            | ((self._impl[1] as u32 & 0xFF) << 16)
            | ((self._impl[2] as u32 & 0xFFF) << 4)
            | (self._impl[3] as u32 & 0xF)
    }

    /// Unpacks a 32-bit integer into a `Version`.
    #[inline]
    pub const fn unpack(packed: u32) -> Self {
        Self {
            _impl: [
                ((packed >> 24) & 0xFF) as u16,
                ((packed >> 16) & 0xFF) as u16,
                ((packed >> 4) & 0xFFF) as u16,
                (packed & 0xF) as u16,
            ],
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::const_default()
    }
}

impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let major = self._impl[0];
        let minor = self._impl[1];
        let patch = self._impl[2];
        let build = self._impl[3];
        write!(f, "{major}.{minor}.{patch}.{build}",)
    }
}

impl core::ops::Index<usize> for Version {
    type Output = u16;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self._impl[index]
    }
}

impl core::ops::IndexMut<usize> for Version {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self._impl[index]
    }
}

impl core::str::FromStr for Version {
    type Err = VersionParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Version::const_from_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, snafu::Snafu)]
pub enum VersionParseError {
    /// Expected at most 4 parts, but got {parts} parts
    TooManyParts { parts: usize },

    /// Expected a number but got invalid character: `{character}`
    InvalidCharacter { character: char },

    /// Expected numbers after the dots, but got none in part {part}
    MissingNumber { part: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_ord() {
        let v1 = Version::new(1, 2, 3, 4);
        let v2 = Version::new(1, 2, 3, 5);
        let v3 = Version::new(2, 0, 0, 0);
        let v4 = Version::new(1, 2, 3, 4);

        assert!(v1 < v2);
        assert!(v2 > v1);
        assert!(v3 > v1);
        assert!(v1 == v4);
    }
}
