#![allow(bad_style)]
#![allow(unused)]
#![allow(clippy::all)]

use libavif_sys::*;
use libc::*;

include!(concat!(env!("OUT_DIR"), "/ctest.rs"));
