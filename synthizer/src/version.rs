use crate::internal_prelude::*;

/// Query the underlying Synthizer version, returning a `(major, minor, patch)`
/// tuple.  This isn't the version of the crate, but of the bound library.
pub fn get_version() -> (u32, u32, u32) {
    let mut major = 0;
    let mut minor = 0;
    let mut patch = 0;
    unsafe {
        syz_getVersion(
            &mut major as *mut u32,
            &mut minor as *mut u32,
            &mut patch as *mut u32,
        )
    };
    (major, minor, patch)
}
