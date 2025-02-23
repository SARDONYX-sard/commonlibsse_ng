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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messaging_interface() {
        if let Ok(dirs) = document_dir() {
            println!("{}", dirs.display());
        };
    }
}
