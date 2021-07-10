use std::sync::Arc;

use synthizer_sys::*;

use crate::casting::*;
use crate::context::*;
use crate::errors::*;
use crate::generator::*;
use crate::handle::*;
use crate::*;

#[derive(Clone)]
pub struct DirectSource(pub(crate) Handle);

impl DirectSource {
    pub fn new(context: &Context) -> Result<DirectSource> {
        let mut h = Default::default();
        check_error(unsafe {
            syz_createDirectSource(&mut h as *mut syz_Handle, context.to_syz_handle())
        })?;
        Ok(DirectSource(Handle::new(h)))
    }

    source_properties!();

    object_common!();
    pausable_common!();
    source_common!();
}

handle_traits!(DirectSource);
