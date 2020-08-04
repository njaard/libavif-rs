#![allow(clippy::match_like_matches_macro)]

fn main() {
    let mut cfg = ctest::TestGenerator::new();

    cfg.include("../libavif/include/");
    cfg.header("avif/avif.h");

    cfg.skip_struct(|t| match t {
        "avifEncoderData" | "avifDecoder" => true, // opaque
        _ => false,
    });

    cfg.skip_signededness(|t| match t {
        // enum sign mismatch
        "avifDecoderSource"
        | "avifRange"
        | "avifPixelFormat"
        | "avifChromaSamplePosition"
        | "avifColorPrimaries"
        | "avifTransferCharacteristics"
        | "avifMatrixCoefficients"
        | "avifTransformationFlags"
        | "avifRGBFormat"
        | "avifChromaUpsampling"
        | "avifCodecChoice"
        | "avifResult" => true,
        _ => false,
    });

    cfg.type_name(|t, _is_struct, _is_union| {
        match t {
            "__enum" | "avifPlanesFlags" => "int",
            "avifEncoderData" => "struct avifEncoderData",
            t => t,
        }
        .to_string()
    });

    cfg.generate("../src/lib.rs", "ctest.rs");
}
