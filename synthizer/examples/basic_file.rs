//! Play a file by loading it into a buffer.
//!
//! This example uses `.expect`.  Real code probably shouldn't.

use synthizer as syz;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: example <file>");
        return;
    }

    let _init_guard = syz::initialize().expect("Should initialize");
    let context = syz::Context::new().expect("Should create context");
    let src = syz::DirectSource::new(&context).expect("Should be able to create source");
    let generator =
        syz::BufferGenerator::new(&context).expect("Should be able to create generator");
    let buffer = syz::Buffer::from_file(args[1].as_str()).expect("Should load buffer");
    generator.set_buffer(&buffer).expect("Should set buffer");
    src.add_generator(&generator)
        .expect("Should be able to add generator");

    println!("Press enter to exit");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
