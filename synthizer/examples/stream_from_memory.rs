use std::{fs::File, io::Read};

use anyhow::Result;

use synthizer as syz;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return Ok(());
    }

    let _guard = syz::initialize()?;
    let ctx = synthizer::Context::new()?;
    let gen = {
        let mut file = File::open(args[1].as_str())?;
        let mut data = vec![];
        file.read_to_end(&mut data)?;
        let stream = syz::StreamHandle::from_vec(data)?;
        syz::StreamingGenerator::from_stream_handle(&ctx, stream)?
    };
    let src = syz::DirectSource::new(&ctx)?;
    src.add_generator(&gen)?;

    println!("Press enter to exit");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    Ok(())
}
