use std::os::raw::c_char;
use std::path::Path;

use crate::internal_prelude::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct StreamingGenerator(pub(crate) Handle);

impl StreamingGenerator {
    pub fn from_file<P: AsRef<Path>>(context: &Context, path: P) -> Result<StreamingGenerator> {
        let path = path.as_ref();
        let u_str = path
            .to_str()
            .ok_or_else(|| Error::rust_error("Path is not valid utf8"))?;
        let c_str = std::ffi::CString::new(u_str)
            .map_err(|_| Error::rust_error("Path contains a NULL byte"))?;

        wrap_constructor(|ud, cb| {
            let mut h = Default::default();
            check_error(unsafe {
                syz_createStreamingGeneratorFromFile(
                    &mut h,
                    context.to_syz_handle(),
                    c_str.as_ptr(),
                    null_mut(),
                    ud,
                    Some(cb),
                )
            })?;
            Ok(StreamingGenerator(Handle::new(h)))
        })
    }

    pub fn from_stream_handle(
        context: &Context,
        handle: StreamHandle,
    ) -> Result<StreamingGenerator> {
        handle.with_userdata(move |sh, ud, cb| {
            let mut out = 0;
            check_error(unsafe {
                syz_createStreamingGeneratorFromStreamHandle(
                    &mut out as *mut syz_Handle,
                    context.to_syz_handle(),
                    sh,
                    null_mut(),
                    ud,
                    Some(cb),
                )
            })?;
            let ret = StreamingGenerator(Handle::new(out));
            Ok(ret)
        })
    }

    pub fn from_stream_params(
        context: &Context,
        protocol: &str,
        path: &str,
        param: usize,
    ) -> Result<StreamingGenerator> {
        // The below transmute uses the fact that `usize` is the size of a
        // pointer on all common platforms.
        wrap_constructor(|ud, cb| {
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
                    null_mut(),
                    ud,
                    Some(cb),
                )
            })?;
            Ok(StreamingGenerator(Handle::new(h)))
        })
    }

    generator_properties!();
    bool_p!(SYZ_P_LOOPING, get_looping, set_looping);
    double_p!(
        SYZ_P_PLAYBACK_POSITION,
        get_playback_position,
        set_playback_position
    );

    object_common!();
    pausable_common!();
}

handle_traits!(StreamingGenerator);
