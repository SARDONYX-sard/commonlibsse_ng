// C++ Original code
// - https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/ID.h
// - header_t::read: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/ID.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT
//! AddressLibrary header parser

use crate::rel::version::Version;

/// AddressLibrary header information
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Header {
    /// The version information of the address library.
    pub version: Version,

    /// The size of pointers in the address library, typically 8 bytes for 64-bit systems.
    pointer_size: u32,

    /// The number of addresses contained in the address library.
    address_count: u32,
}

impl Header {
    /// Creates a new `Header` instance with the given version, pointer size, and address count.
    ///
    /// # Example
    ///
    /// ```rust
    /// use commonlibsse_ng::rel::id::Header;
    /// use commonlibsse_ng::rel::version::Version;
    ///
    /// let header = Header::new(Version::new(1, 5, 97, 0), 8, 778674);
    /// assert_eq!(header.pointer_size(), 8);
    /// ```
    pub const fn new(version: Version, pointer_size: u32, address_count: u32) -> Self {
        Self {
            version,
            pointer_size,
            address_count,
        }
    }

    /// Parses a `Header` from a reader.
    ///
    /// Reads the format version, the address library version, name length, pointer size, and address count.
    ///
    /// # Errors
    ///
    /// Returns a `HeaderError` if any step in the reading process fails, such as:
    /// - Reading format version
    /// - Unsupported address format
    /// - Reading version, name length, pointer size, or address count
    ///
    /// # Example
    ///
    /// ```rust
    /// use commonlibsse_ng::rel::id::Header;
    /// use commonlibsse_ng::rel::version::Version;
    /// use std::io::Cursor;
    ///
    /// let binary_data: &[u8] = &[
    ///     0x01, 0x00, 0x00, 0x00, // 00000000:  u32_le: Format version => 0x00000001
    ///
    ///     // Skyrim version(1.5.97.0)
    ///     0x01, 0x00, 0x00, 0x00, // 00000004:  u32_le: Major -> 1
    ///     0x05, 0x00, 0x00, 0x00, // 00000008:  u32_le: Minor -> 5
    ///     0x61, 0x00, 0x00, 0x00, // 0000000C:  u32_le: Patch -> 97
    ///     0x00, 0x00, 0x00, 0x00, // 00000010:  u32_le: build -> 0
    ///
    ///     0x0C, 0x00, 0x00, 0x00, // 00000014:  u32_le: name length -> 0xc -> 12bytes
    ///
    ///     // The string "SkyrimSE.exe"(12bytes len) is being read here in ASCII
    ///     0x53, 0x6B, 0x79, 0x72, // 00000018:  The string (0x53 = 'S', 0x6B = 'k', 0x79 = 'y', 0x72 = 'r')
    ///     0x69, 0x6D, 0x53, 0x45, // 0000001C:  The string  (0x69 = 'i', 0x6D = 'm', 0x53 = 'S', 0x45 = 'E')
    ///     0x2E, 0x65, 0x78, 0x65, // 00000020:  The string ".exe" (0x2E = '.', 0x65 = 'e', etc.)
    ///
    ///     0x08, 0x00, 0x00, 0x00, // 00000024:  u32_le: The pointer size (this should be the pointer size, which is 8 bytes in this case)
    ///
    ///     0xB2, 0xE1, 0x0B, 0x00, // 00000028:  u32_le: Address count (0xbe1b2 -> 778_674)
    /// ];
    ///
    /// let mut cursor = Cursor::new(binary_data);
    /// let header = Header::from_reader(&mut cursor, 1).expect("Failed to read header");
    /// assert_eq!(header.version, Version::new(1, 5, 97, 0));
    /// assert_eq!(header.pointer_size(), 8);
    /// assert_eq!(header.address_count(), 778674);
    /// ```
    pub fn from_reader<R>(reader: &mut R, expected_fmt_ver: u8) -> Result<Self, HeaderError>
    where
        R: std::io::Read + std::io::Seek,
    {
        use snafu::ResultExt as _;

        // Read format version: 4bytes(1..=4 bytes)
        {
            let mut format = [0_u8; 4];
            reader
                .read_exact(&mut format)
                .context(ReadFormatVersionSnafu)?;
            let format = i32::from_le_bytes(format);

            if format != expected_fmt_ver as i32 {
                return Err(HeaderError::UnexpectedFormat {
                    expected: expected_fmt_ver,
                    actual_format: format,
                });
            }
        }

        // Read version: next 16bytes(5..=20 bytes nth)
        let version = {
            let mut version = [0_u8; 16];
            reader.read_exact(&mut version).context(ReadVersionSnafu)?;
            let version = u32_to_u16_array(u8_to_le_u32_array(version));
            Version::new(version[0], version[1], version[2], version[3])
        };

        // Read name length: next 4bytes(20..=23 bytes nth)
        // This value is usually `0x0c` -> 12bytes.
        {
            let mut name_len = [0_u8; 4];
            reader
                .read_exact(&mut name_len)
                .context(ReadNameLengthSnafu)?;
            let name_len = i32::from_le_bytes(name_len) as i64;
            reader
                .seek(std::io::SeekFrom::Current(name_len))
                .context(SeekAfterNameLengthSnafu)?;
        }

        // Read pointer size: next 4bytes(usually 0x24..=0x27 bytes nth)
        // This value is almost always 8(bytes) -> 64bit
        let pointer_size = {
            let mut pointer_size = [0_u8; 4];
            reader
                .read_exact(&mut pointer_size)
                .context(ReadPointerSizeSnafu)?;
            u32::from_le_bytes(pointer_size)
        };

        // Read address count: next 4bytes(usually 0x28..=0x2b bytes nth)
        let address_count = {
            let mut address_count = [0_u8; 4];
            reader
                .read_exact(&mut address_count)
                .context(ReadAddressCountSnafu)?;
            u32::from_le_bytes(address_count)
        };

        Ok(Self {
            version,
            address_count,
            pointer_size,
        })
    }

