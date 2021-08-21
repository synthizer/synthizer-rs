use crate::internal_prelude::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct DirectSource(pub(crate) Handle);

impl DirectSource {
    pub fn new(context: &Context) -> Result<DirectSource> {
        wrap_constructor(|ud, cb| {
            let mut h = Default::default();
            check_error(unsafe {
                syz_createDirectSource(
                    &mut h as *mut syz_Handle,
                    context.to_syz_handle(),
                    ud,
                    Some(cb),
                )
            })?;
            Ok(DirectSource(Handle::new(h)))
        })
    }

    source_properties!();

    object_common!();
    pausable_common!();
    source_common!();
}

handle_traits!(DirectSource);
