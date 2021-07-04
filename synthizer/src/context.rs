//! The `Context`.
use synthizer_sys::*;

use crate::errors::*;
use crate::events;
use crate::handle::*;

/// The `Context` represents an audio device.
#[derive(Clone)]
pub struct Context(pub(crate) Handle);

impl Context {
    pub fn new() -> Result<Context> {
        let mut h = 0;
        check_error(unsafe { syz_createContext(&mut h as *mut syz_Handle) })?;
        Ok(Context(Handle(h)))
    }

    pub fn enable_events(&self) -> Result<()> {
        check_error(unsafe { syz_contextEnableEvents(self.to_handle()) })
    }

    /// Get any pending events.  The returned iterator will not block, and iterates over any pending events until the first error.  This is lazy: to limit the number of events received, use `.take`.
    pub fn get_events<'a>(&'a self) -> impl Iterator<Item = Result<events::Event>> + 'a {
        events::EventIterator {
            context: self,
            errored: false,
        }
    }
}

impl ToHandle for Context {
    fn to_handle(&self) -> syz_Handle {
        self.0 .0
    }
}
