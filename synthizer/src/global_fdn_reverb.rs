use synthizer_sys::*;

use crate::context::*;
use crate::errors::*;
use crate::handle::*;
use crate::*;

#[derive(Clone)]
pub struct GlobalFdnReverb(Handle);

impl GlobalFdnReverb {
    pub fn new(context: &Context) -> Result<GlobalFdnReverb> {
        let mut h = Default::default();
        check_error(unsafe {
            syz_createGlobalFdnReverb(&mut h as *mut syz_Handle, context.to_syz_handle())
        })?;
        Ok(GlobalFdnReverb(Handle(h)))
    }

    effect_properties!();
    double_p!(SYZ_P_MEAN_FREE_PATH, get_mean_free_path, set_mean_free_path);
    double_p!(SYZ_P_T60, get_t60, set_t60);
    double_p!(
        SYZ_P_LATE_REFLECTIONS_LF_ROLLOFF,
        get_late_reflections_lf_rolloff,
        set_late_reflections_lf_rolloff
    );
    double_p!(
        SYZ_P_LATE_REFLECTIONS_LF_REFERENCE,
        get_late_reflections_lf_reference,
        set_late_reflections_lf_reference
    );
    double_p!(
        SYZ_P_LATE_REFLECTIONS_HF_ROLLOFF,
        get_late_reflections_hf_rolloff,
        set_late_reflections_hf_rolloff
    );
    double_p!(
        SYZ_P_LATE_REFLECTIONS_HF_REFERENCE,
        get_late_reflections_hf_reference,
        set_late_reflections_hf_reference
    );
    double_p!(
        SYZ_P_LATE_REFLECTIONS_DIFFUSION,
        get_late_reflections_diffusion,
        set_late_reflections_diffusion
    );
    double_p!(
        SYZ_P_LATE_REFLECTIONS_MODULATION_DEPTH,
        get_late_reflections_modulation_depth,
        set_late_reflections_modulation_depth
    );
    double_p!(
        SYZ_P_LATE_REFLECTIONS_MODULATION_FREQUENCY,
        get_late_reflections_modulation_frequency,
        set_late_reflections_modulation_frequency
    );
    double_p!(
        SYZ_P_LATE_REFLECTIONS_DELAY,
        get_late_reflections_delay,
        set_late_reflections_delay
    );

    effect_common!();
}

impl ToSyzHandle for GlobalFdnReverb {
    fn to_syz_handle(&self) -> syz_Handle {
        self.0 .0
    }
}
