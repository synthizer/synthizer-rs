use std::os::raw::{c_uint, c_ulonglong};
use std::path::Path;
use std::sync::Arc;

use synthizer_sys::*;

use crate::casting::*;
use crate::errors::*;
use crate::handle::*;
use crate::*;

#[derive(Clone)]
pub struct Buffer(pub(crate) Handle);

impl Buffer {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Buffer> {
        let path = path.as_ref();
        let u_str = path
            .to_str()
            .ok_or_else(|| Error::rust_error("Path is not valid utf8"))?;
        let c_str = std::ffi::CString::new(u_str)
            .map_err(|_| Error::rust_error("Path contains a NULL byte"))?;

        let mut h = Default::default();
        check_error(unsafe {
            syz_createBufferFromFile(
                &mut h as *mut syz_Handle,
                c_str.as_ptr(),
                std::ptr::null_mut(),
                None,
            )
        })?;
        Ok(Buffer(Handle::new(h)))
    }

    pub fn from_encoded_data(data: &&[u8]) -> Result<Buffer> {
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

    object_common!();
}

handle_traits!(Buffer);
