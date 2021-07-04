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
