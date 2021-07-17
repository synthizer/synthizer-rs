use crate::internal_prelude::*;

#[derive(Clone)]
pub struct BufferGenerator(pub(crate) Handle);

impl BufferGenerator {
    pub fn new(context: &Context) -> Result<BufferGenerator> {
        let mut h = 0;
        check_error(unsafe {
            syz_createBufferGenerator(
                &mut h as *mut u64,
                context.to_syz_handle(),
                std::ptr::null_mut(),
                None,
            )
        })?;
        Ok(BufferGenerator(Handle::new(h)))
    }

    generator_properties!();
    bool_p!(SYZ_P_LOOPING, get_looping, set_looping);
    double_p!(
        SYZ_P_PLAYBACK_POSITION,
        get_playback_position,
        set_playback_position
    );
    object_p!(SYZ_P_BUFFER, set_buffer);

    object_common!();
    pausable_common!();
}

handle_traits!(BufferGenerator);
