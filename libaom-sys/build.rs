use std::env;

fn main() {
    if env::var_os("DOCS_RS").is_some() {
        return;
    }

    let mut aom = cmake::Config::new("vendor");
    aom.profile("Release")
        .define("ENABLE_DOCS", "0")
        .define("ENABLE_EXAMPLES", "0")
        .define("ENABLE_TESTDATA", "0")
        .define("ENABLE_TESTS", "0")
        .define("ENABLE_TOOLS", "0");

    let host = env::var("HOST").expect("HOST");
    let target = env::var("TARGET").expect("TARGET");
    if host != target {
        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH");
        aom.define("AOM_TARGET_CPU", target_arch);
    }

    let using_msvc = target.ends_with("-msvc");

    if using_msvc {
        // aom's cmake build hides install target for msvc
        aom.build_target("aom");
    }

    let dst = aom.build();

    // without install target these files are all over the place
    if using_msvc {
        let root = std::path::PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("root"));
        // DEP_AOM_INCLUDE variable
        println!("cargo:include={}", root.join("vendor").display());

        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("build").join("Release").display()
        );
    } else {
        println!("cargo:include={}", dst.join("include").display());
        // DEP_AOM_PKGCONFIG variable
        println!(
            "cargo:pkgconfig={}",
            dst.join("lib").join("pkgconfig").display()
        );

        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib").display()
        );
    }
    println!("cargo:rustc-link-lib=static=aom");
}
