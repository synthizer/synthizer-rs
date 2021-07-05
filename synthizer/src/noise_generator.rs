use synthizer_sys::*;

use crate::errors::*;
use crate::handle::*;
use crate::*;

#[derive(Clone)]
pub struct NoiseGenerator(Handle);

impl NoiseGenerator {
    pub fn new(context: &Context, channels: u32) -> Result<NoiseGenerator> {
        let mut h = Default::default();
        check_error(unsafe {
            syz_createNoiseGenerator(&mut h as *mut syz_Handle, context.to_handle(), channels)
        })?;
        Ok(NoiseGenerator(Handle(h)))
    }
}

impl ToHandle for NoiseGenerator {
    fn to_handle(&self) -> syz_Handle {
        self.0 .0
    }
}
