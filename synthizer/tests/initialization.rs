use synthizer as syz;

#[test]
fn initialize() {
    let _guard = syz::initialize().expect("Synthizer should initialize");
}
