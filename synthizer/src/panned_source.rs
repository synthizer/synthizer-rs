use synthizer_sys::*;

use crate::context::*;
use crate::errors::*;
use crate::handle::*;

#[derive(Clone)]
pub struct PannedSource(Handle);

impl PannedSource {
    pub fn new(context: &Context) -> Result<PannedSource> {
        let mut h = Default::default();
        check_error(unsafe { syz_createPannedSource(&mut h as *mut syz_Handle, context.to_handle()) })?;
        Ok(PannedSource(Handle(h)))
    }
}


impl ToHandle for PannedSource {
    fn to_handle(&self) -> syz_Handle {
        self.0.0
    }
}
