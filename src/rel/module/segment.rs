// C++ Original code
// - https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/Module.h
// - load_segments, clear: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/Module.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Defines segment(e.g. `.text`) types.

/// Represents a memory segment in a module.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Segment {
    /// Base address of the proxy module.
    pub proxy_base: usize,
    /// Virtual address of the segment.
    pub address: u32,
    /// Size of the segment in bytes.
    pub size: u32,
}

impl Segment {
    /// Creates a new segment instance.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::rel::module::Segment;
    /// let segment = Segment::new(0x1000, 0x2000, 0x500);
    /// assert_eq!(segment.proxy_base, 0x1000);
    /// ```
    #[inline]
    pub const fn new(proxy_base: usize, address: u32, size: u32) -> Self {
        Self {
            proxy_base,
            address,
            size,
        }
    }

    /// Computes the offset of the segment from the proxy base.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::rel::module::Segment;
    /// let segment = Segment::new(0x1000, 0x2000, 0x500);
    /// assert_eq!(segment.offset(), 0x1000);
    /// ```
    #[inline]
    pub const fn offset(&self) -> usize {
        (self.address as usize).wrapping_sub(self.proxy_base)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
/// Represents different sections in a binary executable.
pub enum SegmentName {
    /// Executable code section (typically `.text`).
    #[default]
    Textx,

    /// Initialized data section (typically `.idata`).
    Idata,

    /// Read-only data section (typically `.rdata`).
    Rdata,

    /// Writable data section (typically `.data`).
    Data,

    /// Exception handling metadata section (typically `.pdata`).
    Pdata,

    /// Thread-local storage section (typically `.tls`).
    Tls,

    /// Writable text section (uncommon, but may exist for self-modifying code).
    Textw,

    /// Global function identifiers section (typically `.gfids`).
    Gfids,

    /// Total number of sections (used for iteration or bounds checking).
    Total,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_offset() {
        let segment = Segment::new(0x1000, 0x2000, 0x500);
        assert_eq!(segment.offset(), 0x1000);
    }
}
