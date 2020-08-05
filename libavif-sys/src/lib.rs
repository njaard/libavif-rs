#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "codec-aom")]
extern crate libaom_sys; // mark it as used

#[cfg(feature = "codec-rav1e")]
pub use rav1e::capi::*;

pub const AVIF_PLANE_COUNT_YUV: usize = 3;

pub type avifBool = libc::c_int;

pub type __enum = libc::c_int;

pub type avifPlanesFlags = __enum;
pub const AVIF_PLANES_YUV: avifPlanesFlags = 1;
pub const AVIF_PLANES_A: avifPlanesFlags = 1 << 1;
pub const AVIF_PLANES_ALL: avifPlanesFlags = 0xff;

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

pub type avifRange = __enum;

pub const AVIF_RANGE_LIMITED: avifRange = 0;
pub const AVIF_RANGE_FULL: avifRange = 1;

pub type avifPixelFormat = __enum;

pub const AVIF_PIXEL_FORMAT_NONE: avifPixelFormat = 0;
pub const AVIF_PIXEL_FORMAT_YUV444: avifPixelFormat = 1;
pub const AVIF_PIXEL_FORMAT_YUV422: avifPixelFormat = 2;
pub const AVIF_PIXEL_FORMAT_YUV420: avifPixelFormat = 3;
pub const AVIF_PIXEL_FORMAT_YUV400: avifPixelFormat = 4;

pub type avifChromaSamplePosition = __enum;

pub const AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN: avifChromaSamplePosition = 0;
pub const AVIF_CHROMA_SAMPLE_POSITION_VERTICAL: avifChromaSamplePosition = 1;
pub const AVIF_CHROMA_SAMPLE_POSITION_COLOCATED: avifChromaSamplePosition = 2;

pub type avifColorPrimaries = __enum;

pub const AVIF_COLOR_PRIMARIES_UNKNOWN: avifColorPrimaries = 0;
pub const AVIF_COLOR_PRIMARIES_BT709: avifColorPrimaries = 1;
pub const AVIF_COLOR_PRIMARIES_IEC61966_2_4: avifColorPrimaries = 1;
pub const AVIF_COLOR_PRIMARIES_UNSPECIFIED: avifColorPrimaries = 2;
pub const AVIF_COLOR_PRIMARIES_BT470M: avifColorPrimaries = 4;
pub const AVIF_COLOR_PRIMARIES_BT470BG: avifColorPrimaries = 5;
pub const AVIF_COLOR_PRIMARIES_BT601: avifColorPrimaries = 6;
pub const AVIF_COLOR_PRIMARIES_SMPTE240: avifColorPrimaries = 7;
pub const AVIF_COLOR_PRIMARIES_GENERIC_FILM: avifColorPrimaries = 8;
pub const AVIF_COLOR_PRIMARIES_BT2020: avifColorPrimaries = 9;
pub const AVIF_COLOR_PRIMARIES_XYZ: avifColorPrimaries = 10;
pub const AVIF_COLOR_PRIMARIES_SMPTE431: avifColorPrimaries = 11;
pub const AVIF_COLOR_PRIMARIES_SMPTE432: avifColorPrimaries = 12;
pub const AVIF_COLOR_PRIMARIES_EBU3213: avifColorPrimaries = 22;

pub type avifTransferCharacteristics = __enum;

