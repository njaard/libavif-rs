#[cfg(feature = "codec-rav1e")]
pub use rav1e::capi::*;

pub const AVIF_PLANE_COUNT_RGB: usize = 3;
pub const AVIF_PLANE_COUNT_YUV: usize = 3;

#[allow(non_camel_case_types)]
pub type avifBool = libc::c_int;

#[allow(non_camel_case_types)]
pub type __enum = libc::c_int;

#[allow(non_camel_case_types)]
pub type avifProfileFormat = __enum;

pub const AVIF_PROFILE_FORMAT_NONE: avifProfileFormat = 0;
pub const AVIF_PROFILE_FORMAT_ICC: avifProfileFormat = 1;
pub const AVIF_PROFILE_FORMAT_NCLX: avifProfileFormat = 2;

#[allow(non_camel_case_types)]
pub type avifPlanesFlags = __enum;
pub const AVIF_PLANES_RGB: avifPlanesFlags = 1;
pub const AVIF_PLANES_YUV: avifPlanesFlags = 1 << 1;
pub const AVIF_PLANES_A: avifPlanesFlags = 1 << 2;
pub const AVIF_PLANES_ALL: avifPlanesFlags = 0xff;

#[allow(non_camel_case_types)]
pub type avifDecoderSource = __enum;
/// If a moov box is present in the .avif(s), use the tracks in it, otherwise decode the primary item.
pub const AVIF_DECODER_SOURCE_AUTO: avifDecoderSource = 0;

/// Use the primary item and the aux (alpha) item in the avif(s).
/// This is where single-image avifs store their image.
pub const AVIF_DECODER_SOURCE_PRIMARY_ITEM: avifDecoderSource = 1;

/// Use the chunks inside primary/aux tracks in the moov block.
/// This is where avifs image sequences store their images.
pub const AVIF_DECODER_SOURCE_TRACKS: avifDecoderSource = 2;

// Decode the thumbnail item. Currently unimplemented.
// pub const AVIF_DECODER_SOURCE_THUMBNAIL_ITEM

#[allow(non_camel_case_types)]
pub type avifRange = __enum;

pub const AVIF_RANGE_LIMITED: avifRange = 0;
pub const AVIF_RANGE_FULL: avifRange = 1;

#[allow(non_camel_case_types)]
pub type avifPixelFormat = __enum;

pub const AVIF_PIXEL_FORMAT_NONE: avifPixelFormat = 0;
pub const AVIF_PIXEL_FORMAT_YUV444: avifPixelFormat = 1;
pub const AVIF_PIXEL_FORMAT_YUV422: avifPixelFormat = 2;
pub const AVIF_PIXEL_FORMAT_YUV420: avifPixelFormat = 3;
pub const AVIF_PIXEL_FORMAT_YV12: avifPixelFormat = 4;

