use std::path::Path;

use synthizer_sys::*;

use crate::errors::*;
use crate::handle::*;
use crate::*;

#[derive(Clone)]
pub struct StreamingGenerator(Handle);

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
            syz_createStreamingGeneratorFromFile(&mut h, context.to_handle(), c_str.as_ptr())
        })?;
        Ok(StreamingGenerator(Handle(h)))
    }
}

impl ToHandle for StreamingGenerator {
    fn to_handle(&self) -> syz_Handle {
        self.0 .0
    }
}