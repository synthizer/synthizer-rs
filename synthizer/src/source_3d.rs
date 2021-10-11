use crate::internal_prelude::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Source3D(pub(crate) Handle);

impl Source3D {
    pub fn new(
        context: &Context,
        panner_strategy: PannerStrategy,
        initial_position: (f64, f64, f64),
    ) -> Result<Source3D> {
        wrap_constructor(|ud, cb| {
            let mut h = Default::default();
            check_error(unsafe {
                syz_createSource3D(
                    &mut h as *mut syz_Handle,
                    context.to_syz_handle(),
                    panner_strategy as i32,
                    initial_position.0,
                    initial_position.1,
                    initial_position.2,
                    null_mut(),
                    ud,
                    Some(cb),
                )
            })?;
            Ok(Source3D(Handle::new(h)))
        })
    }

    source_properties!();
    enum_p!(
        DistanceModel,
        SYZ_P_DISTANCE_MODEL,
        get_distance_model,
        set_distance_model
    );
    double_p!(SYZ_P_DISTANCE_REF, get_distance_ref, set_distance_ref);
    double_p!(SYZ_P_DISTANCE_MAX, get_distance_max, set_distance_max);
    double_p!(SYZ_P_ROLLOFF, get_rolloff, set_rolloff);
    double_p!(
        SYZ_P_CLOSENESS_BOOST,
        get_closeness_boost,
        set_closeness_boost
    );
    double_p!(
        SYZ_P_CLOSENESS_BOOST_DISTANCE,
        get_closeness_boost_distance,
        set_closeness_boost_distance
    );
    double3_p!(SYZ_P_POSITION, get_position, set_position);
    double6_p!(SYZ_P_ORIENTATION, get_orientation, set_orientation);

    object_common!();
    pausable_common!();
    source_common!();
}

handle_traits!(Source3D);
