use std::path::Path;
use std::sync::Arc;

use synthizer_sys::*;

use crate::casting::*;
use crate::errors::*;
use crate::handle::*;
use crate::*;

#[derive(Clone)]
pub struct StreamingGenerator(pub(crate) Handle);

impl StreamingGenerator {
    pub fn from_file<P: AsRef<Path>>(context: &Context, path: P) -> Result<StreamingGenerator> {
        let path = path.as_ref();
        let u_str = path
            .to_str()
            .ok_or_else(|| Error::rust_error("Path is not valid utf8"))?;
        let c_str = std::ffi::CString::new(u_str)
            .map_err(|_| Error::rust_error("Path contains a NULL byte"))?;

        let mut h = Default::default();
        check_error(unsafe {
            syz_createStreamingGeneratorFromFile(
                &mut h,
                context.to_syz_handle(),
                c_str.as_ptr(),
                std::ptr::null_mut(),
                None,
            )
        })?;
        Ok(StreamingGenerator(Handle::new(h)))
    }

    pub fn from_stream_handle(
        context: &Context,
        handle: StreamHandle,
    ) -> Result<StreamingGenerator> {
        let mut h = Default::default();
        check_error(unsafe {
            syz_createStreamingGeneratorFromStreamHandle(
                &mut h as *mut syz_Handle,
                context.to_syz_handle(),
                handle.get_handle(),
                std::ptr::null_mut(),
                None,
            )
        })?;
        let ret = StreamingGenerator(Handle::new(h));
        handle.link(&ret.0)?;
        Ok(ret)
    }

    generator_properties!();
    double_p!(
        SYZ_P_PLAYBACK_POSITION,
        get_playback_position,
        set_playback_position
    );

    object_common!();
    pausable_common!();
}

handle_traits!(StreamingGenerator);
