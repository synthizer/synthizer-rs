use crate::handle::*;
use crate::*;

/// A marker trait for anything that can be a generator.
pub trait Generator: ToHandle {}

// These traits are about making it impossible to do things we know aren't
// possible.  Since Synthizer also validates, let's allow people to use `Handle`
// as a generator.
impl Generator for Handle {}

impl Generator for BufferGenerator {}
impl Generator for NoiseGenerator {}
