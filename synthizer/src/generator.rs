use crate::handle::*;
use crate::*;

/// A marker trait for anything that can be a generator.
pub trait IsGenerator: ToSyzHandle {}

// These traits are about making it impossible to do things we know aren't
// possible.  Since Synthizer also validates, let's allow people to use `Handle`
// as a generator.
impl IsGenerator for Handle {}

impl IsGenerator for BufferGenerator {}
impl IsGenerator for NoiseGenerator {}
impl IsGenerator for StreamingGenerator {}
