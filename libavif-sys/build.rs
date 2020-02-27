use std::env;
use std::fs;
use std::process::Command;

use cmake::Config;

fn main() {
    let mut _build_paths = String::new();
    let mut avif = Config::new("libavif");

    #[cfg(feature = "codec-aom")]
    {
        eprintln!("building aom");
        // let aom = "/home/charles/dev/libavif/target/release/build/libavif-sys-e65a2afd78d3a783/out/build";

        let mut aom = Config::new("aom");
        aom.define("ENABLE_DOCS", "0")
            .define("ENABLE_EXAMPLES", "0")
            .define("ENABLE_TESTDATA", "0")
            .define("ENABLE_TESTS", "0")
            .define("ENABLE_TOOLS", "0");

        #[cfg(feature = "__internal_aom_generic_target")]
        {
            aom.define("AOM_TARGET_CPU", "generic");
        }

        aom.profile("Release").build();

        avif.define("AVIF_CODEC_AOM", "1");
        println!("cargo:rustc-link-lib=static=aom");
    }

    #[cfg(feature = "codec-rav1e")]
    {
        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        fs::create_dir_all(&format!("{}/include/rav1e", env::var("OUT_DIR").unwrap()))
            .expect("mkdir");
        fs::copy(
            &format!("{}/rav1e.h", crate_dir),
            &format!("{}/include/rav1e/rav1e.h", env::var("OUT_DIR").unwrap()),
        )
        .expect("copy rav1e.h");
        avif.define("AVIF_CODEC_RAV1E", "1")
            .define("AVIF_CODEC_LIBRARIES", "rav1e")
            .define("LIBRAV1E_LIBRARY_PATH", "-rav1e");
    }

    #[cfg(feature = "codec-dav1d")]
    {
        let build_path = env::var("OUT_DIR").unwrap() + "/dav1d";
        {
            println!("cargo:rustc-link-lib=static=dav1d");
            println!("cargo:rustc-link-search=native={}", build_path);
            let s = Command::new("meson")
                .arg("--default-library=static")
                .arg("--buildtype")
                .arg("release")
                .arg(&build_path)
                .arg("dav1d")
                .arg("--prefix")
                .arg(&format!("{}/install", build_path))
                .status()
                .expect("meson");
            assert!(s.success());
            let s = Command::new("ninja")
                .current_dir(&build_path)
                .arg("install")
                .status()
                .expect("ninja");
            assert!(s.success());
        }
        _build_paths += &format!("{}/install;", build_path);
        println!("cargo:rustc-link-search=native={}/src", build_path);
        avif.define("AVIF_CODEC_DAV1D", "1");
    }

    eprintln!("building libavif");

    let mut avif_built = avif
        .define("CMAKE_PREFIX_PATH", &_build_paths)
        .define("BUILD_SHARED_LIBS", "0")
        .profile("Release")
        .build();
    avif_built.push("lib");
    println!("cargo:rustc-link-search=native={}", avif_built.display());
    println!("cargo:rustc-link-lib=static=avif");
    println!("cargo:outdir={}", env::var("OUT_DIR").unwrap());
}
