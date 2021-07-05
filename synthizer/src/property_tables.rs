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
                Ok(unsafe { std::mem::transmute(out) })
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

macro_rules! double3_p {
    ($t: ty, $syz_const: expr, $getter: ident, $setter: ident) => {
        impl $t {
            pub fn $getter(&self) -> Result<(f64, f64, f64)> {
                let mut o1 = Default::default();
                let mut o2 = Default::default();
                let mut o3 = Default::default();
                check_error(unsafe {
                    syz_getD3(
                        &mut o1 as *mut f64,
                        &mut o2 as *mut f64,
                        &mut o3 as *mut f64,
                        self.to_handle(),
                        $syz_const as i32,
                    )
                })?;
                Ok((o1, o2, o3))
            }

            pub fn $setter(&self, values: (f64, f64, f64)) -> Result<()> {
                check_error(unsafe {
                    syz_setD3(
                        self.to_handle(),
                        $syz_const as i32,
                        values.0,
                        values.1,
                        values.2,
                    )
                })
            }
        }
    };
}

macro_rules! double6_p {
    ($t: ty, $syz_const: expr, $getter: ident, $setter: ident) => {
        impl $t {
            pub fn $getter(&self) -> Result<(f64, f64, f64, f64, f64, f64)> {
                let mut o1 = Default::default();
                let mut o2 = Default::default();
                let mut o3 = Default::default();
                let mut o4 = Default::default();
                let mut o5 = Default::default();
                let mut o6 = Default::default();
                check_error(unsafe {
                    syz_getD6(
                        &mut o1 as *mut f64,
                        &mut o2 as *mut f64,
                        &mut o3 as *mut f64,
                        &mut o4 as *mut f64,
                        &mut o5 as *mut f64,
                        &mut o6 as *mut f64,
                        self.to_handle(),
                        $syz_const as i32,
                    )
                })?;
                Ok((o1, o2, o3, o4, o5, o6))
            }

            pub fn $setter(&self, values: (f64, f64, f64, f64, f64, f64)) -> Result<()> {
                check_error(unsafe {
                    syz_setD6(
                        self.to_handle(),
                        $syz_const as i32,
                        values.0,
                        values.1,
                        values.2,
                        values.3,
                        values.4,
                        values.5,
                    )
                })
            }
        }
    };
}

double_p!(Context, SYZ_P_GAIN, get_gain, set_gain);
enum_p!(
    Context,
    PannerStrategy,
    SYZ_P_DEFAULT_PANNER_STRATEGY,
    get_default_panner_strategy,
    set_default_panner_strategy
);
enum_p!(
    Context,
    DistanceModel,
    SYZ_P_DEFAULT_DISTANCE_MODEL,
    get_default_distance_model,
    set_default_distance_model
);
double_p!(
    Context,
    SYZ_P_DEFAULT_DISTANCE_REF,
    get_default_distance_ref,
    set_default_distance_ref
);
double_p!(
    Context,
    SYZ_P_DEFAULT_DISTANCE_MAX,
    get_default_distance_max,
    set_default_distance_max
);
double_p!(
    Context,
    SYZ_P_DEFAULT_ROLLOFF,
    get_default_rolloff,
    set_default_rolloff
);
double_p!(
    Context,
    SYZ_P_DEFAULT_CLOSENESS_BOOST,
    get_default_closeness_boost,
    set_default_closeness_boost
);
double_p!(
    Context,
    SYZ_P_DEFAULT_CLOSENESS_BOOST_DISTANCE,
    get_default_closeness_boost_distance,
    set_default_closeness_boost_distance
);
double3_p!(Context, SYZ_P_POSITION, get_position, set_position);
double6_p!(Context, SYZ_P_ORIENTATION, get_orientation, set_orientation);

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
enum_p!(
    PannedSource,
    PannerStrategy,
    SYZ_P_PANNER_STRATEGY,
    get_panner_strategy,
    set_panner_strategy
);
double_p!(PannedSource, SYZ_P_ELEVATION, get_elevation, set_elevation);
double_p!(PannedSource, SYZ_P_AZIMUTH, get_azimuth, set_azimuth);
double_p!(
    PannedSource,
    SYZ_P_PANNING_SCALAR,
    get_panning_scalar,
    set_panning_scalar
);

source_properties!(Source3D);
enum_p!(
    Source3D,
    DistanceModel,
    SYZ_P_DISTANCE_MODEL,
    get_distance_model,
    set_distance_model
);
double_p!(
    Source3D,
    SYZ_P_DISTANCE_REF,
    get_distance_ref,
    set_distance_ref
);
double_p!(
    Source3D,
    SYZ_P_DISTANCE_MAX,
    get_distance_max,
    set_distance_max
);
double_p!(Source3D, SYZ_P_ROLLOFF, get_rolloff, set_rolloff);
double_p!(
    Source3D,
    SYZ_P_CLOSENESS_BOOST,
    get_closeness_boost,
    set_closeness_boost
);
double_p!(
    Source3D,
    SYZ_P_CLOSENESS_BOOST_DISTANCE,
    get_closeness_boost_distance,
    set_closeness_boost_distance
);
double3_p!(Source3D, SYZ_P_POSITION, get_position, set_position);
double6_p!(
    Source3D,
    SYZ_P_ORIENTATION,
    get_orientation,
    set_orientation
);

generator_properties!(NoiseGenerator);
enum_p!(
    NoiseGenerator,
    NoiseType,
    SYZ_P_NOISE_TYPE,
    get_noise_type,
    set_noise_type
);

generator_properties!(StreamingGenerator);
double_p!(
    StreamingGenerator,
    SYZ_P_PLAYBACK_POSITION,
    get_playback_position,
    set_playback_position
);

macro_rules! effect_properties {
    ($t: ty) => {
        double_p!($t, SYZ_P_GAIN, get_gain, set_gain);
        biquad_p!($t, SYZ_P_FILTER_INPUT, get_filter_input, set_filter_input);
    };
}

effect_properties!(GlobalEcho);
