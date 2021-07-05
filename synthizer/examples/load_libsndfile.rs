//! Demonstrates loading Libsndfile.  This example takes two arguments; the first is a path to Libsndfile, and the second a path to a file to decode.
use synthizer as syz;

fn main() -> syz::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        panic!("Usage: {} <libsndfile-path> <file-path>", args[0]);
    }

    let mut init_config = syz::LibraryConfig::new();
    init_config.log_level(syz::LogLevel::Debug);
    init_config.log_to_stderr();
    init_config.load_libsndfile(args[1].as_str())?;
    let _init_guard = init_config.initialize()?;

    let context = syz::Context::new()?;
    let src = syz::DirectSource::new(&context).unwrap();
    let gen = syz::BufferGenerator::new(&context)?;
    let buffer = syz::Buffer::from_file(args[2].as_str())?;
    gen.set_buffer(&buffer)?;
    src.add_generator(&gen)?;

    println!("Press enter to exit");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    Ok(())
}
