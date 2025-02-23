use crate::rel::module::ModuleState;
use crate::rex::win32::document_dir;
use crate::skse::version::RUNTIME_SSE_1_6_1170;
use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};
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
/// Searching in all diminutions where the current directory is `[..]\steamapps\common\Skyrim Special Edition`.
pub fn log_directory() -> Result<PathBuf, LogError> {
    let mut path = document_dir().map_err(|_| LogError::NotFoundLogDir)?;
    path.push("My Games");

    let (runtime, version) =
        ModuleState::map_or_init(|module| (module.runtime, module.version.clone()))?;

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
static RELOAD_HANDLE: OnceLock<Handle<LevelFilter, Registry>> = OnceLock::new();
static GUARD: OnceLock<tracing_appender::non_blocking::WorkerGuard> = OnceLock::new();

/// Initializes logger.
///
/// # Errors
/// Double init
pub fn init<P, D>(log_dir: P, file_name: D, level: LevelFilter) -> Result<(), LogError>
where
    P: AsRef<std::path::Path>,
    D: AsRef<std::path::Path>,
{
    let log_path = log_dir.as_ref();
    let _ = std::fs::create_dir_all(log_path);
    let file = std::fs::File::create(log_path.join(file_name))
        .map_err(|_e| LogError::FailedCreateLogFile)?;
    let (non_blocking, guard) =
        tracing_appender::non_blocking::NonBlockingBuilder::default().finish(file);

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
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();

    GUARD.set(guard).map_err(|_e| LogError::FailedInitLog)?;
    RELOAD_HANDLE
        .set(reload_handle)
        .map_err(|_e| LogError::FailedInitLog)
}

/// If unknown log level, fallback to `error`.
///
/// # Errors
/// If logger uninitialized.
pub fn change_level(log_level: &str) -> Result<(), LogError> {
    let new_filter =
        <LevelFilter as core::str::FromStr>::from_str(log_level).unwrap_or_else(|_e| {
            tracing::warn!("Unknown log level: {log_level}. Fallback to `error`");
            LevelFilter::ERROR
        });
    match RELOAD_HANDLE.get() {
        Some(log) => Ok(log.modify(|filter| *filter = new_filter)?),
        None => Err(LogError::UninitLog),
    }
}

/// Logger Error
#[derive(Debug, snafu::Snafu)]
pub enum LogError {
    /// Not found log dir.
    NotFoundLogDir,

    /// Failed to create a log file.
    FailedCreateLogFile,
    /// Failed to Initialize a log.
    FailedInitLog,

    /// Logger uninitialized.
    UninitLog,

    /// Failed to change the log level.
    #[snafu(transparent)]
    Reload {
        source: tracing_subscriber::reload::Error,
    },

    #[snafu(transparent)]
    Module {
        source: crate::rel::module::ModuleStateError,
    },
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
