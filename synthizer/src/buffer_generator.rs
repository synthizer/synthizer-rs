use synthizer_sys::*;

use crate::context::*;
use crate::errors::*;
use crate::handle::*;

#[derive(Clone)]
pub struct BufferGenerator(Handle);

impl BufferGenerator {
    pub fn new(context: &Context) -> Result<BufferGenerator> {
        let mut h = 0;
        check_error(unsafe { syz_createBufferGenerator(&mut h as *mut u64, context.to_handle()) })?;
        Ok(BufferGenerator(Handle(h)))
    }
}

impl ToHandle for BufferGenerator {
    fn to_handle(&self) -> syz_Handle {
        self.0.0
    }
}
