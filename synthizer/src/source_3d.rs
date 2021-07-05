use synthizer_sys::*;

use crate::context::*;
use crate::errors::*;
use crate::handle::*;

#[derive(Clone)]
pub struct Source3D(Handle);

impl Source3D {
    pub fn new(context: &Context) -> Result<Source3D> {
        let mut h = Default::default();
        check_error(unsafe { syz_createSource3D(&mut h as *mut syz_Handle, context.to_handle()) })?;
        Ok(Source3D(Handle(h)))
    }
}

impl ToHandle for Source3D {
    fn to_handle(&self) -> syz_Handle {
        self.0 .0
    }
}
