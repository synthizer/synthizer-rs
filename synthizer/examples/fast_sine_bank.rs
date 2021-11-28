//! Demonstrate sweeping some basic waveforms using `FastSineBankGenerator`.
use std::thread::sleep;
use std::time::Duration;
use synthizer as syz;

fn main() -> syz::Result<()> {
    let _init_guard = syz::initialize()?;
    let context = syz::Context::new()?;

    let cases: Vec<(
        &str,
        fn(&syz::Context, f64) -> syz::Result<syz::FastSineBankGenerator>,
    )> = vec![
        ("sine", |c, f| syz::FastSineBankGenerator::new_sine(c, f)),
        ("square partials=30", |c, f| {
            syz::FastSineBankGenerator::new_square(c, f, 30)
        }),
        ("triangle partials=30", |c, f| {
            syz::FastSineBankGenerator::new_triangle(c, f, 30)
        }),
        ("saw partials=30", |c, f| {
            syz::FastSineBankGenerator::new_saw(c, f, 30)
        }),
    ];

    let source = syz::DirectSource::new(&context)?;

    for (name, func) in cases.into_iter() {
        println!("{}", name);
        let mut freq = 300.0;
        let gen = func(&context, freq)?;
        source.add_generator(&gen)?;
        for _ in 0..20 {
            freq -= 10.0;
            gen.frequency().set(freq)?;
            sleep(Duration::from_millis(20));
        }
        std::mem::drop(gen);
        sleep(Duration::from_millis(50));
    }

    Ok(())
}
