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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
#[repr(i32)]
pub enum NoiseType {
    Uniform = SYZ_NOISE_TYPE_UNIFORM as i32,
    Vm = SYZ_NOISE_TYPE_VM as i32,
    FilteredBrown = SYZ_NOISE_TYPE_FILTERED_BROWN as i32,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(i32)]
pub enum ObjectType {
    Context = SYZ_OTYPE_CONTEXT as i32,
    Buffer = SYZ_OTYPE_BUFFER as i32,
    BufferGenerator = SYZ_OTYPE_BUFFER_GENERATOR as i32,
    StreamingGenerator = SYZ_OTYPE_STREAMING_GENERATOR as i32,
    NoiseGenerator = SYZ_OTYPE_NOISE_GENERATOR as i32,
    DirectSource = SYZ_OTYPE_DIRECT_SOURCE as i32,
    PannedSource = SYZ_OTYPE_PANNED_SOURCE as i32,
    Source3D = SYZ_OTYPE_SOURCE_3D as i32,
    GlobalEcho = SYZ_OTYPE_GLOBAL_ECHO as i32,
    GlobalFdnReverb = SYZ_OTYPE_GLOBAL_FDN_REVERB as i32,
    StreamHandle = SYZ_OTYPE_STREAM_HANDLE as i32,
}
