use std::sync::Arc;

use synthizer_sys::*;

use crate::casting::*;
use crate::context::*;
use crate::errors::*;
use crate::generator::*;
use crate::handle::*;
use crate::*;

#[derive(Clone)]
pub struct PannedSource(pub(crate) Handle);

impl PannedSource {
    pub fn new(context: &Context) -> Result<PannedSource> {
        let mut h = Default::default();
        check_error(unsafe {
            syz_createPannedSource(&mut h as *mut syz_Handle, context.to_syz_handle())
        })?;
        Ok(PannedSource(Handle::new(h)))
    }

    source_properties!();
    enum_p!(
        PannerStrategy,
        SYZ_P_PANNER_STRATEGY,
        get_panner_strategy,
        set_panner_strategy
    );
    double_p!(SYZ_P_ELEVATION, get_elevation, set_elevation);
    double_p!(SYZ_P_AZIMUTH, get_azimuth, set_azimuth);
    double_p!(SYZ_P_PANNING_SCALAR, get_panning_scalar, set_panning_scalar);

    object_common!();
    pausable_common!();
    source_common!();
}

handle_traits!(PannedSource);
