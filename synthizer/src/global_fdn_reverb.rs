use synthizer_sys::*;

use crate::context::*;
use crate::errors::*;
use crate::handle::*;

#[derive(Clone)]
pub struct GlobalFdnReverb(Handle);

impl GlobalFdnReverb {
    pub fn new(context: &Context) -> Result<GlobalFdnReverb> {
        let mut h = Default::default();
        check_error(unsafe {
            syz_createGlobalFdnReverb(&mut h as *mut syz_Handle, context.to_handle())
        })?;
        Ok(GlobalFdnReverb(Handle(h)))
    }
}

impl ToHandle for GlobalFdnReverb {
    fn to_handle(&self) -> syz_Handle {
        self.0 .0
    }
}
