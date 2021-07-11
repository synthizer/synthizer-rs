//! Demonstrates custom streams by implementing one that wraps a file.
use std::fs::File;

use anyhow::Result;

use synthizer as syz;

/// Helper struct to get around Rust not letting us implement these traits for
/// `File` directly.
struct FileStream(File);

impl syz::CloseStream for FileStream {
    fn close(&mut self) -> Result<(), Box<dyn std::fmt::Display>> {
        // Stdlib files don't panic in Drop, so just let Synthizer drop it.
        Ok(())
    }
}

impl std::io::Read for FileStream {
    fn read(&mut self, dest: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(dest)
    }
}

impl std::io::Seek for FileStream {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.0.seek(pos)
    }
}
fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return Ok(());
    }

    let file = std::fs::File::open(args[1].as_str())?;

    let mut cfg = syz::LibraryConfig::default();
    cfg.log_level(syz::LogLevel::Debug);
    cfg.log_to_stderr();
    let _guard = cfg.initialize()?;

    let ctx = synthizer::Context::new()?;
    let stream_def = synthizer::CustomStreamDef::from_seek(FileStream(file))?;
    let stream = synthizer::StreamHandle::from_stream_def(stream_def)?;
    let gen = syz::StreamingGenerator::from_stream_handle(&ctx, stream)?;
    let src = syz::DirectSource::new(&ctx)?;
    src.add_generator(&gen)?;

    println!("Press enter to exit");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    Ok(())
}
