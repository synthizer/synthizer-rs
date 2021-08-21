use crate::internal_prelude::*;

/// Represents the source "base class".  It is possible to convert to this type
/// from any source, in order to use source-common functionality.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Source(pub(crate) Handle);

impl Source {
    source_properties!();

    object_common!();
    pausable_common!();
    source_common!();
}

handle_traits!(Source);
