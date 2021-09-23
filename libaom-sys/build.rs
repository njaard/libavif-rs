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

    let dst = aom.build();

    // DEP_AOM_INCLUDE, DEP_AOM_PKGCONFIG variables
    println!("cargo:include={}", dst.join("include").display());
    println!(
        "cargo:pkgconfig={}",
        dst.join("lib").join("pkgconfig").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=aom");
}
