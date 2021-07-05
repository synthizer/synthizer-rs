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
