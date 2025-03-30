use crate::rel::module::ModuleState;
use crate::rex::win32::document_dir;
use crate::skse::version::RUNTIME_SSE_1_6_1170;
use snafu::ResultExt as _;
use std::path::{Path, PathBuf};
#[cfg(feature = "tracing")]
use std::sync::OnceLock;
#[cfg(feature = "tracing")]
use tracing_subscriber::{
    Registry,
    filter::LevelFilter,
    fmt,
    prelude::*,
    reload::{self, Handle},
};

/// Get the log directory.(e.g. `$USER/documents/Skyrim Special Edition/SKSE`)
///
/// # Errors
/// Returns an error if the document dir could not be obtained
///
/// # Note
/// Searching in all diminutions where the current directory is `[..]/steamapps/common/Skyrim Special Edition`.
pub fn log_directory() -> Result<PathBuf, LogInitError> {
    let mut path = document_dir().map_err(|_| LogInitError::NotFoundDocumentDir)?;
    path.push("My Games");

    let (runtime, version) =
        ModuleState::map_or_init(|module| (module.runtime, module.version.clone()))
            .context(UnexpectedModuleStateSnafu)?;

    if runtime.is_vr() {
        path.push("Skyrim VR");
    } else if Path::new("steam_api64.dll").exists() {
        if Path::new("openvr_api.dll").exists() {
            path.push("Skyrim VR");
        } else if version >= RUNTIME_SSE_1_6_1170 {
            path.push("Skyrim.INI");
        } else {
            path.push("Skyrim Special Edition");
        }
    } else {
        path.push("Skyrim Special Edition GOG");
    }
    path.push("SKSE");
    Ok(path)
}

/// Global variable to allow dynamic level changes in logger.
#[cfg(feature = "tracing")]
static RELOAD_HANDLE: OnceLock<Handle<LevelFilter, Registry>> = OnceLock::new();
#[cfg(feature = "tracing")]
static GUARD: OnceLock<tracing_appender::non_blocking::WorkerGuard> = OnceLock::new();

/// Initializes logger.
///
/// # Errors
/// - If the directory for logs is not found
/// - Logger did not have permission to create files
#[cfg(feature = "tracing")]
pub fn init<P, D>(log_dir: P, file_name: D, level: LevelFilter) -> Result<(), LogInitError>
where
    P: AsRef<Path>,
    D: AsRef<Path>,
{
    _init(log_dir.as_ref(), file_name.as_ref(), level)
}

/// Initializes logger with skse log directory.
///
/// # Errors
/// - If the directory for logs is not found
/// - Logger did not have permission to create files
#[cfg(feature = "tracing")]
pub fn init_with_log_dir<P>(file_name: P, level: LevelFilter) -> Result<(), LogInitError>
where
    P: AsRef<Path>,
{
    _init(&log_directory()?, file_name.as_ref(), level)
}

#[cfg(feature = "tracing")]
fn _init(log_dir: &Path, file_name: &Path, level: LevelFilter) -> Result<(), LogInitError> {
    use tracing_appender::non_blocking::NonBlockingBuilder;

    let _ = std::fs::create_dir_all(log_dir);
    let file = std::fs::File::create(log_dir.join(file_name))
        .map_err(|_e| LogInitError::FailedCreateLogFile)?;
    let (non_blocking, guard) = NonBlockingBuilder::default().finish(file);

    // Unable `pretty()` & `with_ansi(false)` combination in `#[tracing::instrument]`
    // ref: https://github.com/tokio-rs/tracing/issues/1310
    let fmt_layer = fmt::layer()
        .compact()
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_writer(non_blocking);

    let (filter, reload_handle) = reload::Layer::new(level);
    tracing_subscriber::registry().with(filter).with(fmt_layer).init();

    GUARD.set(guard).map_err(|_e| LogInitError::FailedInitLog)?;
    RELOAD_HANDLE.set(reload_handle).map_err(|_e| LogInitError::FailedInitLog)
}

/// If unknown log level, fallback to `error`.
///
/// # Errors
/// If logger uninitialized.
#[cfg(feature = "tracing")]
pub fn change_level(log_level: &str) -> Result<(), LogReloadError> {
    use snafu::ResultExt as _;

    let new_filter =
        <LevelFilter as core::str::FromStr>::from_str(log_level).unwrap_or_else(|_e| {
            tracing::warn!("Unknown log level: {log_level}. Fallback to `error`");
            LevelFilter::ERROR
        });

    RELOAD_HANDLE.get().map_or(Err(LogReloadError::UninitLog), |log| {
        log.modify(|filter| *filter = new_filter).context(ReloadSnafu)
    })
}

/// Error that may occur when changing logger settings
#[cfg(feature = "tracing")]
#[derive(Debug, snafu::Snafu)]
pub enum LogReloadError {
    /// Logger uninitialized.
    UninitLog,

    /// Failed to change the log level: {source}
    Reload { source: tracing_subscriber::reload::Error },
}

/// Possible errors during logger initialization
#[derive(Debug, snafu::Snafu)]
pub enum LogInitError {
    /// The logger could not be initialized because the document directory was not found.
    NotFoundDocumentDir,

    /// Logger could not be initialized because runtime information could not be obtained.: {source}
    UnexpectedModuleState { source: crate::rel::module::ModuleStateError },

    /// Failed to create a log file.
    FailedCreateLogFile,

    /// Failed to Initialize a log.
    FailedInitLog,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_directory() {
        let log_dir = log_directory().unwrap();
        println!("{}", log_dir.display());
    }
}
