use std::path::PathBuf;

/// # Errors
/// Returns an error if the known dir could not be obtained
pub fn known_folder(folder_id: windows::core::GUID) -> windows::core::Result<PathBuf> {
    use windows::Win32::UI::Shell::KNOWN_FOLDER_FLAG;
    unsafe {
        let path = windows::Win32::UI::Shell::SHGetKnownFolderPath(
            &folder_id,
            KNOWN_FOLDER_FLAG(0),
            None,
        )?;
        Ok(PathBuf::from(path.to_hstring().to_os_string()))
    }
}

/// Get the document dir.
/// # Errors
/// Returns an error if the document dir could not be obtained
#[inline]
pub fn document_dir() -> windows::core::Result<PathBuf> {
    known_folder(windows::Win32::UI::Shell::FOLDERID_Documents)
}

/// Show message(For critical error)
pub fn message_box(title: &str, message: &str) {
    use windows::Win32::UI::WindowsAndMessaging::{MB_OK, MessageBoxW};
    use windows::core::HSTRING;

    let title = HSTRING::from(title.to_string());
    let message = HSTRING::from(message.to_string());
    let _result = unsafe { MessageBoxW(None, &message, &title, MB_OK) };
}

/// Check memory page access permissions
pub fn is_valid_range(ptr: *const u8, len: usize) -> bool {
    use windows::Win32::System::Memory::{MEMORY_BASIC_INFORMATION, PAGE_NOACCESS, VirtualQuery};

    unsafe {
        let mut mbi = MEMORY_BASIC_INFORMATION::default();
        let mut check_ptr = ptr;

        while (check_ptr as usize) < (ptr as usize + len) {
            let is_invalid = VirtualQuery(
                Some(check_ptr.cast()),
                &mut mbi,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            ) == 0;

            if is_invalid {
                return false;
            }

            if mbi.Protect == PAGE_NOACCESS {
                return false;
            }

            check_ptr = check_ptr.add(mbi.RegionSize);
        }
        true
    }
}

/// Is this an accessible struct?
///
/// Returns `false` if
/// - The address pointed to by `T` is null
/// - When the address pointed to by `T` is not located at a memory address that is a multiple of `T` (unaligned)
/// - There is no permission to access an address of size `T`.
pub fn is_accessible_struct<T>(target: *const T) -> bool {
    if target.is_null() || !target.is_aligned() {
        return false;
    }

    is_valid_range(target.cast(), core::mem::size_of::<T>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messaging_interface() {
        if let Ok(dirs) = document_dir() {
            println!("{}", dirs.display());
        };
    }

    #[test]
    fn test_is_valid_range() {
        let valid_data = [42_u8; 16];
        let valid_ptr = valid_data.as_ptr();
        assert!(is_valid_range(valid_ptr, 16));

        let invalid_ptr: *const u8 = core::ptr::null();
        assert!(!is_valid_range(invalid_ptr, 16));
    }
}
