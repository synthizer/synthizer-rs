use cmake;

fn main() {
    println!("cargo:rerun-if-changed=synthizer-vendored");

    let is_msvc = std::env::var("TARGET").unwrap().ends_with("msvc");
    let mut cfg = cmake::Config::new("synthizer-vendored");
    if is_msvc {
        cfg.cxxflag("/EHsc");
    }
    let dst = cfg.build();

    println!("cargo:rustc-link-search=all={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=synthizer");
}
