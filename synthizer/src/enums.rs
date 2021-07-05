//! Holds enums that don't clearly belong to any specific moudle
use synthizer_sys::*;

#[repr(i32)]
pub enum PannerStrategy {
    Hrtf = SYZ_PANNER_STRATEGY_HRTF as i32,
    Stereo = SYZ_PANNER_STRATEGY_STEREO as i32,
}
