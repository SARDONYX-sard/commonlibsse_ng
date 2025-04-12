/// Shows a modal message box in-game using Skyrim's `CreateMessage` API.
///
/// This macro works similarly to Rust's [`format!`] macro, allowing formatted strings.
/// Internally, it constructs a `CString` from the formatted string and passes it to [`DebugMessageBox`].
///
/// # Example
///
/// ```no_run
/// let x= 0;
/// let y= 1;
/// commonlibsse_ng::debug_message_box!("Player position: x = {}, y = {}", x, y);
/// ```
///
/// # Message Box Layout
/// ```txt
/// +--------------------------+
/// |    This is a message     |
/// |                          |
/// |          [OK]            |  <-- Primary Buttons
/// +--------------------------+
/// ```
///
/// # Notes
///
/// - The string is converted to a C-style string (`CString`), which **must not contain null bytes (`\0`)** in the middle.
///   If a null byte is present, the message will be replaced with `"Invalid CString"` as a fallback.
/// - This uses heap allocation under the hood via `CString::new`.
///
/// # See also
/// - [`DebugMessageBox`] for the underlying function.
/// - [`CString`] for how the formatted string is converted.
///
/// # Note
/// This macro does not panic, but it may silently fail and show a fallback message
/// if the formatted string contains interior null bytes.
///
/// [`DebugMessageBox`]: commonlibsse_ng::re::Misc::DebugMessageBox
#[macro_export]
macro_rules! debug_message_box {
    ($($arg:tt)*) => {{
        use std::ffi::CString;
        let formatted = format!($($arg)*);
        if let Ok(c_string) = CString::new(formatted) {
            $crate::re::Misc::DebugMessageBox(c_string.as_c_str());
        } else {
            $crate::re::Misc::DebugMessageBox(c"[DebugMessageBox Error] <Null byte found in string>");
        }
    }};
}
