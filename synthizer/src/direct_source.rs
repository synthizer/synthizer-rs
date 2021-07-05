use synthizer_sys::*;

use crate::context::*;
use crate::errors::*;
use crate::handle::*;
#[derive(Clone)]
pub struct DirectSource(Handle);

impl DirectSource {
    pub fn new(context: &Context) -> Result<DirectSource> {
        let mut h = Default::default();
        check_error(unsafe {
            syz_createDirectSource(&mut h as *mut syz_Handle, context.to_handle())
        })?;
        Ok(DirectSource(Handle(h)))
    }
}

impl ToHandle for DirectSource {
    fn to_handle(&self) -> syz_Handle {
        self.0 .0
    }
}
