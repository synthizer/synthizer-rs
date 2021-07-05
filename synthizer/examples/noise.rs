//! Demonstrate/test the noise generator by cycling through all the nnoise
//! types.
use synthizer as syz;

fn main() -> syz::Result<()> {
    let _guard = syz::initialize()?;

    let ctx = syz::Context::new()?;
    let src = syz::DirectSource::new(&ctx)?;
    let gen = syz::NoiseGenerator::new(&ctx, 2)?;
    src.add_generator(&gen)?;

    for i in [
        syz::NoiseType::Uniform,
        syz::NoiseType::Vm,
        syz::NoiseType::FilteredBrown,
    ]
    .iter()
    {
        println!("{:?}", i);
        gen.set_noise_type(*i);
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    Ok(())
}
