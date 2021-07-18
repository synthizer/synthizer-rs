# Synthizer

![CI Status](https://github.com/synthizer/synthizer-rs/actions/workflows/ci.yaml/badge.svg)
[GitHub Sponsors](https://github.com/sponsors/ahicks92)

Current targeted Synthizer version: 0.10.0

Official, high-level bindings to
[Synthizer](https://github.com/synthizer/synthizer), a library for 3D audio and
effects.

Synthizer itself has a [language-agnostic/C manual](https://synthizer.github.io)
which generally covers Synthizer conceptually.  These bindings generally map
one-for-one in fairly obvious ways.  See Rustdoc (docs.rs build pending) or this
repository for Rust-specific examples and instructions for use; we don't put a
lot of examples in this readme inorder to benefit from doctests.  But it's as
easy as:

```
//! Play a file by loading it into a buffer.
use synthizer as syz;

fn main() -> syz::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Usage: example <file>");
    }

    let _init_guard = syz::initialize()?;
    let context = syz::Context::new()?;
    let src = syz::DirectSource::new(&context)?;
    let generator = syz::BufferGenerator::new(&context)?;
    let buffer = syz::Buffer::from_file(args[1].as_str())?;
    generator.set_buffer(&buffer)?;
    src.add_generator(&generator)?;

    println!("Press enter to exit");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    Ok(())
}
```

With the additional complexity being justified by things such as multiple
generators per source, being able to reuse buffers, etc.


## Building

This should build out of the box on most platforms, but Windows may have
difficulties.  If you can't build on Windows, try building under an MSVC shell,
and if that doesn't work open an issue.
