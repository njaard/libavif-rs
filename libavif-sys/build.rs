use std::env;
#[cfg(feature = "codec-rav1e")]
use std::fs;
use std::path::Path;

use cmake::Config;

fn main() {
    if env::var_os("DOCS_RS").is_some() {
        return;
    }

    let out_dir_ = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_);
    let mut avif = Config::new("libavif");

    #[allow(unused_mut)]
    let mut pc_paths: Vec<_> = env::var("PKG_CONFIG_PATH")
        .map(|v| env::split_paths(&v).collect())
        .unwrap_or_default();

    avif.define("BUILD_SHARED_LIBS", "0");
    // Required for clang 12 on macOS, and likely all future compilers libavif hasn't been tweaked for yet
    avif.define("AVIF_ENABLE_WERROR", "0");

    if env::var_os("CI").is_some() {
        avif.very_verbose(true);
    }

    #[cfg(feature = "codec-aom")]
    {
        let include =
            env::var_os("DEP_AOM_INCLUDE").expect("libaom-sys should have set include path");
        avif.define("AVIF_CODEC_AOM", "1");
        avif.define("AOM_INCLUDE_DIR", include);

        let pc_path =
            env::var_os("DEP_AOM_PKGCONFIG").expect("libaom-sys should have set pkgconfig path");
        avif.define("AOM_LIBRARY", pc_path.clone());
        pc_paths.insert(0, pc_path.into());
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

    #[cfg(feature = "codec-dav1d")]
    {
        let include =
            env::var_os("DEP_DAV1D_INCLUDE").expect("libdav1d-sys should have set pkgconfig path");
        avif.define("AVIF_CODEC_DAV1D", "1");
        avif.define("DAV1D_INCLUDE_DIR", include);

        if let Some(pc_path) = env::var_os("DEP_DAV1D_PKGCONFIG") {
            pc_paths.insert(0, pc_path.into());
        }

        if let Some(staticlib) = env::var_os("DEP_DAV1D_STATICLIB") {
            avif.define("DAV1D_LIBRARY", staticlib);
        }
    }

    eprintln!("building libavif");

    let local_pc_files = env::join_paths(pc_paths).unwrap();

    eprintln!("pc=\"{:?}\"", local_pc_files);
    avif.env("PKG_CONFIG_PATH", local_pc_files);

    avif.profile(if env::var("PROFILE").expect("PROFILE") == "release" {
        "Release"
    } else {
        "Debug"
    })
    .configure_arg("-DCMAKE_INSTALL_LIBDIR=lib")
    .configure_arg("-DCMAKE_DISABLE_FIND_PACKAGE_libyuv=1");
    if env::var("LIBAVIF_CROSS_WIN32").is_ok() {
        avif.configure_arg("-T host=x64").configure_arg("-A Win32");
    }
    let avif_built = avif.build();

    println!(
        "cargo:rustc-link-search=native={}",
        avif_built.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=avif");

    println!("cargo:outdir={}", out_dir.display());
}
