AVIF is an image codec based on the next-generation
open, royalty-free video coding format [AV1](https://en.wikipedia.org/wiki/AV1).

AVIF gets compression ratios [considerably better than JPEG](https://netflixtechblog.com/avif-for-next-generation-image-coding-b1d75675fe4)
for similar quality levels.

This crate wraps the Alliance for Open Media's [libavif](https://github.com/AOMediaCodec/libavif)
into an unsafe rust crate.

Minimal safe wrappers are provided by [libavif](https://crates.io/crates/libavif) and
a `image`-compatible [libavif-image](https://crates.io/crates/libavif-image).

# Codecs
This crate, by default, uses `dav1d` for decoding and `rav1e` for encoding, because
they are the best performing. You can also disable decoding or encoding for smaller
libraries or also disable those features and instead use `aom`.

# Compiling
`cmake` is required for compiling the crate.

`dav1d` requires ninja/meson for compiling.

`rav1e` is implemented in Rust and has no other dependencies (though note that libavif itself
uses it through its C-API.

# Platform support
This crate is only expected to compile on Linux. Patches accepted.

