use crate::internal_prelude::*;

#[derive(Clone)]
pub struct DirectSource(pub(crate) Handle);

impl DirectSource {
    pub fn new(context: &Context) -> Result<DirectSource> {
        let mut h = Default::default();
        check_error(unsafe {
            syz_createDirectSource(
                &mut h as *mut syz_Handle,
                context.to_syz_handle(),
                std::ptr::null_mut(),
                None,
            )
        })?;
        Ok(DirectSource(Handle::new(h)))
    }

    source_properties!();

    object_common!();
    pausable_common!();
    source_common!();
}

handle_traits!(DirectSource);
