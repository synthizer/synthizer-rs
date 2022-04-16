//! Implement casting (upcasting and downcasting) for Synthizer objects. Allows
//! converting any object to a handle, and upcasting/downcasting along the
//! inheritance hierarchy.
use crate::handle::*;
use crate::*;

/// Set up the list of base classes and casting functionality.
macro_rules! downcast {
    // Generate downcasting functionality to convert `t1` to `t2`.
    ($t1: ty, $t2: ty) => {
        impl From<$t1> for $t2 {
            fn from(input: $t1) -> $t2 {
                <$t2>::from_handle_internal(input.into_handle())
            }
        }

        impl From<&$t1> for $t2 {
            fn from(other: &$t1) -> $t2 {
                <$t2>::from_handle_internal(other.handle_ref().clone())
            }
        }
    };

    // And this one to support running it on a whole list of types, from most to least derived.
    ($t1: ty, $t2: ty, $($tys: ty),+) => {
        downcast!($t1, $t2);
        downcast!($t1, $($tys),*);
    }
}

downcast!(Buffer, Handle);

downcast!(Generator, Handle);
downcast!(BufferGenerator, Generator, Handle);
downcast!(NoiseGenerator, Generator, Handle);
downcast!(StreamingGenerator, Generator, Handle);

downcast!(Source, Handle);
downcast!(DirectSource, Source, Handle);
downcast!(AngularPannedSource, Source, Handle);
downcast!(ScalarPannedSource, Source, Handle);
downcast!(Source3D, Source, Handle);

mod cast_target {
    use super::*;

    pub trait CastTarget: Sized {
        fn cast_from(h: &Handle) -> Result<Option<Self>>;
    }
}

impl CastTarget for Handle {
    fn cast_from(h: &Handle) -> Result<Option<Handle>> {
        Ok(Some(h.handle_ref().clone()))
    }
}

pub(crate) use cast_target::*;

/// Macro to allow trying to cast to a given type from a list of type constants
/// it supports.
macro_rules! cast {
    ($t: ty, $($otypes: expr),+) => {
        impl CastTarget for $t {
            fn cast_from(h: &Handle) -> Result<Option<$t>> {
                let otype = h.get_type()?;
                for i in [$($otypes),*].iter() {
                    if i == &otype {
                        return Ok(Some(<$t>::from_handle_internal((*h).clone())));
                    }
                }
                Ok(None)
            }
        }
    }
}

cast!(Buffer, ObjectType::Buffer);

cast!(
    Generator,
    ObjectType::BufferGenerator,
    ObjectType::StreamingGenerator,
    ObjectType::NoiseGenerator,
    ObjectType::FastSineBankGenerator
);
cast!(BufferGenerator, ObjectType::BufferGenerator);
cast!(NoiseGenerator, ObjectType::NoiseGenerator);
cast!(StreamingGenerator, ObjectType::StreamingGenerator);
cast!(FastSineBankGenerator, ObjectType::FastSineBankGenerator);

cast!(
    Source,
    ObjectType::DirectSource,
    ObjectType::AngularPannedSource,
    ObjectType::ScalarPannedSource,
    ObjectType::Source3D
);
cast!(DirectSource, ObjectType::DirectSource);
cast!(AngularPannedSource, ObjectType::AngularPannedSource);
cast!(ScalarPannedSource, ObjectType::ScalarPannedSource);
cast!(Source3D, ObjectType::Source3D);

// Macro to punch out a bunch of `TryFrom` impls.
macro_rules! try_from {
    ($t: ty, $from: ty) => {
        impl std::convert::TryFrom<&$from> for $t {
            type Error = crate::errors::Error;
            fn try_from(other: &$from) -> Result<$t> {
                <$t>::cast_from(other.handle_ref())?.ok_or_else(|| crate::errors::Error::rust_error("Type mismatch"))
            }
        }


    impl std::convert::TryFrom<$from> for $t {
        type Error = crate::errors::Error;
        fn try_from(other: $from) -> Result<$t> {
            <$t>::cast_from(other.handle_ref())?.ok_or_else(|| crate::errors::Error::rust_error("Type mismatch"))
        }
    }
    };
    ($t: ty, $t2: ty, $($ts:ty),+) => {
        try_from!($t, $t2);
        try_from!($t, $($ts),*);
    }
}

try_from!(Buffer, Handle);

try_from!(Generator, Handle);
try_from!(BufferGenerator, Generator, Handle);
try_from!(NoiseGenerator, Generator, Handle);
try_from!(StreamingGenerator, Generator, Handle);
try_from!(FastSineBankGenerator, Generator, Handle);

try_from!(Source, Handle);
try_from!(DirectSource, Source, Handle);
try_from!(AngularPannedSource, Source, Handle);
try_from!(ScalarPannedSource, Source, Handle);
try_from!(Source3D, Source, Handle);
