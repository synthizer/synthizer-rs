use std::io::{Read, Seek};
use std::os::raw::{c_char, c_longlong, c_uint, c_ulonglong};
use std::path::Path;

use crate::internal_prelude::*;
use crate::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Buffer(pub(crate) Handle);

impl Buffer {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Buffer> {
        let path = path.as_ref();
        let u_str = path
            .to_str()
            .ok_or_else(|| Error::rust_error("Path is not valid utf8"))?;
        let c_str = std::ffi::CString::new(u_str)
            .map_err(|_| Error::rust_error("Path contains a NULL byte"))?;

        wrap_constructor(|ud, cb| {
            let mut h = Default::default();
            check_error(unsafe {
                syz_createBufferFromFile(&mut h as *mut syz_Handle, c_str.as_ptr(), ud, Some(cb))
            })?;
            Ok(Buffer(Handle::new(h)))
        })
    }

    pub fn from_encoded_data(data: &[u8]) -> Result<Buffer> {
        let mut h = Default::default();
        check_error(unsafe {
            syz_createBufferFromEncodedData(
                &mut h as *mut syz_Handle,
                data.len() as c_ulonglong,
                &data[0] as *const u8 as *const i8,
                std::ptr::null_mut(),
                None,
            )
        })?;
        Ok(Buffer(Handle::new(h)))
    }

    pub fn from_float_array(sr: c_uint, channels: c_uint, data: &[f32]) -> Result<Buffer> {
        if data.is_empty() {
            return Err(Error::rust_error(
                "Cannot create a buffer from an empty array",
            ));
        }
        if channels == 0 {
            return Err(Error::rust_error("channels may not be 0"));
        }
        if data.len() % channels as usize != 0 {
            return Err(Error::rust_error(
                "Length of data must be a multiple of the channel count",
            ));
        }

        let mut h = Default::default();
        check_error(unsafe {
            syz_createBufferFromFloatArray(
                &mut h as *mut syz_Handle,
                sr,
                channels,
                data.len() as c_ulonglong / channels as c_ulonglong,
                &data[0] as *const f32,
                std::ptr::null_mut(),
                None,
            )
        })?;
        Ok(Buffer(Handle::new(h)))
    }

    pub fn from_stream_handle(&self, handle: StreamHandle) -> Result<Buffer> {
        let mut h = Default::default();
        check_error(unsafe {
            syz_createBufferFromStreamHandle(
                &mut h as *mut syz_Handle,
                handle.get_handle(),
                std::ptr::null_mut(),
                None,
            )
        })?;
        // No need to link: buffers consume the stream entirely in the calling thread.
        Ok(Buffer(Handle::new(h)))
    }

    pub fn from_stream_params(protocol: &str, path: &str, param: usize) -> Result<Buffer> {
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
            syz_createBufferFromStreamParams(
                &mut h as *mut syz_Handle,
                protocol_ptr as *const c_char,
                path_ptr as *const c_char,
                std::mem::transmute(param),
                std::ptr::null_mut(),
                None,
            )
        })?;
        Ok(Buffer(Handle::new(h)))
    }

    /// Decode a buffer from a `Read + Seek` implementation.
    ///
    /// This avoids the Sync and 'static requirements on going through a stream
    /// handle.
    ///
    /// We require `Seek` as well because there are some formats that Synthizer
    /// can't decode without it, most notably wav.
    pub fn from_read_seek<R: Read + Seek>(mut reader: R) -> Result<Buffer> {
        let size = reader
            .seek(std::io::SeekFrom::End(0))
            .and_then(|_| reader.stream_position())
            .and_then(|_| reader.seek(std::io::SeekFrom::Start(0)))
            .map_err(|x| Error::rust_error(&format!("{}", x)))?;

        let cdata = CustomStreamData {
            err_msg: Default::default(),
            stream: reader,
        };

        let mut boxed = Box::new(cdata);

        let mut sdef = syz_CustomStreamDef {
            read_cb: Some(stream_read_cb::<R>),
            seek_cb: Some(stream_seek_cb::<R>),
            length: size as c_longlong,
            userdata: &mut *boxed as *mut CustomStreamData<R> as *mut std::ffi::c_void,
            ..Default::default()
        };

        let mut sh = 0;
        check_error(unsafe {
            syz_createStreamHandleFromCustomStream(
                &mut sh as *mut syz_Handle,
                &mut sdef as *mut syz_CustomStreamDef,
                null_mut(),
                None,
            )
        })?;
        // Immediately put this behind a handle that doesn't escape this
        // function, which ensures that we always drop it when we go out of
        // scope.
        let sh = Handle::new(sh);

        wrap_constructor(|ud, cb| {
            let mut h = Default::default();
            check_error(unsafe {
                syz_createBufferFromStreamHandle(
                    &mut h as *mut syz_Handle,
                    sh.to_syz_handle(),
                    ud,
                    Some(cb),
                )
            })?;
            Ok(Buffer(Handle::new(h)))
        })
    }

    pub fn get_length_in_samples(&self) -> Result<u32> {
        let mut out = Default::default();
        check_error(unsafe {
            syz_bufferGetLengthInSamples(&mut out as *mut u32, self.to_syz_handle())
        })?;
        Ok(out)
    }

    pub fn get_length_in_seconds(&self) -> Result<f64> {
        let mut out = Default::default();
        check_error(unsafe {
            syz_bufferGetLengthInSeconds(&mut out as *mut f64, self.to_syz_handle())
        })?;
        Ok(out)
    }

    pub fn get_channels(&self) -> Result<u32> {
        let mut out = Default::default();
        check_error(unsafe { syz_bufferGetChannels(&mut out as *mut u32, self.to_syz_handle()) })?;
        Ok(out)
    }

    pub fn get_size_in_bytes(&self) -> Result<u64> {
        let mut out = 0;
        check_error(unsafe {
            syz_bufferGetSizeInBytes(&mut out as *mut u64, self.to_syz_handle())
        })?;
        Ok(out)
    }
    object_common!();
}

handle_traits!(Buffer);