pub const AVIF_TRANSFER_CHARACTERISTICS_UNKNOWN: avifTransferCharacteristics = 0;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT709: avifTransferCharacteristics = 1;
pub const AVIF_TRANSFER_CHARACTERISTICS_UNSPECIFIED: avifTransferCharacteristics = 2;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT470M: avifTransferCharacteristics = 4;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT470BG: avifTransferCharacteristics = 5;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT601: avifTransferCharacteristics = 6;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE240: avifTransferCharacteristics = 7;
pub const AVIF_TRANSFER_CHARACTERISTICS_LINEAR: avifTransferCharacteristics = 8;
pub const AVIF_TRANSFER_CHARACTERISTICS_LOG100: avifTransferCharacteristics = 9;
pub const AVIF_TRANSFER_CHARACTERISTICS_LOG100_SQRT10: avifTransferCharacteristics = 10;
pub const AVIF_TRANSFER_CHARACTERISTICS_IEC61966: avifTransferCharacteristics = 11;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT1361: avifTransferCharacteristics = 12;
pub const AVIF_TRANSFER_CHARACTERISTICS_SRGB: avifTransferCharacteristics = 13;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT2020_10BIT: avifTransferCharacteristics = 14;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT2020_12BIT: avifTransferCharacteristics = 15;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE2084: avifTransferCharacteristics = 16;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE428: avifTransferCharacteristics = 17;
pub const AVIF_TRANSFER_CHARACTERISTICS_HLG: avifTransferCharacteristics = 18;

pub type avifMatrixCoefficients = __enum;

pub const AVIF_MATRIX_COEFFICIENTS_IDENTITY: avifMatrixCoefficients = 0;
pub const AVIF_MATRIX_COEFFICIENTS_BT709: avifMatrixCoefficients = 1;
pub const AVIF_MATRIX_COEFFICIENTS_UNSPECIFIED: avifMatrixCoefficients = 2;
pub const AVIF_MATRIX_COEFFICIENTS_FCC: avifMatrixCoefficients = 4;
pub const AVIF_MATRIX_COEFFICIENTS_BT470BG: avifMatrixCoefficients = 5;
pub const AVIF_MATRIX_COEFFICIENTS_BT601: avifMatrixCoefficients = 6;
pub const AVIF_MATRIX_COEFFICIENTS_SMPTE240: avifMatrixCoefficients = 7;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO: avifMatrixCoefficients = 8;
pub const AVIF_MATRIX_COEFFICIENTS_BT2020_NCL: avifMatrixCoefficients = 9;
pub const AVIF_MATRIX_COEFFICIENTS_BT2020_CL: avifMatrixCoefficients = 10;
pub const AVIF_MATRIX_COEFFICIENTS_SMPTE2085: avifMatrixCoefficients = 11;
pub const AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_NCL: avifMatrixCoefficients = 12;
pub const AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_CL: avifMatrixCoefficients = 13;
pub const AVIF_MATRIX_COEFFICIENTS_ICTCP: avifMatrixCoefficients = 14;

pub type avifTransformationFlags = __enum;
pub const AVIF_TRANSFORM_NONE: avifTransformationFlags = 0;
pub const AVIF_TRANSFORM_PASP: avifTransformationFlags = 1 << 0;
pub const AVIF_TRANSFORM_CLAP: avifTransformationFlags = 1 << 1;
pub const AVIF_TRANSFORM_IROT: avifTransformationFlags = 1 << 2;
pub const AVIF_TRANSFORM_IMIR: avifTransformationFlags = 1 << 3;

/// 'pasp' from ISO/IEC 14496-12:2015 12.1.4.3
///
/// define the relative width and height of a pixel
#[repr(C)]
#[derive(Debug)]
pub struct avifPixelAspectRatioBox {
    pub hSpacing: u32,
    pub vSpacing: u32,
}

/// 'clap' from ISO/IEC 14496-12:2015 12.1.4.3
#[repr(C)]
#[derive(Debug)]
pub struct avifCleanApertureBox {
    /// a fractional number which defines the exact clean aperture width, in counted pixels, of the video image
    widthN: u32,
    widthD: u32,

    /// a fractional number which defines the exact clean aperture height, in counted pixels, of the video image
    heightN: u32,
    heightD: u32,

    /// a fractional number which defines the horizontal offset of clean aperture centre minus (width‐1)/2. Typically 0.
    horizOffN: u32,
    horizOffD: u32,

    /// a fractional number which defines the vertical offset of clean aperture centre minus (height‐1)/2. Typically 0.
    vertOffN: u32,
    vertOffD: u32,
}

