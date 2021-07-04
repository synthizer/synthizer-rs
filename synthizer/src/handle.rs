use synthizer_sys::*;

pub(crate) struct Handle(pub(crate) syz_Handle);

impl Clone for Handle {
    fn clone(&self) -> Handle {
        unsafe {
            syz_handleIncRef(self.0);
        }
        Handle(self.0)
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            syz_handleDecRef(self.0);
        }
    }
}

/// Internal helper trait to convert things to handles.  We don't use `From`
/// because that's public: people should be using the bindings, not converting
/// handles directly.
pub(crate) trait ToHandle {
    fn to_handle(&self) -> syz_Handle;
}
