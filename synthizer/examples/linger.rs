//! test linger behavior.
use synthizer as syz;

fn main() -> syz::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Usage: example <file>");
    }

    let _init_guard = syz::initialize()?;
    let context = syz::Context::new()?;
    let buffer = syz::Buffer::from_file(args[1].as_str())?;

    {
        let del_cfg = syz::DeleteBehaviorConfigBuilder::new()
            .linger(true)
            .linger_timeout(5.0)
            .build();
        let src = syz::DirectSource::new(&context)?;
        let generator = syz::BufferGenerator::new(&context)?;
        generator.buffer().set(&buffer)?;
        src.add_generator(&generator)?;
        src.config_delete_behavior(&del_cfg)?;
        generator.config_delete_behavior(&del_cfg)?;
    }

    println!("Press enter to exit");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    Ok(())
}
