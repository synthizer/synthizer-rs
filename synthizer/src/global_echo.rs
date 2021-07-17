use crate::internal_prelude::*;

#[derive(Clone)]
pub struct GlobalEcho(pub(crate) Handle);

/// Re-exported Synthizer `syz_EchoTapConfig` type.  Using this instead of a
/// dedicated struct prevents needing to clone your taps on the way to
/// Synthizer.
pub type EchoTapConfig = syz_EchoTapConfig;

impl GlobalEcho {
    pub fn new(context: &Context) -> Result<GlobalEcho> {
        let mut h = Default::default();
        check_error(unsafe {
            syz_createGlobalEcho(&mut h, context.to_syz_handle(), std::ptr::null_mut(), None)
        })?;
        Ok(GlobalEcho(Handle::new(h)))
    }

    /// An empty slice clears the taps. Alternatively, you can use `clear_taps`.
    pub fn set_taps(&self, taps: &[EchoTapConfig]) -> Result<()> {
        if taps.is_empty() {
            return self.clear_taps();
        }

        check_error(unsafe {
            syz_globalEchoSetTaps(
                self.to_syz_handle(),
                taps.len() as u32,
                &taps[0] as *const syz_EchoTapConfig,
            )
        })
    }

    pub fn clear_taps(&self) -> Result<()> {
        check_error(unsafe { syz_globalEchoSetTaps(self.to_syz_handle(), 0, std::ptr::null()) })
    }

    effect_properties!();

    object_common!();
}

handle_traits!(GlobalEcho);
