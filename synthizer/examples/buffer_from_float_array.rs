//! Demonstrate loading buffers from raw decoded data by generating binaural
//! beats.
use anyhow::Result;

use synthizer as syz;

const SR: u32 = 10000;
const FREQ_L: f64 = 100.0;
const FREQ_R: f64 = 105.0;

fn main() -> Result<()> {
    let mut data: Vec<f32> = vec![];

    for i in 0..SR {
        let l = (2.0 * i as f64 * FREQ_L * std::f64::consts::PI / SR as f64).sin();
        let r = (2.0 * i as f64 * std::f64::consts::PI * FREQ_R / SR as f64).sin();
        data.push(l as f32);
        data.push(r as f32);
    }

    let _guard = syz::initialize()?;
    let ctx = synthizer::Context::new()?;
    let buffer = syz::Buffer::from_float_array(SR, 2, &data[..])?;
    let gen = syz::BufferGenerator::new(&ctx)?;
    let src = syz::DirectSource::new(&ctx)?;
    gen.buffer().set(&buffer)?;
    gen.looping().set(true)?;
    src.add_generator(&gen)?;

    println!("Press enter to exit");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    Ok(())
}
