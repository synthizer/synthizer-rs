use synthizer_sys::*;

use crate::context::*;
use crate::errors::*;
use crate::generator::*;
use crate::handle::*;
use crate::*;

#[derive(Clone)]
pub struct Source3D(pub(crate) Handle);

impl Source3D {
    pub fn new(context: &Context) -> Result<Source3D> {
        let mut h = Default::default();
        check_error(unsafe {
            syz_createSource3D(&mut h as *mut syz_Handle, context.to_syz_handle())
        })?;
        Ok(Source3D(Handle(h)))
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
