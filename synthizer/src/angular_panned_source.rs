use crate::internal_prelude::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct AngularPannedSource(pub(crate) Handle);

impl AngularPannedSource {
    pub fn new(
        context: &Context,
        panner_strategy: PannerStrategy,
        azimuth: f64,
        elevation: f64,
    ) -> Result<AngularPannedSource> {
        wrap_constructor(|ud, cb| {
            let mut h = Default::default();
            check_error(unsafe {
                syz_createAngularPannedSource(
                    &mut h as *mut syz_Handle,
                    context.to_syz_handle(),
                    panner_strategy as i32,
                    azimuth,
                    elevation,
                    std::ptr::null_mut(),
                    ud,
                    Some(cb),
                )
            })?;
            Ok(AngularPannedSource(Handle::new(h)))
        })
    }

    source_properties!();
    double_p!(SYZ_P_ELEVATION, get_elevation, set_elevation);
    double_p!(SYZ_P_AZIMUTH, get_azimuth, set_azimuth);

    object_common!();
    pausable_common!();
    source_common!();
}

handle_traits!(AngularPannedSource);
