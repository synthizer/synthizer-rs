//! Holds enums that don't clearly belong to any specific moudle
use synthizer_sys::*;

mod transmutable {
    /// Marker trait so that we can internally make sure that it's safe to
    /// transmute enums in generic contexts.  Guarantees that the enum came from
    /// us, and is backed by an i32.
    pub unsafe trait I32TransmutableEnum {
        /// Work around no from/into impls on enums with repr(i32).
        fn as_i32(&self) -> i32;
        // And work around transmuting the other way.
        unsafe fn from_i32(val: i32) -> Self;
    }
}

pub(crate) use transmutable::*;

macro_rules! impl_transmutable {
    ($t: ty) => {
        unsafe impl I32TransmutableEnum for $t {
            fn as_i32(&self) -> i32 {
                *self as i32
            }

            unsafe fn from_i32(val: i32) -> Self {
                std::mem::transmute(val)
            }
        }
    };
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(i32)]
pub enum PannerStrategy {
    Delegate = SYZ_PANNER_STRATEGY_DELEGATE as i32,
    Hrtf = SYZ_PANNER_STRATEGY_HRTF as i32,
    Stereo = SYZ_PANNER_STRATEGY_STEREO as i32,
}

impl_transmutable!(PannerStrategy);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(i32)]
pub enum DistanceModel {
    None = SYZ_DISTANCE_MODEL_NONE as i32,
    Linear = SYZ_DISTANCE_MODEL_LINEAR as i32,
    Exponential = SYZ_DISTANCE_MODEL_EXPONENTIAL as i32,
    Inverse = SYZ_DISTANCE_MODEL_INVERSE as i32,
}

impl_transmutable!(DistanceModel);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
#[repr(i32)]
pub enum NoiseType {
    Uniform = SYZ_NOISE_TYPE_UNIFORM as i32,
    Vm = SYZ_NOISE_TYPE_VM as i32,
    FilteredBrown = SYZ_NOISE_TYPE_FILTERED_BROWN as i32,
}

impl_transmutable!(NoiseType);

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(i32)]
pub enum ObjectType {
    AutomationBatch = SYZ_OTYPE_AUTOMATION_BATCH as i32,
    Context = SYZ_OTYPE_CONTEXT as i32,
    Buffer = SYZ_OTYPE_BUFFER as i32,
    BufferGenerator = SYZ_OTYPE_BUFFER_GENERATOR as i32,
    StreamingGenerator = SYZ_OTYPE_STREAMING_GENERATOR as i32,
    NoiseGenerator = SYZ_OTYPE_NOISE_GENERATOR as i32,
    DirectSource = SYZ_OTYPE_DIRECT_SOURCE as i32,
    AngularPannedSource = SYZ_OTYPE_ANGULAR_PANNED_SOURCE as i32,
    ScalarPannedSource = SYZ_OTYPE_SCALAR_PANNED_SOURCE as i32,
    Source3D = SYZ_OTYPE_SOURCE_3D as i32,
    GlobalEcho = SYZ_OTYPE_GLOBAL_ECHO as i32,
    GlobalFdnReverb = SYZ_OTYPE_GLOBAL_FDN_REVERB as i32,
    StreamHandle = SYZ_OTYPE_STREAM_HANDLE as i32,
}

impl_transmutable!(ObjectType);

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(i32)]
pub enum InterpolationType {
    None = SYZ_INTERPOLATION_TYPE_NONE as i32,
    Linear = SYZ_INTERPOLATION_TYPE_LINEAR as i32,
}

impl_transmutable!(InterpolationType);
