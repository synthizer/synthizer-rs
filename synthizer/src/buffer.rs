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
            syz_createBufferFromFile(&mut h as *mut syz_Handle, c_str.as_ptr())
        })?;
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
