use synthizer_sys::*;

use crate::errors::*;

pub struct Handle(pub(crate) syz_Handle);

impl Drop for Handle {
    fn drop(&mut self) {
        check_error(unsafe { syz_handleDecRef(self.0) })
            .expect("Dropping handles should not error");
    }
}

impl Clone for Handle {
    fn clone(&self) -> Handle {
        check_error(unsafe { syz_handleIncRef(self.0) }).expect("Incrementing reference counts should never error");
        Handle(self.0)
    }
}

/// Internal helper trait to convert things to handles.  We don't use `From`
/// because that's public: people should be using the bindings, not converting
/// handles directly.
pub(crate) trait ToHandle {
    fn to_handle(&self) -> syz_Handle;
}
