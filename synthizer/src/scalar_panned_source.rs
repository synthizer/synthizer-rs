use crate::internal_prelude::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct ScalarPannedSource(pub(crate) Handle);

impl ScalarPannedSource {
    pub fn new(
        context: &Context,
        panner_strategy: PannerStrategy,
        panning_scalar: f64,
    ) -> Result<ScalarPannedSource> {
        wrap_constructor(|ud, cb| {
            let mut h = Default::default();
            check_error(unsafe {
                syz_createScalarPannedSource(
                    &mut h as *mut syz_Handle,
                    context.to_syz_handle(),
                    panner_strategy as i32,
                    panning_scalar,
                    std::ptr::null_mut(),
                    ud,
                    Some(cb),
                )
            })?;
            Ok(ScalarPannedSource(Handle::new(h)))
        })
    }

    source_properties!();
    double_p!(SYZ_P_PANNING_SCALAR, get_panning_scalar, set_panning_scalar);

    object_common!();
    pausable_common!();
    source_common!();
}

handle_traits!(ScalarPannedSource);
