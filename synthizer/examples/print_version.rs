use synthizer as syz;

fn main() {
    let (major, minor, patch) = syz::get_version();
    println!("{}.{}.{}", major, minor, patch);
}
