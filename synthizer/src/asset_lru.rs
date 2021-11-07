use std::io::{BufReader, Read, Seek};

use ::asset_lru::Decoder;

use crate::*;

/// A decoder implementation for the `asset_lru` crate, which can be used to
/// intelligently decode buffers.
#[derive(Debug, Default)]
pub struct BufferAssetLruDecoder;

impl BufferAssetLruDecoder {
    pub fn new() -> BufferAssetLruDecoder {
        BufferAssetLruDecoder
    }
}

struct WrappedReader<T: 'static + Send + Read>(T);

impl<T: Read + 'static + Send> Read for WrappedReader<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl<T: Read + Send + 'static> CloseStream for WrappedReader<T> {
    fn close(&mut self) -> std::result::Result<(), Box<dyn std::fmt::Display>> {
        Ok(())
    }
}

impl Decoder for BufferAssetLruDecoder {
    type Error = Error;
    type Output = Buffer;

    fn decode<R: Read + Seek>(&self, reader: R) -> Result<Buffer> {
        let br = BufReader::new(reader);
        Buffer::from_read_seek(br)
    }

    fn estimate_cost(&self, buffer: &Buffer) -> Result<u64> {
        buffer.get_size_in_bytes()
    }
}
