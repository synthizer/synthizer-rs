use synthizer_sys::*;

use crate::casting::*;
use crate::handle::*;
use crate::*;

/// Represents the generator "base class".  A [From] impl lets you get to this
/// object from any kind of generator.
#[derive(Clone)]
pub struct Generator(pub(crate) Handle);

impl Generator {
    object_common!();
    pausable_common!();
    generator_properties!();
}

handle_traits!(Generator);

/// A marker trait for anything that can be a generator.
pub trait IsGenerator: ToSyzHandle {}

// These traits are about making it impossible to do things we know aren't
// possible.  Since Synthizer also validates, let's allow people to use `Handle`
// as a generator.
impl IsGenerator for Handle {}

impl IsGenerator for BufferGenerator {}
impl IsGenerator for NoiseGenerator {}
impl IsGenerator for StreamingGenerator {}
