use core::ffi::{CStr, c_char};
use core::ptr;

use crate::re::GameSettingCollection::GameSettingCollection;
use crate::re::IMessageBoxCallback::{IMessageBoxCallback, Message};
use crate::re::Setting::SettingValue;

#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 51420, ae_id = 52269)]
pub fn CreateMessage(
    message: *const c_char,
    callback: *mut IMessageBoxCallback,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    button_text: *const c_char,
    secondary_button_text: *const c_char,
) {
}

/// The configuration for displaying the message box.
pub struct MessageBoxConfig<'a> {
    /// The message to be displayed in the message box.
    pub message: &'a CStr,
    /// Text for the primary button (defaults to `"OK"`)
    ///
    /// If a task is registered, press this button in the game to execute it.
    pub button_text: &'a CStr,
    /// Text for the secondary button (no secondary button if None)
    pub secondary_button_text: Option<&'a CStr>,
    /// Optional task (closure) to execute when a button is pressed
    ///
    /// FIXME: Currently not running as it crashes mysteriously.
    pub task: Option<fn(Message)>,
}

impl Default for MessageBoxConfig<'_> {
    fn default() -> Self {
        Self {
            message: c"",
            button_text: c"OK",
            secondary_button_text: None,
            task: Default::default(),
        }
    }
}

/// Displays a message box with the given configuration.
///
/// This function creates a message box with the provided `message` and optional button texts.
/// If a `task` is provided, it will be executed when the button is pressed.
///
/// # Example
/// ```no_run
/// use std::ffi::CString;
/// use commonlibsse_ng::re::Misc::{DebugMessageBoxWithConfig, MessageBoxConfig};
///
/// let message = CString::new(format!("This is a {}", "message")).unwrap();
///
/// let config = MessageBoxConfig {
///     message: &message,
///     button_text: c"Yes",
///     secondary_button_text: Some(c"No"),
///     task: None,
/// };
///
/// DebugMessageBoxWithConfig(config);
/// ```
///
/// # Message Box Layout
/// ```txt
/// +--------------------------+
/// |    This is a message     |
/// |                          |
/// |     [Yes]     [No]       |  <-- Primary and Secondary Buttons
/// +--------------------------+
/// ```
#[inline]
pub fn DebugMessageBoxWithConfig(config: MessageBoxConfig) {
    // If secondary button text is not provided, use null pointer
    let secondary_button_text =
        config.secondary_button_text.map_or(ptr::null(), |cstr| cstr.as_ptr());

    // FIXME: Currently not running as it crashes mysteriously.
    // If a task is provided, create a callback for it
    // use crate::re::OldMessageBoxCallback::OldMessageBoxCallback;
    // let callback_ptr = config
    //     .task
    //     .map_or(ptr::null_mut(), |task| Box::into_raw(Box::new(OldMessageBoxCallback::new(task))));

    CreateMessage(
        config.message.as_ptr(),
        // callback_ptr.cast(),
        ptr::null_mut(),
        0,
        4,
        10,
        config.button_text.as_ptr(),
        secondary_button_text,
    );
    // drop(unsafe { Box::from_raw(callback_ptr) });
}

/// Display a simple dialog box with the given message.
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
/// # Example
/// ```no_run
/// commonlibsse_ng::re::Misc::DebugMessageBox(c"This is a message");
/// ```
#[inline]
pub fn DebugMessageBox(message: &CStr) {
    DebugMessageBoxWithConfig(MessageBoxConfig { message, ..Default::default() });
}

/// C++ `DebugMessageBox`
///
/// with i18n `OK` button
pub fn DebugMessageOkBox(message: &CStr) {
    unsafe {
        let ok_button = GameSettingCollection::get_singleton()
            .and_then(|gsc| gsc.__base.settings.__base.__base.__base.get(&c"sOk".as_ptr()))
            .and_then(|setting| {
                if let SettingValue::String(string) = setting.as_ref()?.get_value() {
                    return Some(string);
                }
                None
            })
            .unwrap_or(c"OK");
        DebugMessageBoxWithConfig(MessageBoxConfig {
            message,
            button_text: ok_button,
            ..Default::default()
        });
    }
}
