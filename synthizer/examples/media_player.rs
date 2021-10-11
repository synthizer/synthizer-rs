//! Implement the media player example from the Python tutorial, but in Rust.
use std::io::Write;
use std::str::FromStr;

use synthizer as syz;

fn help() {
    print!(
        r#"Commands:
pause, play: pause and/or play the media.
pos x y z: move the source to the position x y z.
seek pos: seek to position pos in seconds.
gain value: control the gain of the generator in db
quit: exit.
"#
    );
}

fn main_impl(input_file: &str) -> syz::Result<()> {
    let mut init_cfg = syz::LibraryConfig::new();
    init_cfg.log_to_stderr();
    init_cfg.log_level(syz::LogLevel::Debug);
    let _guard = init_cfg.initialize()?;

    let ctx = syz::Context::new()?;
    ctx.set_default_panner_strategy(syz::PannerStrategy::Hrtf)?;

    let src = syz::Source3D::new(&ctx, syz::PannerStrategy::Delegate, (0.0, 0.0, 0.0))?;
    let gen = syz::BufferGenerator::new(&ctx)?;
    let syz_buf = syz::Buffer::from_file(input_file)?;
    gen.set_buffer(&syz_buf)?;
    src.add_generator(&gen)?;

    let mut looping = false;
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    loop {
        print!("> ");
        stdout.flush().expect("Should flush");

        let mut line = String::new();
        stdin.read_line(&mut line).expect("Should always read");
        let line = line.trim();

        if line == "quit" {
            break;
        }

        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "help" => {
                help();
            }
            "pause" => {
                src.pause()?;
            }
            "play" => {
                src.play()?;
            }
            "loop" => {
                looping = !looping;
                gen.set_looping(looping)?;
            }
            "seek" => {
                if parts.len() != 2 {
                    println!("Usage: seek <pos>");
                }
                let pos = match f64::from_str(parts[1]) {
                    Ok(f) => f,
                    Err(_) => {
                        println!("Invalid position");
                        continue;
                    }
                };
                gen.set_playback_position(pos)?;
            }
            "pos" => {
                if parts.len() != 4 {
                    println!("Usage: pos x y z");
                    continue;
                }
                let m_x = f64::from_str(parts[1]);
                let m_y = f64::from_str(parts[2]);
                let m_z = f64::from_str(parts[3]);
                let pos = match (m_x, m_y, m_z) {
                    (Ok(x), Ok(y), Ok(z)) => (x, y, z),
                    _ => {
                        println!("Unable to parse position");
                        continue;
                    }
                };
                src.set_position(pos)?;
            }
            "gain" => {
                if parts.len() != 2 {
                    println!("Usage: gain <db>");
                    continue;
                }
                let gain_db = match f64::from_str(parts[1]) {
                    Ok(g) => g,
                    Err(_) => {
                        println!("Unable to parse gain");
                        continue;
                    }
                };
                let gain = 10.0f64.powf(gain_db / 20.0);
                src.set_gain(gain)?;
            }
            _ => {
                println!("Invalid command. Type help for usage");
            }
        }
    }

    Ok(())
}

fn main() -> std::result::Result<(), &'static str> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return Err("Failed");
    }

    main_impl(args[1].as_str()).expect("Should succeed");
    Ok(())
}
