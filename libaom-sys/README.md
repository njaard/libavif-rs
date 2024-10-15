# Rust bindings to libaom

By default, this crate uses `cmake` to build `libaom`, and links it statically.

To link dynamically to an existing build instead, set env variables: `LIB_AOM_STATIC_LIB_PATH`, `LIB_AOM_INCLUDE_PATH`, `LIB_AOM_PKG_CONFIG_PATH`.
