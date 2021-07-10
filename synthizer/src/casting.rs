//! Implement casting (upcasting and downcasting) for Synthizer objects. Allows converting any object to a handle, and upcasting/downcasting along the inheritance hierarchy.
use crate::*;

macro_rules! downcast {
    // Generate downcasting functionality to convert `t1` to `t2`.
    ($t1: ty, $t2: ty) => {
        impl From<$t1> for $t2 {
            fn from(input: $t1) -> $t2 {
                <$t2>::from_handle_internal(input.into_handle())
            }
        }
    };

    // And this one to support running it on a whole list of types.
    ($t1: ty, $t2: ty, $($tys: ty),+) => {
        downcast!($t1, $t2);
        downcast!($t1, $($tys),*);
    }
}

downcast!(Buffer, Handle);
downcast!(BufferGenerator, Handle);
downcast!(DirectSource, Handle);
downcast!(NoiseGenerator, Handle);
downcast!(PannedSource, Source, Handle);
downcast!(Source3D, Source, Handle);
downcast!(StreamingGenerator, Source, Handle);
