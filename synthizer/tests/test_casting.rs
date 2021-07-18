//! Basic smoke test for casting. We don't check all the tables, just enough to
//! know that it works.
use std::convert::TryInto;

use synthizer as syz;

#[test]
fn test_casting() -> syz::Result<()> {
    let _guard = syz::initialize()?;

    let ctx = syz::Context::new()?;
    let buffer_generator = syz::BufferGenerator::new(&ctx)?;

    assert!(matches!(
        buffer_generator.cast_to::<syz::Generator>()?,
        Some(_)
    ));
    assert!(matches!(
        buffer_generator.cast_to::<syz::Handle>()?,
        Some(_)
    ));
    let _: syz::Generator = (&buffer_generator).try_into().expect("TryFrom should work");
    let h: syz::Handle = (&buffer_generator).try_into().expect("TryFrom should work");

    let _: syz::BufferGenerator = (&h).try_into().expect("TryFrom should work");
    let _: syz::Generator = (&h).try_into().expect("TryFrom should work");

    Ok(())
}
