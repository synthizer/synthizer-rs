use synthizer_sys::*;

use crate::generator::*;
use crate::handle::*;
use crate::*;

/// Represents the source "base class".  It is possible to convert to this type
/// from any source, in order to use source-common functionality.
#[derive(Clone)]
pub struct Source(pub(crate) Handle);

impl Source {
    source_properties!();

    object_common!();
    pausable_common!();
    source_common!();
}

impl ToSyzHandle for Source {
    fn to_syz_handle(&self) -> syz_Handle {
        self.0 .0
    }
}
