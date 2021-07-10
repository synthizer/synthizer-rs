//! Implement casting (upcasting and downcasting) for Synthizer objects. Allows converting any object to a handle, and upcasting/downcasting along the inheritance hierarchy.
use crate::*;

/// Set up the list of base classes and casting functionality.  The
macro_rules! cast {
    // Generate downcasting functionality to convert `t1` to `t2`.
    ($t1: ty, $t2: ty) => {
        impl From<$t1> for $t2 {
            fn from(input: $t1) -> $t2 {
                <$t2>::from_handle_internal(input.into_handle())
            }
        }
    };

    // And this one to support running it on a whole list of types, from most to least derived.
    ($t1: ty, $t2: ty, $($tys: ty),+) => {
        cast!($t1, $t2);
        cast!($t1, $($tys),*);
    }
}

cast!(Buffer, Handle);

cast!(Generator, Handle);
cast!(BufferGenerator, Generator, Handle);
cast!(NoiseGenerator, Generator, Handle);
cast!(StreamingGenerator, Generator, Handle);

cast!(Source, Handle);
cast!(DirectSource, Source, Handle);
cast!(PannedSource, Source, Handle);
cast!(Source3D, Source, Handle);
