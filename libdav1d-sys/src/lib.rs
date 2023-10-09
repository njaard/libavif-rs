// cd vendor; meson setup build; cd build; ninja
// bindgen --constified-enum-module="Dav1dInloopFilterType" --default-enum-style=rust --opaque-type=va_list --no-layout-tests --allowlist-item='^[Dd][aA][vV].*' --blocklist-item='^_.*' vendor/include/dav1d/dav1d.h -- -I vendor/include/dav1d/ -I vendor/build/include/dav1d/ > src/ffi.rs

#[allow(bad_style)]
#[allow(rustdoc::broken_intra_doc_links)]
#[allow(clippy::all)]
mod ffi;
pub use ffi::*;

#[test]
fn poke() {
    unsafe {
        let ver = std::ffi::CStr::from_ptr(dav1d_version()).to_str().unwrap();
        assert!(!ver.is_empty());
    }
}
