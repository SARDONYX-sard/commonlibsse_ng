// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/Version.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(feature = "no_sys")]
use crate::rel::version::Version;
#[cfg(not(feature = "no_sys"))]
use crate::sys::REL::Version;

/// Retrieves the file version of the specified executable or DLL.
///
/// # Errors
/// - The [`lang-codepage`] part is **assumed to be an US English exe**, so if it is not an US English exe, the acquisition will fail.
/// - It also fails if the version is not mixed in the exe.
///
/// # Example
/// ```no_run
/// use commonlibsse_ng::rel::version::{get_file_version, Version};
///
/// let target = r"D:\STEAM\steamapps\common\Skyrim Special Edition\SkyrimSE.exe";
/// let result = get_file_version(target);
/// assert_eq!(result, Ok(Version::new(1, 6, 1170, 0)));
/// ```
///
/// [`lang-codepage`]: https://learn.microsoft.com/windows/win32/api/winver/nf-winver-verqueryvaluew#stringfileinfolang-codepagestring-name
pub fn get_file_version(filename: &str) -> Result<Version, FileVersionError> {
    // https://microsoft.github.io/windows-docs-rs/doc/windows/?search=GetFileVersionInfoSizeW
    use core::ptr;
    use windows::Win32::Storage::FileSystem::{
        GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW,
    };

    let filename_w = windows::core::HSTRING::from(filename);

    let mut dummy = 0;
    let size = unsafe { GetFileVersionInfoSizeW(&filename_w, Some(&mut dummy)) };
    if size == 0 {
        return Err(FileVersionError::VersionInfoSize {
            filename: filename.to_string(),
        });
    }

    let mut buf = vec![0u8; size as usize];

    if let Err(err) =
        unsafe { GetFileVersionInfoW(&filename_w, None, size, buf.as_mut_ptr().cast()) }
    {
        return Err(FileVersionError::VersionInfoRetrieval {
            filename: filename.to_string(),
            err,
        });
    }

    let ver_str = {
        let buf_void_ptr = buf.as_mut_ptr().cast();
        let query_path = windows::core::h!("\\StringFileInfo\\040904B0\\ProductVersion"); // NOTE: assumed 040904B0(US English, Unicode
        let mut ver_buf = ptr::null_mut();
        let mut ver_len: u32 = 0;
        if unsafe { VerQueryValueW(buf_void_ptr, query_path, &mut ver_buf, &mut ver_len) }.as_bool()
            == false
        {
            return Err(FileVersionError::VersionQuery {
                filename: filename.to_string(),
            });
        }

        let slice = unsafe { core::slice::from_raw_parts(ver_buf as *const u16, ver_len as usize) };
        String::from_utf16_lossy(slice)
    };

    let mut version = Version::const_default();
    for (i, token) in ver_str.split('.').take(4).enumerate() {
        if let Ok(num) = token.parse::<u16>() {
            version[i] = num;
        }
    }

    Ok(version)
}

/// Error types for file version retrieval.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, snafu::Snafu)]
pub enum FileVersionError {
    /// Failed to get file version info size for '{filename}'
    VersionInfoSize { filename: String },

    /// Failed to retrieve file version info for '{filename}', err: {err}
    VersionInfoRetrieval {
        filename: String,
        err: windows::core::Error,
    },

    /// Failed to query product version for '{filename}'
    VersionQuery { filename: String },

    /// Invalid version format in '{filename}'
    VersionFormat { filename: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_file_version() {
        // Use a system file that is guaranteed to exist on Windows.
        let target = "C:\\Windows\\splwow64.exe";
        let version = get_file_version(target).unwrap_or_else(|err| panic!("{err}"));
        dbg!(version);
    }

    #[test]
    fn test_invalid_file_version() {
        let target = "C:\\nonexistent_file.exe";
        let result = get_file_version(target);

        let expected_err = Err(FileVersionError::VersionInfoSize {
            filename: target.to_string(),
        });

        assert_eq!(result, expected_err);
    }
}
