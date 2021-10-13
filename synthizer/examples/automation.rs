//! Demonstrate automation by generating binaural beats, applying an envelope,
//! and waiting for the event to signal that it's done.
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
    ctx.enable_events()?;
    let buffer = syz::Buffer::from_float_array(SR, 2, &data[..])?;
    let gen = syz::BufferGenerator::new(&ctx)?;
    let src = syz::DirectSource::new(&ctx)?;
    gen.buffer().set(&buffer)?;
    gen.looping().set(true)?;

    // before adding to the source, automate the generator.
    gen.gain().set(0.0)?;
    let mut ab = syz::AutomationBatch::new(&ctx)?;
    let timebase = gen.suggested_automation_time().get()?;
    ab.set_double(
        &gen,
        gen.gain(),
        timebase + 0.05,
        1.0,
        syz::InterpolationType::Linear,
    )?
    .set_double(
        &gen,
        gen.gain(),
        timebase + 0.01,
        0.7,
        syz::InterpolationType::Linear,
    )?
    .set_double(
        &gen,
        gen.gain(),
        timebase + 0.2,
        0.1,
        syz::InterpolationType::Linear,
    )?
    .set_double(
        &gen,
        gen.gain(),
        timebase + 0.7,
        0.1,
        syz::InterpolationType::None,
    )?
    .set_double(
        &gen,
        gen.gain(),
        timebase + 1.5,
        0.0,
        syz::InterpolationType::Linear,
    )?
    .send_user_event(&gen, 0.6, 5)?;
    ab.execute()?;
    src.add_generator(&gen)?;

    println!("Waiting on automation event...");
    loop {
        for e in ctx.get_events() {
            if let syz::EventType::UserAutomation { param } = e?.r#type {
                println!("Got param {}", param);
                return Ok(());
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
