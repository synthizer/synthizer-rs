fn main() {
    if let Ok(_) = std::env::var("DOCS_RS") {
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

    // At the moment Rust always links the release version of the MSVC
    // runtime: https://github.com/rust-lang/rust/issues/39016 This may
    let rt = if is_static_crt() {
        "MultiThreaded"
    } else {
        "MultiThreadedDLL"
    };
    cfg.configure_arg(&format!("-DCMAKE_MSVC_RUNTIME_LIBRARY={}", rt));

    // Don't build examples etc.
    cfg.configure_arg("-DSYZ_INTEGRATING=ON");

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

fn is_static_crt() -> bool {
    let features = match std::env::var("CARGO_CFG_TARGET_FEATURE") {
        Ok(f) => f,
        Err(_) => return false,
    };

    for feature in features.split(',') {
        if feature == "crt-static" {
            return true;
        }
    }

    false
}
