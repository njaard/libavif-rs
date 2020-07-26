use std::env;
#[cfg(feature = "codec-rav1e")]
use std::fs;
use std::path::Path;
#[cfg(feature = "codec-dav1d")]
use std::process::Command;

use std::ffi::OsString;
use cmake::Config;

fn main() {
    let out_dir_ = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_);
    let mut _build_paths = vec![];
    let mut avif = Config::new("libavif");

    #[cfg(feature = "codec-aom")]
    {
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
        let crate_dir_ = env::var("CARGO_MANIFEST_DIR").unwrap();
        let crate_dir = Path::new(&crate_dir_);
        fs::create_dir_all(out_dir.join("include").join("rav1e")).expect("mkdir");
        fs::copy(
            crate_dir.join("rav1e.h"),
            out_dir.join("include").join("rav1e").join("rav1e.h"),
        )
        .expect("copy rav1e.h");

        avif.define("AVIF_CODEC_RAV1E", "1")
            .define("AVIF_CODEC_LIBRARIES", "rav1e")
            .define("RAV1E_LIBRARY", "-rav1e");
    }

    let mut pc_paths: Vec<_> = env::var("PKG_CONFIG_PATH")
        .ok()
        .iter()
        .flat_map(|v| env::split_paths(v))
        .collect();
    pc_paths.reverse();

    pc_paths.push(out_dir.join("lib").join("pkgconfig"));

    #[cfg(feature = "codec-dav1d")]
    let _dav1d_libbpath = {
        let david_build_path = out_dir.join("dav1d");
        {
            let s = Command::new("meson")
                .arg("--default-library=static")
                .arg("--buildtype")
                .arg("release")
                .arg(&david_build_path)
                .arg("dav1d")
                .arg("--prefix")
                .arg(david_build_path.join("install"))
                .status()
                .expect("meson");
            assert!(s.success());

            let s = Command::new("ninja")
                .current_dir(&david_build_path)
                .arg("install")
                .status()
                .expect("ninja");
            assert!(s.success());
            #[cfg(all(target_os = "windows", feature = "codec-dav1d"))]
            std::fs::rename(
                david_build_path.join("src").join("libdav1d.a"),
                david_build_path.join("src").join("dav1d.lib")
            ).unwrap();
            eprintln!("david_build_path is {:?}", david_build_path)
        }
        _build_paths.push(david_build_path.join("install"));
        println!("cargo:rustc-link-lib=static=dav1d");
        // even though we've installed dav1d's static lib to install/lib, we get it from src/
        // because it will install to `install/lib/x86_64-linux-gnu/libdav1d.a`
        // (on linux). We do the same on windows just for consistency.
        println!("cargo:rustc-link-search=native={}", david_build_path.join("src").display());
        avif.define("AVIF_CODEC_DAV1D", "1");
        pc_paths.push(david_build_path.join("install").join("lib").join("pkgconfig"));
        david_build_path.join("install").join("lib").join("libdav1d.a")
    };

    eprintln!("building libavif");

    let local_pc_files = env::join_paths(pc_paths.iter().rev()).unwrap();
    let mut build_paths: OsString = OsString::new();
    for s in _build_paths {
        build_paths.push(s.as_os_str().to_owned());
        // build_paths.push(";");
    }

    eprintln!("pc=\"{:?}\"; bp=\"{:?}\"", local_pc_files, build_paths);

    let avif_built = avif
        .define("CMAKE_PREFIX_PATH", build_paths)
        .define("BUILD_SHARED_LIBS", "0");

    #[cfg(all(target_os = "windows", feature = "codec-dav1d"))]
    avif_built.define("DAV1D_LIBRARY", _dav1d_libbpath);

    let mut avif_built = avif_built
        .env("PKG_CONFIG_PATH", local_pc_files)
        .profile("Release")
        .build();

    avif_built.push("lib");
    println!("cargo:rustc-link-search=native={}", avif_built.display());
    println!("cargo:rustc-link-lib=static=avif");
    println!("cargo:outdir={}", out_dir.display());
}