/// 'irot' from ISO/IEC 23008-12:2017 6.5.10
#[repr(C)]
#[derive(Debug)]
pub struct avifImageRotation {
    /// angle * 90 specifies the angle (in anti-clockwise direction) in units of degrees.
    /// legal values: [0-3]
    angle: u8,
}

/// 'imir' from ISO/IEC 23008-12:2017 6.5.12
#[repr(C)]
#[derive(Debug)]
pub struct avifImageMirror {
    /// axis specifies a vertical (axis = 0) or horizontal (axis = 1) axis for the mirroring operation.
    /// legal values: [0, 1]
    axis: u8,
}

pub type avifRGBFormat = __enum;

pub const AVIF_RGB_FORMAT_RGB: avifRGBFormat = 0;
pub const AVIF_RGB_FORMAT_RGBA: avifRGBFormat = 1;
pub const AVIF_RGB_FORMAT_ARGB: avifRGBFormat = 2;
pub const AVIF_RGB_FORMAT_BGR: avifRGBFormat = 3;
pub const AVIF_RGB_FORMAT_BGRA: avifRGBFormat = 4;
pub const AVIF_RGB_FORMAT_ABGR: avifRGBFormat = 5;

pub type avifChromaUpsampling = __enum;

pub const AVIF_CHROMA_UPSAMPLING_BILINEAR: avifChromaUpsampling = 0;
pub const AVIF_CHROMA_UPSAMPLING_NEAREST: avifChromaUpsampling = 1;

pub type avifCodecChoice = __enum;
pub const AVIF_CODEC_CHOICE_AUTO: avifCodecChoice = 0;
pub const AVIF_CODEC_CHOICE_AOM: avifCodecChoice = 1;
pub const AVIF_CODEC_CHOICE_DAV1D: avifCodecChoice = 2;
pub const AVIF_CODEC_CHOICE_RAV1E: avifCodecChoice = 4;

#[repr(C)]
#[derive(Debug)]
pub struct avifIOStats {
    colorOBUSize: libc::size_t,
    alphaOBUSize: libc::size_t,
}

pub const AVIF_QUANTIZER_LOSSLESS: libc::c_int = 0;
pub const AVIF_QUANTIZER_BEST_QUALITY: libc::c_int = 0;
pub const AVIF_QUANTIZER_WORST_QUALITY: libc::c_int = 63;

pub const AVIF_SPEED_DEFAULT: libc::c_int = -1;
pub const AVIF_SPEED_SLOWEST: libc::c_int = 0;
pub const AVIF_SPEED_FASTEST: libc::c_int = 10;

#[repr(C)]
#[derive(Debug)]
pub struct avifImage {
    pub width: u32,
    pub height: u32,
    /// all planes must share this depth; if depth>8, all planes are uint16_t internally
    pub depth: u32,

    pub yuvFormat: avifPixelFormat,
    pub yuvRange: avifRange,
    pub yuvChromaSamplePosition: avifChromaSamplePosition,
    pub yuvPlanes: [*mut u8; AVIF_PLANE_COUNT_YUV],
    pub yuvRowBytes: [u32; AVIF_PLANE_COUNT_YUV],
    pub imageOwnsYUVPlanes: avifBool,

    pub alphaRange: avifRange,
    pub alphaPlane: *mut u8,
    pub alphaRowBytes: u32,
    pub imageOwnsAlphaPlane: avifBool,

    pub icc: avifRWData,

    pub colorPrimaries: avifColorPrimaries,
    pub transferCharacteristics: avifTransferCharacteristics,
    pub matrixCoefficients: avifMatrixCoefficients,

    pub transformFlags: u32,
    pub pasp: avifPixelAspectRatioBox,
    pub clap: avifCleanApertureBox,
    pub irot: avifImageRotation,
    pub imir: avifImageMirror,

    // Metadata - set with avifImageSetMetadata*() before write, check .size>0 for existence after read
    pub exif: avifRWData,
    pub xmp: avifRWData,
}

