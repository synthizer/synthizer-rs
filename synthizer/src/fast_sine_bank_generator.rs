use crate::internal_prelude::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct FastSineBankGenerator(pub(crate) Handle);

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct SineBankWave(syz_SineBankWave);

impl SineBankWave {
    pub fn new(frequency_mul: f64, phase: f64, gain: f64) -> SineBankWave {
        SineBankWave(syz_SineBankWave {
            frequency_mul,
            phase,
            gain,
        })
    }
}

impl FastSineBankGenerator {
    pub fn new(
        context: &Context,
        initial_frequency: f64,
        waves: &[SineBankWave],
    ) -> Result<FastSineBankGenerator> {
        // No good way to get a pointer to the first element of an empty slice.
        if waves.is_empty() {
            return Err(Error::rust_error(
                "Cannot build FastSineBankGenerator with no waves",
            ));
        }

        wrap_constructor(|ud, cb| {
            let mut h = 0;
            check_error(unsafe {
                let cfg = syz_SineBankConfig {
                    initial_frequency,
                    wave_count: waves.len() as u64,
                    // The cast is okay because of repr(transparent) on
                    // SineBankWave.
                    waves: waves.get_unchecked(0) as *const SineBankWave as *const syz_SineBankWave,
                };

                syz_createFastSineBankGenerator(
                    &mut h as *mut u64,
                    context.to_syz_handle(),
                    &cfg as *const syz_SineBankConfig,
                    null_mut(),
                    ud,
                    Some(cb),
                )
            })?;
            Ok(FastSineBankGenerator(Handle::new(h)))
        })
    }

    pub fn new_sine(context: &Context, initial_frequency: f64) -> Result<FastSineBankGenerator> {
        wrap_constructor(|ud, cb| {
            let mut h = 0;
            check_error(unsafe {
                syz_createFastSineBankGeneratorSine(
                    &mut h as *mut u64,
                    context.to_syz_handle(),
                    initial_frequency,
                    null_mut(),
                    ud,
                    Some(cb),
                )
            })?;
            Ok(FastSineBankGenerator(Handle::new(h)))
        })
    }

    pub fn new_square(
        context: &Context,
        initial_frequency: f64,
        partials: u32,
    ) -> Result<FastSineBankGenerator> {
        wrap_constructor(|ud, cb| {
            let mut h = 0;
            check_error(unsafe {
                syz_createFastSineBankGeneratorSquare(
                    &mut h as *mut u64,
                    context.to_syz_handle(),
                    initial_frequency,
                    partials,
                    null_mut(),
                    ud,
                    Some(cb),
                )
            })?;
            Ok(FastSineBankGenerator(Handle::new(h)))
        })
    }

    pub fn new_triangle(
        context: &Context,
        initial_frequency: f64,
        partials: u32,
    ) -> Result<FastSineBankGenerator> {
        wrap_constructor(|ud, cb| {
            let mut h = 0;
            check_error(unsafe {
                syz_createFastSineBankGeneratorTriangle(
                    &mut h as *mut u64,
                    context.to_syz_handle(),
                    initial_frequency,
                    partials,
                    null_mut(),
                    ud,
                    Some(cb),
                )
            })?;
            Ok(FastSineBankGenerator(Handle::new(h)))
        })
    }

    pub fn new_saw(
        context: &Context,
        initial_frequency: f64,
        partials: u32,
    ) -> Result<FastSineBankGenerator> {
        wrap_constructor(|ud, cb| {
            let mut h = 0;
            check_error(unsafe {
                syz_createFastSineBankGeneratorSaw(
                    &mut h as *mut u64,
                    context.to_syz_handle(),
                    initial_frequency,
                    partials,
                    null_mut(),
                    ud,
                    Some(cb),
                )
            })?;
            Ok(FastSineBankGenerator(Handle::new(h)))
        })
    }

    generator_properties!();
    double_p!(SYZ_P_FREQUENCY, frequency);

    object_common!();
    pausable_common!();
}

handle_traits!(FastSineBankGenerator);
