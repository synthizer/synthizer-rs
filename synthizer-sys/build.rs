fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        // Docs don't link.
        return;
    }

    println!("cargo:rerun-if-changed=synthizer-vendored");

    let mut cfg = cmake::Config::new("synthizer-vendored");

    #[cfg(target_env = "msvc")]
    {
        cfg.cxxflag("/EHsc");
        // We need to use Ninja in CI.
        if std::env::var("CI").is_ok() {
            cfg.generator("Ninja");
        }
    }

    let dst = cfg.build();

    println!("cargo:rustc-link-search=all={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=synthizer");

    #[cfg(target_family = "unix")]
    {
        #[cfg(not(target_os = "macos"))]
        println!("cargo:rustc-link-lib=stdc++");
        #[cfg(target_os = "macos")]
        println!("cargo:rustc-link-lib=c++");
    }
}
