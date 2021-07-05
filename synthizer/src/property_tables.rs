use synthizer_sys::*;

use crate::errors::*;
use crate::handle::*;
use crate::*;

macro_rules! bool_p {
    ($t: ty, $syz_const: expr, $getter: ident, $setter: ident) => {
        impl $t {
            pub fn $getter(&self) -> Result<bool> {
                let mut out = Default::default();
                check_error(unsafe {
                    syz_getI(&mut out as *mut i32, self.to_handle(), $syz_const as i32)
                })?;
                Ok(out != 0)
            }

            pub fn $setter(&self, value: bool) -> Result<()> {
                check_error(unsafe {
                    syz_setI(self.to_handle(), $syz_const as i32, value as i32)
                })?;
                Ok(())
            }
        }
    };
}

macro_rules! enum_p {
    ($t: ty, $e: ty, $syz_const: expr, $getter: ident, $setter: ident) => {
        impl $t {
            pub fn $getter(&self) -> Result<$e> {
                let mut out = Default::default();
                check_error(unsafe {
                    syz_getI(&mut out as *mut i32, self.to_handle(), $syz_const as i32)
                })?;
                Ok(unsafe{std::mem::transmute(out)})
            }

            pub fn $setter(&self, value: $e) -> Result<()> {
                check_error(unsafe {
                    syz_setI(self.to_handle(), $syz_const as i32, value as i32)
                })?;
                Ok(())
            }
        }
    };
}

macro_rules! double_p {
    ($t: ty, $syz_const: expr, $getter: ident, $setter: ident) => {
        impl $t {
            pub fn $getter(&self) -> Result<f64> {
                let mut out = Default::default();
                check_error(unsafe {
                    syz_getD(&mut out as *mut f64, self.to_handle(), $syz_const as i32)
                })?;
                Ok(out)
            }

            pub fn $setter(&self, value: f64) -> Result<()> {
                check_error(unsafe { syz_setD(self.to_handle(), $syz_const as i32, value) })?;
                Ok(())
            }
        }
    };
}

macro_rules! object_p {
    ($t: ty, $syz_const: expr, $setter: ident) => {
        impl $t {
            pub fn $setter<T: ToHandle>(&self, obj: &T) -> Result<()> {
                check_error(unsafe {
                    syz_setO(self.to_handle(), $syz_const as i32, obj.to_handle())
                })?;
                Ok(())
            }
        }
    };
}

macro_rules! biquad_p {
    ($t: ty, $syz_const: expr, $getter:ident, $setter: ident) => {
        impl $t {
            pub fn $getter(&self) -> Result<BiquadConfig> {
                let mut out = Default::default();
                check_error(unsafe {
                    syz_getBiquad(
                        &mut out as *mut syz_BiquadConfig,
                        self.to_handle(),
                        $syz_const as i32,
                    )
                })?;
                Ok(BiquadConfig { cfg: out })
            }

            pub fn $setter(&self, cfg: &BiquadConfig) -> Result<()> {
                check_error(unsafe {
                    syz_setBiquad(
                        self.to_handle(),
                        $syz_const as i32,
                        &cfg.cfg as *const syz_BiquadConfig,
                    )
                })
            }
        }
    };
}

macro_rules! generator_properties {
    ($t: ty) => {
        double_p!($t, SYZ_P_PITCH_BEND, get_pitch_bend, set_pitch_bend);
        double_p!($t, SYZ_P_GAIN, get_gain, set_gain);
    };
}

generator_properties!(BufferGenerator);
bool_p!(BufferGenerator, SYZ_P_LOOPING, get_looping, set_looping);
double_p!(
    BufferGenerator,
    SYZ_P_PLAYBACK_POSITION,
    get_playback_position,
    set_playback_position
);
object_p!(BufferGenerator, SYZ_P_BUFFER, set_buffer);

macro_rules! source_properties {
    ($t: ty) => {
        double_p!($t, SYZ_P_GAIN, get_gain, set_gain);
        biquad_p!($t, SYZ_P_FILTER, get_filter, set_filter);
        biquad_p!(
            $t,
            SYZ_P_FILTER_DIRECT,
            get_filter_direct,
            set_filter_direct
        );
        biquad_p!(
            $t,
            SYZ_P_FILTER_EFFECTS,
            get_filter_effects,
            set_filter_effects
        );
    };
}

source_properties!(DirectSource);




source_properties!(PannedSource);
enum_p!(PannedSource, PannerStrategy, SYZ_P_PANNER_STRATEGY, get_panner_strategy, set_panner_strategy);
double_p!(PannedSource, SYZ_P_ELEVATION, get_elevation, set_elevation);
double_p!(PannedSource, SYZ_P_AZIMUTH, get_azimuth, set_azimuth);
double_p!(PannedSource, SYZ_P_PANNING_SCALAR, get_panning_scalar, set_panning_scalar);
