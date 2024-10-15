use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if env::var_os("DOCS_RS").is_some() {
        return;
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let install_dir = out_dir.join("install");
    let mut meson = Command::new("meson");

    meson.env("DESTDIR", &install_dir);

    let target = env::var("TARGET").expect("TARGET");

    if target == "i686-pc-windows-msvc" {
        meson.arg("--cross-file").arg("i686-win-msvc.meson");
    }
    if target == "x86_64-pc-windows-gnu" {
        meson.arg("--cross-file").arg("x86_64-w64-mingw32.meson");
    }
    if target == "aarch64-unknown-linux-gnu" {
        meson
            .arg("--cross-file")
            .arg("aarch64-unknown-linux-gnu.meson");
    }
    if target == "wasm32-unknown-emscripten" {
        meson
            .arg("--cross-file")
            .arg("wasm32-unknown-emscripten.meson");
    }

    let s = meson
        .arg("--default-library=static")
        .arg("--buildtype")
        .arg(if env::var("PROFILE").expect("PROFILE") == "release" {
            "release"
        } else {
            "debug"
        })
        .arg("--prefix=/")
        .arg("--libdir=lib")
        .arg(&out_dir)
        .arg("vendor")
        .status();
    match s {
        Ok(s) => if !s.success() {
            println!("cargo:warning=Meson build failed. See error log for details");
            std::process::exit(1);
        }
        Err(err) => {
            println!("cargo:warning=This crate requires meson (and ninja) to be installed: https://mesonbuild.com/");
            println!("cargo:warning=meson: {err}");
            std::process::exit(2);
        }
    }

    eprintln!("Installing dav1d into {:?}", install_dir);
    let s = Command::new("ninja")
        .env("DESTDIR", &install_dir)
        .current_dir(&out_dir)
        .arg("install")
        .status();
    match s {
        Ok(s) => if !s.success() {
            println!("cargo:warning=ninja build failed. See error log for details");
            std::process::exit(1);
        }
        Err(err) => {
            println!("cargo:warning=This crate requires ninja to be installed: https://ninja-build.org/");
            println!("cargo:warning=ninja: {err}");
            std::process::exit(2);
        }
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    let mut unix_path = install_dir.join("lib").join("libdav1d.a");
    if !unix_path.exists() {
        unix_path = std::fs::read_dir(install_dir.join("lib"))
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| !e.file_name().to_str().map_or(false, |n| n.starts_with('.')))
            .map(|e| e.path().join("libdav1d.a"))
            .find(|p| p.exists())
            .expect("can't find libdav1d.a in install dir");
    }
    let staticlib_path = if target_os == "windows" && target_env == "msvc" {
        let win_path = install_dir.join("lib").join("dav1d.lib");
        if !win_path.exists() {
            std::fs::rename(&unix_path, &win_path).unwrap();
        }
        win_path
    } else {
        assert!(unix_path.exists(), "ninja didn't install the library");
        unix_path
    };
    println!(
        "cargo:rustc-link-search=native={}",
        staticlib_path.parent().unwrap().display()
    );
    println!("cargo:rustc-link-lib=static=dav1d");

    // DEP_DAV1D_PRODUCTS variable to set as cmake include path
    println!("cargo:products={}", install_dir.display());
    // DEP_DAV1D_PKGCONFIG variable to add to pkg-config's search path
    println!(
        "cargo:pkgconfig={}",
        install_dir.join("lib").join("pkgconfig").display()
    );
    // DEP_DAV1D_STATICLIB with actual path to the library
    println!("cargo:staticlib={}", staticlib_path.display());
    // DEP_DAV1D_INCLUDE with header include path
    println!("cargo:include={}", install_dir.join("include").display());

    #[cfg(feature = "generate")]
    generate_bindings();
}

#[cfg(feature = "generate")]
fn generate_bindings() {
    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header("vendor/include/dav1d/dav1d.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .constified_enum_module("Dav1dInloopFilterType")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .layout_tests(false)
        .allowlist_item("^[Dd][aA][vV].*")
        .blocklist_item("^_.*")
        .clang_args(&[
            "-I",
            "vendor/include/dav1d/",
            "-I",
            out_path.join("install/include/dav1d/").to_str().unwrap(),
        ])
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
