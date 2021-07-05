//! Demonstrates loading Libsndfile.  This example takes two arguments; the first is a path to Libsndfile, and the second a path to a file to decode.
use synthizer as syz;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        println!("Usage: {} <libsndfile-path> <file-path>", args[0]);
        return;
    }

    let mut init_config = syz::LibraryConfig::new();
    init_config.log_level(syz::LogLevel::Debug);
    init_config.log_to_stderr();
    init_config
        .load_libsndfile(args[1].as_str())
        .expect("Should be able to set Libsndfile path");
    let _init_guard = init_config.initialize().expect("Should initialize");

    let context = syz::Context::new().unwrap();
    let src = syz::DirectSource::new(&context).unwrap();
    let gen = syz::BufferGenerator::new(&context).unwrap();
    let buffer = syz::Buffer::from_file(args[2].as_str()).unwrap();
    gen.set_buffer(&buffer).unwrap();
    src.add_generator(&gen).unwrap();

    println!("Press enter to exit");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
