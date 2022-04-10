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
        .status()
        .expect("This crate requires meson (and ninja) to be installed: https://mesonbuild.com/");
    assert!(s.success(), "meson failed");

    eprintln!("Installing dav1d into {:?}", install_dir);
    let s = Command::new("ninja")
        .env("DESTDIR", &install_dir)
        .current_dir(&out_dir)
        .arg("install")
        .status()
        .expect("This crate requires ninja to be installed: https://ninja-build.org/");
    assert!(s.success(), "ninja failed");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

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
    let staticlib_path = if target_os == "windows" {
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
}
