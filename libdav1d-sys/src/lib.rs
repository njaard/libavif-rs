//! These are raw FFI bindings for [libdav1d](https://code.videolan.org/videolan/dav1d), a fast software AV1 decoder.
//! Refer to libdav1d's documentation for details.

// build with --feature=generate, then copy target/.../bindings.rs to ffi.rs

#![allow(bad_style)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(clippy::all)]

type __builtin_va_list = *mut std::ffi::c_void;
type __va_list_tag = *mut std::ffi::c_void;

#[cfg(feature = "generate")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "generate"))]
include!("../ffi.rs");

#[allow(bad_style)]
pub const fn DAV1D_ERR(errno: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    (if libc::EPERM > 0 { -errno } else { errno }) as _
}

pub const DAV1D_ERR_AGAIN: ::std::os::raw::c_int = DAV1D_ERR(libc::EAGAIN);
pub const DAV1D_ERR_INVAL: ::std::os::raw::c_int = DAV1D_ERR(libc::EINVAL);
pub const DAV1D_ERR_NOMEM: ::std::os::raw::c_int = DAV1D_ERR(libc::ENOMEM);
pub const DAV1D_ERR_NOPROTOOPT: ::std::os::raw::c_int = DAV1D_ERR(libc::ENOPROTOOPT);

#[test]
fn poke() {
    unsafe {
        let ver = std::ffi::CStr::from_ptr(dav1d_version()).to_str().unwrap();
        assert!(!ver.is_empty());
    }
}
