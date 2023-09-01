# libavif-sys

[![crates.io](https://img.shields.io/crates/v/libavif-sys.svg)](https://crates.io/crates/libavif-sys)
[![Documentation](https://docs.rs/libavif-sys/badge.svg)](https://docs.rs/libavif-sys)
[![BSD-2-Clause licensed](https://img.shields.io/crates/l/libavif-sys.svg)](../LICENSE)
[![Rustc Version 1.62+](https://img.shields.io/badge/rustc-1.62+-lightgray.svg)](https://blog.rust-lang.org/2022/06/30/Rust-1.62.0.html)
[![CI](https://github.com/njaard/libavif-rs/workflows/CI/badge.svg)](https://github.com/njaard/libavif-rs/actions?query=workflow%3ACI)

AVIF is an image codec based on the next-generation
open, royalty-free video coding format [AV1](https://en.wikipedia.org/wiki/AV1).

AVIF gets compression ratios [considerably better than JPEG](https://netflixtechblog.com/avif-for-next-generation-image-coding-b1d75675fe4)
for similar quality levels.

This crate wraps the Alliance for Open Media's [libavif](https://github.com/AOMediaCodec/libavif)
into an unsafe rust crate.

Minimal safe wrappers are provided by [libavif](https://crates.io/crates/libavif) and
a `image`-compatible [libavif-image](https://crates.io/crates/libavif-image).

# Codec Features
This crate, by default, uses `codec-dav1d` for decoding and `codec-rav1e` for encoding, because
they have the best (speed) performance. You can disable those features and instead use `codec-aom`.

# Compiling
* cmake is required for compiling the crate.
* `dav1d` requires ninja/meson and nasm
* `rav1e` is implemented in Rust and has no other dependencies (though note that libavif itself
uses it via rav1e's C-API).

# Platform support
This crate is tested on Linux, MacOS, and Windows.

# License
This crate is released under the BSD-2-Clause license. The dependant
C-libraries are released under similarly permissive licenses. Patent 
licenses are also provided.


