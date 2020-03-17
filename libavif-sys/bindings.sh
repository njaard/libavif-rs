#!/bin/bash

bindgen libavif/include/avif/avif.h --no-prepend-enum-name --with-derive-default --rust-target 1.36 --size_t-is-usize -o src/bindings.rs
