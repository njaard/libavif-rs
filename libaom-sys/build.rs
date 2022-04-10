use std::env;

fn main() {
    if env::var_os("DOCS_RS").is_some() {
        return;
    }

    let mut aom = cmake::Config::new("vendor");
    aom.profile(if env::var("PROFILE").expect("PROFILE") == "release" {
        "Release"
    } else {
        "Debug"
    })
    .define("ENABLE_DOCS", "0")
    .define("ENABLE_EXAMPLES", "0")
    .define("ENABLE_TESTDATA", "0")
    .define("ENABLE_TESTS", "0")
    .define("ENABLE_TOOLS", "0")
    .define("CMAKE_INSTALL_LIBDIR", "lib");

    let host = env::var("HOST").expect("HOST");
    let target = env::var("TARGET").expect("TARGET");
    if host != target {
        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH");
        aom.define("AOM_TARGET_CPU", target_arch);
    }

    if env::var("LIBAVIF_CROSS_WIN32").is_ok() {
        aom.configure_arg("-T host=x64").configure_arg("-A Win32");
    }

    if target == "armv7-unknown-linux-gnueabihf" {
        aom.define(
            "CMAKE_TOOLCHAIN_FILE",
            "build/cmake/toolchains/armv7-linux-gcc.cmake",
        );
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
