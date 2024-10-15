//! These are automatically generated C bindings for libaom

#[allow(bad_style)]
mod ffi;
pub use ffi::*;

#[test]
fn test_aom_binding_abi_ver() {
    let cfg = aom_codec_dec_cfg {
        w: 0,
        h: 0,
        threads: 1,
        allow_lowbitdepth: 1,
    };
    let res = unsafe {
        let mut ctx = std::mem::MaybeUninit::uninit();
        aom_codec_dec_init_ver(
            ctx.as_mut_ptr(),
            aom_codec_av1_dx(),
            &cfg,
            0,
            AOM_DECODER_ABI_VERSION as i32,
        )
    };
    assert_eq!(0, res);
}