#[repr(C)]
#[derive(Debug)]
pub struct avifRGBImage {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub format: avifRGBFormat,
    pub chromaUpsampling: avifChromaUpsampling,
    pub ignoreAlpha: avifBool,
    pub pixels: *mut u8,
    pub rowBytes: u32,
}

impl Default for avifRGBImage {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
pub struct avifEncoderData {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avifDecoder {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub struct avifEncoder {
    pub codecChoice: avifCodecChoice,

    /// multithreading is disabled if <2)
    pub maxThreads: libc::c_int,
    /// quality
    pub minQuantizer: libc::c_int,
    /// quality
    pub maxQuantizer: libc::c_int,
    // quality
    pub minQuantizerAlpha: libc::c_int,
    // quality
    pub maxQuantizerAlpha: libc::c_int,

    /// range 0-6. Turn off tiling with 0
    pub tileRowsLog2: libc::c_int,
    /// range 0-6. Turn off tiling with 0
    pub tileColsLog2: libc::c_int,

    /// 0-10: 10 should produce a better quality image
    pub speed: libc::c_int,

    /// How many frames between automatic forced keyframes; 0 to disable (default).
    pub keyframeInterval: libc::c_int,
    /// timescale of the media (Hz)
    pub timescale: u64,

    /// stats from the most recent write
    pub ioStats: avifIOStats,

    // Internals used by the encoder
    pub data: *mut avifEncoderData,
}

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
#[derive(Debug)]
pub struct avifROData {
    pub data: *const u8,
    pub size: libc::size_t,
}

#[repr(C)]
#[derive(Debug)]
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
    pub fn avifImageDestroy(image: *mut avifImage);

    pub fn avifEncoderCreate() -> *mut avifEncoder;
    pub fn avifEncoderWrite(
        encoder: *mut avifEncoder,
        image: *const avifImage,
        output: *mut avifRWData,
    ) -> avifResult;
    pub fn avifEncoderDestroy(encoder: *mut avifEncoder);

    pub fn avifDecoderCreate() -> *mut avifDecoder;
    pub fn avifDecoderDestroy(decoder: *mut avifDecoder);
    pub fn avifDecoderRead(
        decoder: *mut avifDecoder,
        image: *mut avifImage,
        data: *const avifROData,
    ) -> avifResult;
    pub fn avifDecoderSetSource(decoder: *mut avifDecoder, source: avifDecoderSource)
        -> avifResult;
    pub fn avifDecoderParse(decoder: *mut avifDecoder, input: *const avifROData) -> avifResult;
    pub fn avifDecoderNextImage(decoder: *mut avifDecoder) -> avifResult;
    pub fn avifDecoderNthImage(decoder: *mut avifDecoder, frameIndex: u32) -> avifResult;
    pub fn avifDecoderReset(decoder: *mut avifDecoder) -> avifResult;

    pub fn avifRWDataFree(raw: *mut avifRWData);

    pub fn avifRGBImageSetDefaults(rgb: *mut avifRGBImage, image: *const avifImage);
    pub fn avifRGBImageAllocatePixels(rgb: *mut avifRGBImage);
    pub fn avifRGBImageFreePixels(rgb: *mut avifRGBImage);

    pub fn avifImageYUVToRGB(image: *const avifImage, rgb: *mut avifRGBImage) -> avifResult;
    pub fn avifImageRGBToYUV(image: *mut avifImage, rgb: *const avifRGBImage) -> avifResult;

    pub fn avifImageAllocatePlanes(image: *mut avifImage, planes: u32); // Ignores any pre-existing planes
    pub fn avifImageFreePlanes(image: *mut avifImage, planes: u32); // Ignores any pre-existing planes

    pub fn avifVersion() -> *const libc::c_char;
    pub fn avifCodecVersions(outBuffer: *mut libc::c_char);

    /// Returns AVIF_TRUE if input begins with a valid FileTypeBox (ftyp) that supports
    /// either the brand 'avif' or 'avis' (or both), without performing any allocations.
    pub fn avifPeekCompatibleFileType(input: *const avifROData) -> avifBool;
}
