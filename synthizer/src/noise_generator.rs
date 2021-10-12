use crate::internal_prelude::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct NoiseGenerator(pub(crate) Handle);

impl NoiseGenerator {
    pub fn new(context: &Context, channels: u32) -> Result<NoiseGenerator> {
        wrap_constructor(|ud, cb| {
            let mut h = Default::default();
            check_error(unsafe {
                syz_createNoiseGenerator(
                    &mut h as *mut syz_Handle,
                    context.to_syz_handle(),
                    channels,
                    null_mut(),
                    ud,
                    Some(cb),
                )
            })?;
            Ok(NoiseGenerator(Handle::new(h)))
        })
    }

    generator_properties!();
    enum_p!(NoiseType, SYZ_P_NOISE_TYPE, noise_type);

    object_common!();
    pausable_common!();
}

handle_traits!(NoiseGenerator);
