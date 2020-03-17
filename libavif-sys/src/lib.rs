#![allow(
    non_upper_case_globals,
    non_snake_case,
    non_camel_case_types,
    clippy::unreadable_literal
)]

#[cfg(feature = "codec-rav1e")]
pub use rav1e::capi::*;

pub use bindings::*;
mod bindings;
