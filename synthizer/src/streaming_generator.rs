use std::os::raw::c_char;
use std::path::Path;

use crate::internal_prelude::*;

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
        let mut h = handle.get_handle();
        let (ud, ud_free) = handle.get_userdata();
        check_error(unsafe {
            syz_createStreamingGeneratorFromStreamHandle(
                &mut h as *mut syz_Handle,
                context.to_syz_handle(),
                handle.get_handle(),
                ud,
                ud_free,
            )
        })?;
        handle.consume();
        let ret = StreamingGenerator(Handle::new(h));
        Ok(ret)
    }

    pub fn from_stream_params(
        context: &Context,
        protocol: &str,
        path: &str,
        param: usize,
    ) -> Result<StreamingGenerator> {
        // The below transmute uses the fact that `usize` is the size of a
        // pointer on all common platforms.
        let mut h = Default::default();
        let protocol_c = std::ffi::CString::new(protocol)
            .map_err(|_| Error::rust_error("Unable to convert protocol to a C string"))?;
        let path_c = std::ffi::CString::new(path)
            .map_err(|_| Error::rust_error("Unable to convert path to a C string"))?;
        let protocol_ptr = protocol_c.as_ptr();
        let path_ptr = path_c.as_ptr();
        check_error(unsafe {
            syz_createStreamingGeneratorFromStreamParams(
                &mut h as *mut syz_Handle,
                context.to_syz_handle(),
                protocol_ptr as *const c_char,
                path_ptr as *const c_char,
                std::mem::transmute(param),
                std::ptr::null_mut(),
                None,
            )
        })?;
        Ok(StreamingGenerator(Handle::new(h)))
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
