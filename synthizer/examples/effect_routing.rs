//! Demonstrates effect routing.
use std::thread::sleep;
use std::time::Duration;

use synthizer as syz;

fn main() -> syz::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return Ok(());
    }

    let _guard = syz::initialize();

    let ctx = syz::Context::new()?;
    let gen = syz::BufferGenerator::new(&ctx)?;
    let buffer = syz::Buffer::from_file(args[1].as_str())?;
    let src = syz::DirectSource::new(&ctx)?;
    gen.buffer().set(&buffer)?;
    gen.looping().set(true)?;
    src.add_generator(&gen)?;

    // Make a reverb.
    let reverb = syz::GlobalFdnReverb::new(&ctx)?;
    reverb.t60().set(10.0)?;

    // Set up initial connection without configuration.
    ctx.config_route_simple(&src, &reverb)?;
    sleep(Duration::from_secs(5));

    // Let's add a biquad filter.
    ctx.config_route(
        &src,
        &reverb,
        &syz::RouteConfigBuilder::new()
            .set_filter(syz::BiquadConfig::design_lowpass(500.0, syz::DEFAULT_Q)?)
            .build(),
    )?;
    sleep(Duration::from_secs(5));

    // Now remove it.
    ctx.remove_route(&src, &reverb, 0.1)?;
    sleep(Duration::from_secs(5));

    Ok(())
}