#[allow(non_camel_case_types)]
pub type avifCodecChoice = __enum;
pub const AVIF_CODEC_CHOICE_AUTO: avifCodecChoice = 0;
pub const AVIF_CODEC_CHOICE_AOM: avifCodecChoice = 1;
pub const AVIF_CODEC_CHOICE_DAV1D: avifCodecChoice = 2;
pub const AVIF_CODEC_CHOICE_RAV1E: avifCodecChoice = 3;

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct avifIOStats {
    colorOBUSize: libc::size_t,
    alphaOBUSize: libc::size_t,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct avifNclxColorProfile {
    pub colourPrimaries: u16,
    pub transferCharacteristics: u16,
    pub matrixCoefficients: u16,
    pub fullRangeFlag: u8,
}

pub const AVIF_QUANTIZER_LOSSLESS: libc::c_int = 0;
pub const AVIF_QUANTIZER_BEST_QUALITY: libc::c_int = 0;
pub const AVIF_QUANTIZER_WORST_QUALITY: libc::c_int = 63;

pub const AVIF_SPEED_DEFAULT: libc::c_int = -1;
pub const AVIF_SPEED_SLOWEST: libc::c_int = 0;
pub const AVIF_SPEED_FASTEST: libc::c_int = 10;

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct avifImage {
    pub width: u32,
    pub height: u32,
    /// all planes (RGB/YUV/A) must share this depth; if depth>8, all planes are uint16_t internally
    pub depth: u32,

    pub rgbPlanes: [*mut u8; AVIF_PLANE_COUNT_RGB],
    pub rgbRowBytes: [u32; AVIF_PLANE_COUNT_RGB],

    pub yuvFormat: avifPixelFormat,
    pub yuvRange: avifRange,
    pub yuvPlanes: [*mut u8; AVIF_PLANE_COUNT_YUV],
    pub yuvRowBytes: [u32; AVIF_PLANE_COUNT_YUV],

    pub decoderOwnsYUVPlanes: avifBool,

    pub alphaPlane: *mut u8,
    pub alphaRowBytes: u32,
    pub decoderOwnsAlphaPlane: avifBool,

    pub profileFormat: avifProfileFormat,
    pub icc: avifRWData,
    pub nclx: avifNclxColorProfile,

    // Metadata - set with avifImageSetMetadata*() before write, check .size>0 for existence after read
    pub exif: avifRWData,
    pub xmp: avifRWData,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct avifDecoder {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct avifEncoder {
    pub codecChoice: avifCodecChoice,

    /// multithreading is disabled if <2)
    pub maxThreads: libc::c_int,
    /// quality
    pub minQuantizer: libc::c_int,
    /// quality
    pub maxQuantizer: libc::c_int,

    /// range 0-6. Turn off tiling with 0
    pub tileRowsLog2: libc::c_int,
    /// range 0-6. Turn off tiling with 0
    pub tileColsLog2: libc::c_int,

    /// 0-10: 10 should produce a better quality image
    pub speed: libc::c_int,

    /// stats from the most recent write
    pub ioStats: avifIOStats,
}

#[allow(non_camel_case_types)]
pub type avifResult = __enum;

pub const AVIF_RESULT_OK: avifResult = 0;
pub const AVIF_RESULT_UNKNOWN_ERROR: avifResult = 1;
pub const AVIF_RESULT_INVALID_FTYP: avifResult = 2;
pub const AVIF_RESULT_NO_CONTENT: avifResult = 3;
pub const AVIF_RESULT_NO_YUV_FORMAT_SELECTED: avifResult = 4;
pub const AVIF_RESULT_REFORMAT_FAILED: avifResult = 5;
pub const AVIF_RESULT_UNSUPPORTED_DEPTH: avifResult = 6;
pub const AVIF_RESULT_ENCODE_COLOR_FAILED: avifResult = 7;
pub const AVIF_RESULT_ENCODE_ALPHA_FAILED: avifResult = 8;
pub const AVIF_RESULT_BMFF_PARSE_FAILED: avifResult = 9;
pub const AVIF_RESULT_NO_AV1_ITEMS_FOUND: avifResult = 10;
pub const AVIF_RESULT_DECODE_COLOR_FAILED: avifResult = 11;
pub const AVIF_RESULT_DECODE_ALPHA_FAILED: avifResult = 12;
pub const AVIF_RESULT_COLOR_ALPHA_SIZE_MISMATCH: avifResult = 13;
pub const AVIF_RESULT_ISPE_SIZE_MISMATCH: avifResult = 14;
pub const AVIF_RESULT_NO_CODEC_AVAILABLE: avifResult = 15;
pub const AVIF_RESULT_NO_IMAGES_REMAINING: avifResult = 16;
pub const AVIF_RESULT_INVALID_EXIF_PAYLOAD: avifResult = 17;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct avifROData {
    pub data: *const u8,
    pub size: libc::size_t,
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct avifRWData {
    pub data: *mut u8,
    pub size: libc::size_t,
}

impl Default for avifRWData {
    fn default() -> Self {
        Self {
            data: std::ptr::null_mut(),
            size: 0,
        }
    }
}

#[link(name = "avif", kind = "static")]
extern "C" {
    pub fn avifImageCreateEmpty() -> *mut avifImage;
    pub fn avifImageCreate(
        width: libc::c_int,
        height: libc::c_int,
        depth: libc::c_int,
        yuvFormat: avifPixelFormat,
    ) -> *mut avifImage;
    pub fn avifImageDestroy(image: *mut avifImage) -> avifResult;

    pub fn avifEncoderCreate() -> *mut avifEncoder;
    pub fn avifEncoderWrite(
        encoder: *mut avifEncoder,
        image: *mut avifImage,
        output: *mut avifRWData,
    ) -> avifResult;
    pub fn avifEncoderDestroy(encoder: *mut avifEncoder);

    pub fn avifDecoderCreate() -> *mut avifDecoder;
    pub fn avifDecoderDestroy(decoder: *mut avifDecoder) -> avifResult;
    pub fn avifDecoderRead(
        decoder: *mut avifDecoder,
        image: *mut avifImage,
        data: *mut avifROData,
    ) -> avifResult;
    pub fn avifDecoderSetSource(decoder: *mut avifDecoder, source: avifDecoderSource)
        -> avifResult;
    pub fn avifDecoderParse(decoder: *mut avifDecoder, input: *mut avifROData) -> avifResult;
    pub fn avifDecoderNextImage(decoder: *mut avifDecoder) -> avifResult;
    pub fn avifDecoderNthImage(decoder: *mut avifDecoder, frameIndex: u32) -> avifResult;
    pub fn avifDecoderReset(decoder: *mut avifDecoder) -> avifResult;

    pub fn avifRWDataFree(raw: *mut avifRWData);

    pub fn avifImageYUVToRGB(image: *mut avifImage) -> avifResult;
    pub fn avifImageRGBToYUV(image: *mut avifImage) -> avifResult;

    pub fn avifImageAllocatePlanes(image: *mut avifImage, planes: u32); // Ignores any pre-existing planes
    pub fn avifImageFreePlanes(image: *mut avifImage, planes: u32); // Ignores any pre-existing planes

    pub fn avifImageSetProfileNCLX(image: *mut avifImage, nclx: *mut avifNclxColorProfile);

    pub fn avifVersion() -> *const libc::c_char;
    pub fn avifCodecVersions(outBuffer: *mut libc::c_char);

    /// Returns AVIF_TRUE if input begins with a valid FileTypeBox (ftyp) that supports
    /// either the brand 'avif' or 'avis' (or both), without performing any allocations.
    pub fn avifPeekCompatibleFileType(input: *const avifROData) -> avifBool;
}

