use synthizer as syz;

#[test]
fn test_initialize() {
    let _guard = syz::initialize().expect("Synthizer should initialize");
}