    /// Returns the number of addresses in the address library.
    ///
    /// # Example
    ///
    /// ```rust
    /// use commonlibsse_ng::rel::id::Header;
    /// use commonlibsse_ng::rel::version::Version;
    ///
    /// let header = Header::new(Version::new(1, 5, 97, 0), 8, 778674);
    /// assert_eq!(header.address_count(), 778674);
    /// ```
    pub const fn address_count(&self) -> usize {
        self.address_count as usize
    }

    /// Returns the pointer size in bytes, typically 8 bytes for 64-bit systems.
    ///
    /// # Example
    ///
    /// ```rust
    /// use commonlibsse_ng::rel::id::Header;
    /// use commonlibsse_ng::rel::version::Version;
    ///
    /// let header = Header::new(Version::new(1, 5, 97, 0), 8, 778674);
    /// assert_eq!(header.pointer_size(), 8);
    /// ```
    pub const fn pointer_size(&self) -> u64 {
        self.pointer_size as u64
    }
}

/// Type of error that occurs when reading the AddressLibrary header.
#[derive(Debug, snafu::Snafu)] // Derive Snafu error enum
pub enum HeaderError {
    /// Failed to read format version
    #[snafu(display("Failed to read format version: {}", source))]
    ReadFormatVersion { source: std::io::Error },

    /// Expected address library format {expected}, but got {actual_format}
    UnexpectedFormat { expected: u8, actual_format: i32 },

    #[snafu(display("Failed to read version: {}", source))]
    ReadVersion { source: std::io::Error },

    /// Failed to read name length
    #[snafu(display("Failed to read name length: {}", source))]
    ReadNameLength { source: std::io::Error },

    /// Failed to seek after name length
    #[snafu(display("Failed to seek after name length: {}", source))]
    SeekAfterNameLength { source: std::io::Error },

    /// Failed to read pointer size
    #[snafu(display("Failed to read pointer size: {}", source))]
    ReadPointerSize { source: std::io::Error },

    /// Failed to read address count
    #[snafu(display("Failed to read address count: {}", source))]
    ReadAddressCount { source: std::io::Error },
}

// Helper functions for version parsing
const fn u8_to_le_u32_array(input: [u8; 16]) -> [u32; 4] {
    [
        u32::from_le_bytes([input[0], input[1], input[2], input[3]]),
        u32::from_le_bytes([input[4], input[5], input[6], input[7]]),
        u32::from_le_bytes([input[8], input[9], input[10], input[11]]),
        u32::from_le_bytes([input[12], input[13], input[14], input[15]]),
    ]
}

const fn u32_to_u16_array(input: [u32; 4]) -> [u16; 4] {
    [
        input[0] as u16,
        input[1] as u16,
        input[2] as u16,
        input[3] as u16,
    ]
}
