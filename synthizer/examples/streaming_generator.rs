//! Play a file via a `StreamingGenerator`.

use synthizer as syz;

fn main() -> syz::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Usage: example <file>");
    }

    let _init_guard = syz::initialize()?;
    let context = syz::Context::new()?;
    let src = syz::DirectSource::new(&context)?;
    let generator = syz::StreamingGenerator::from_file(&context, args[1].as_str())?;
    src.add_generator(&generator)?;

    println!("Press enter to exit");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    Ok(())
}
