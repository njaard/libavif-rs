# libavif

[![crates.io](https://img.shields.io/crates/v/libavif.svg)](https://crates.io/crates/libavif)
[![Documentation](https://docs.rs/libavif/badge.svg)](https://docs.rs/libavif)
[![BSD-2-Clause licensed](https://img.shields.io/crates/l/libavif.svg)](LICENSE)
[![Rustc Version 1.56+](https://img.shields.io/badge/rustc-1.56+-lightgray.svg)](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html)
[![CI](https://github.com/njaard/libavif-rs/workflows/CI/badge.svg)](https://github.com/njaard/libavif-rs/actions?query=workflow%3ACI)

Initial release of a high-level avif decoder.

This crate is generally too minimal for production use;
consider using [`libavif-image`](https://crates.io/crates/libavif-image)
which provides utility functions for [`image`](https://crates.io/crates/image)
(or the `avif` feature in `image`, which doesn't use this crate).

You can also use the unsafe API in [`libavif-sys`](https://crates.io/crates/libavif-sys).

