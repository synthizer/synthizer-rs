//! The `Context`.
use synthizer_sys::*;

use crate::errors::*;
use crate::handle::*;

/// The `Context` represents an audio device.
#[derive(Clone)]
pub struct Context(Handle);

impl Context {
    pub fn new() -> Result<Context> {
        let mut h=0;
        check_error(unsafe { syz_createContext(&mut h as *mut syz_Handle) })?;
        Ok(Context(Handle(h)))
    }

    pub fn enable_events(&self) -> Result<()> {
        check_error(unsafe { syz_contextEnableEvents(self.to_handle()) })
    }
}


impl ToHandle for Context {
    fn to_handle(&self) -> syz_Handle {
        self.0.0
    }
}
