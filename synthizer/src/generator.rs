use crate::handle::*;
use crate::*;

/// A marker trait for anything that can be a generator.
pub trait Generator: ToHandle {}

impl Generator for BufferGenerator {}
