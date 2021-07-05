use std::path::Path;

use synthizer_sys::*;

use crate::errors::*;

/// An `InitializationGuard` shuts Synthizer down when dropped, and must be kept
/// alive for the duration of your program.
pub struct InitializationGuard();

impl Drop for InitializationGuard {
    fn drop(&mut self) {
        unsafe { syz_shutdown() };
    }
}

/// Initialize Synthizer, returning a `InitializationGuard` which must be kept
/// alive for the duration of your program.
pub fn initialize() -> Result<InitializationGuard> {
    check_error(unsafe { syz_initialize() })?;
    Ok(InitializationGuard())
}

/// A builder to configure Synthizer initialization with non-default values.  To
/// initialize, call `.initialize()`.
pub struct LibraryConfig {
    config: syz_LibraryConfig,
    // Keep the Libsndfile string around until after initialization.
    libsndfile_path_string: Option<std::ffi::CString>,
}

#[repr(i32)]
pub enum LogLevel {
    Error = SYZ_LOG_LEVEL_ERROR as i32,
    Warn = SYZ_LOG_LEVEL_WARN as i32,
    Info = SYZ_LOG_LEVEL_INFO as i32,
    Debug = SYZ_LOG_LEVEL_DEBUG as i32,
}

impl LibraryConfig {
    pub fn new() -> LibraryConfig {
        let mut out = LibraryConfig {
            config: Default::default(),
            libsndfile_path_string: None,
        };
        unsafe { syz_libraryConfigSetDefaults(&mut out.config as *mut syz_LibraryConfig) };
        out
    }

    /// Configure the log level.
    pub fn log_level(&mut self, level: LogLevel) {
        self.config.log_level = unsafe { std::mem::transmute(level) };
    }

    /// Log to STDERR.
    pub fn log_to_stderr(&mut self) {
        self.config.logging_backend = SYZ_LOGGING_BACKEND_STDERR;
    }

    /// Load Libsndfile from the specified path. Errors if the path isn't valid UTF8.
    pub fn load_libsndfile<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let c_str = std::ffi::CString::new(
            path.as_ref()
                .to_str()
                .ok_or_else(|| Error::rust_error("Path isn't valid UTF-8"))?,
        )
        .map_err(|_| Error::rust_error("Unable to convert path to C string"))?;
        self.config.libsndfile_path = c_str.as_ptr();
        self.libsndfile_path_string = Some(c_str);
        Ok(())
    }

    /// Initialize Synthizer.
    pub fn initialize(self) -> Result<InitializationGuard> {
        check_error(unsafe { syz_initializeWithConfig(&self.config as *const syz_LibraryConfig) })?;
        Ok(InitializationGuard())
    }
}

impl Default for LibraryConfig {
    fn default() -> LibraryConfig {
        LibraryConfig::new()
    }
}
