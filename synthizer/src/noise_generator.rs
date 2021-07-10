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
            syz_createNoiseGenerator(&mut h as *mut syz_Handle, context.to_syz_handle(), channels)
        })?;
        Ok(NoiseGenerator(Handle(h)))
    }

    generator_properties!();
    enum_p!(NoiseType, SYZ_P_NOISE_TYPE, get_noise_type, set_noise_type);

    object_common!();
    pausable_common!();
}

impl ToSyzHandle for NoiseGenerator {
    fn to_syz_handle(&self) -> syz_Handle {
        self.0 .0
    }
}
