use crate::{rel::module::ModuleState, rex::win32::document_dir};
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

use super::version::RUNTIME_SSE_1_6_1170;

/// Get the log directory.(e.g. `$USER/documents/Skyrim Special Edition/SKSE`)
///
/// # Errors
/// Returns an error if the document dir could not be obtained
///
/// # Note
/// Searching in all diminutions where the current directory is `[..]\steamapps\common\Skyrim Special Edition`.
pub fn log_directory() -> Result<PathBuf, Error> {
    let mut path = document_dir().map_err(|_| Error::NotFoundLogDir)?;
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
pub fn init<P, D>(log_dir: P, file_name: D, level: LevelFilter) -> Result<(), Error>
where
    P: AsRef<std::path::Path>,
    D: AsRef<std::path::Path>,
{
    let log_path = log_dir.as_ref();
    // let log_dir = &resolver.app_log_dir().context(NotFoundLogDirSnafu)?;
    let file_appender = tracing_appender::rolling::never(log_path, file_name);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

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

    GUARD.set(guard).map_err(|_e| Error::FailedInitLog)?;
    RELOAD_HANDLE
        .set(reload_handle)
        .map_err(|_e| Error::FailedInitLog)
}

/// If unknown log level, fallback to `error`.
///
/// # Errors
/// If logger uninitialized.
pub fn change_level(log_level: &str) -> Result<(), Error> {
    let new_filter =
        <LevelFilter as core::str::FromStr>::from_str(log_level).unwrap_or_else(|_e| {
            tracing::warn!("Unknown log level: {log_level}. Fallback to `error`");
            LevelFilter::ERROR
        });
    match RELOAD_HANDLE.get() {
        Some(log) => Ok(log.modify(|filter| *filter = new_filter)?),
        None => Err(Error::UninitLog),
    }
}

/// Logger Error
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    NotFoundLogDir,

    FailedInitLog,
    UninitLog,

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
