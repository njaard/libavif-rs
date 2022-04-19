use std::env;

fn main() {
    if env::var_os("DOCS_RS").is_some() {
        return;
    }

    if let Ok((libaom_path, libaom_include_path, libaom_pkgconfig_path)) =
        env::var("LIB_AOM_STATIC_LIB_PATH").and_then(|libaom_path| {
            env::var("LIB_AOM_INCLUDE_PATH").and_then(|libaom_include_path| {
                env::var("LIB_AOM_PKG_CONFIG_PATH").map(|libaom_pkgconfig_path| {
                    (libaom_path, libaom_include_path, libaom_pkgconfig_path)
                })
            })
        })
    {
        // DEP_AOM_INCLUDE, DEP_AOM_PKGCONFIG variables
        println!("cargo:include={}", libaom_include_path);
        println!("cargo:pkgconfig={}", libaom_pkgconfig_path);

        println!("cargo:rustc-link-search=native={}", libaom_path);
    } else {
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

        if target.contains("wasm") {
            // https://aomedia.googlesource.com/aom/#emscripten-builds
            aom.define("AOM_TARGET_CPU", "generic")
                .define("CONFIG_ACCOUNTING", "1")
                .define("CONFIG_INSPECTION", "1")
                .define("CONFIG_MULTITHREAD", "0")
                .define("CONFIG_RUNTIME_CPU_DETECT", "0")
                .define("CONFIG_WEBM_IO", "0")
                .define(
                    "CMAKE_TOOLCHAIN_FILE",
                    env::var("EMSCRIPTEN_CMAKE_FILE").expect(
                        "EMSCRIPTEN_CMAKE_FILE must be set if you want to build wasm target, it's the local path to https://github.com/emscripten-core/emscripten/blob/main/cmake/Modules/Platform/Emscripten.cmake",
                    ),
                );
        } else {
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
    }

    println!("cargo:rustc-link-lib=static=aom");
}
