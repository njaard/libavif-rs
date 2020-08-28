//! This crate is only for linking with dav1d library. There are no bindings in this crate.

extern "C" {
    pub fn dav1d_version() -> *const std::os::raw::c_char;
}

#[test]
fn poke() {
    unsafe {
        let ver = std::ffi::CStr::from_ptr(dav1d_version()).to_str().unwrap();
        assert!(!ver.is_empty());
    }
}
