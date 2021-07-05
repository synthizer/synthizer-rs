//! Holds enums that don't clearly belong to any specific moudle
use synthizer_sys::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(i32)]
pub enum PannerStrategy {
    Hrtf = SYZ_PANNER_STRATEGY_HRTF as i32,
    Stereo = SYZ_PANNER_STRATEGY_STEREO as i32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(i32)]
pub enum DistanceModel {
    None = SYZ_DISTANCE_MODEL_NONE as i32,
    Linear = SYZ_DISTANCE_MODEL_LINEAR as i32,
    Exponential = SYZ_DISTANCE_MODEL_EXPONENTIAL as i32,
    Inverse = SYZ_DISTANCE_MODEL_INVERSE as i32,
}
