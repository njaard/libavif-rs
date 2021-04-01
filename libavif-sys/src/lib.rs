#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "codec-aom")]
extern crate libaom_sys; // mark it as used

#[cfg(feature = "codec-dav1d")]
extern crate libdav1d_sys; // mark it as used

#[cfg(feature = "codec-rav1e")]
pub use rav1e::capi::*;

pub const AVIF_PLANE_COUNT_YUV: usize = 3;

pub type avifBool = libc::c_int;

pub type __enum = libc::c_int;

pub type avifPlanesFlags = __enum;
pub const AVIF_PLANES_YUV: avifPlanesFlags = 1;
pub const AVIF_PLANES_A: avifPlanesFlags = 1 << 1;
pub const AVIF_PLANES_ALL: avifPlanesFlags = 0xff;

pub type avifChannelIndex = __enum;
pub const AVIF_CHAN_R: avifChannelIndex = 0;
pub const AVIF_CHAN_G: avifChannelIndex = 1;
pub const AVIF_CHAN_B: avifChannelIndex = 2;

pub const AVIF_CHAN_Y: avifChannelIndex = 0;
pub const AVIF_CHAN_U: avifChannelIndex = 1;
pub const AVIF_CHAN_V: avifChannelIndex = 2;

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifImageTiming {
    /// timescale of the media (Hz)
    pub timescale: u64,
    /// presentation timestamp in seconds (ptsInTimescales / timescale)
    pub pts: f64,
    /// presentation timestamp in "timescales"
    pub ptsInTimescales: u64,
    /// in seconds (durationInTimescales / timescale)
    pub duration: f64,
    /// duration in "timescales"
    pub durationInTimescales: u64,
}

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
#[derive(Debug, Copy, Clone)]
pub struct avifPixelAspectRatioBox {
    pub hSpacing: u32,
    pub vSpacing: u32,
}

/// 'clap' from ISO/IEC 14496-12:2015 12.1.4.3
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
pub struct avifImageRotation {
    /// angle * 90 specifies the angle (in anti-clockwise direction) in units of degrees.
    /// legal values: [0-3]
    angle: u8,
}

/// 'imir' from ISO/IEC 23008-12:2017 6.5.12
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

pub const AVIF_CHROMA_UPSAMPLING_AUTOMATIC: avifChromaUpsampling = 0;
pub const AVIF_CHROMA_UPSAMPLING_FASTEST: avifChromaUpsampling = 1;
pub const AVIF_CHROMA_UPSAMPLING_BEST_QUALITY: avifChromaUpsampling = 2;
pub const AVIF_CHROMA_UPSAMPLING_NEAREST: avifChromaUpsampling = 3;
pub const AVIF_CHROMA_UPSAMPLING_BILINEAR: avifChromaUpsampling = 4;

pub type avifCodecChoice = __enum;
pub const AVIF_CODEC_CHOICE_AUTO: avifCodecChoice = 0;
pub const AVIF_CODEC_CHOICE_AOM: avifCodecChoice = 1;
pub const AVIF_CODEC_CHOICE_DAV1D: avifCodecChoice = 2;
//pub const AVIF_CODEC_CHOICE_LIBGAV1: avifCodecChoice = 3;
pub const AVIF_CODEC_CHOICE_RAV1E: avifCodecChoice = 4;
//pub const AVIF_CODEC_CHOICE_SVT: avifCodecChoice = 5;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifIOStats {
    colorOBUSize: libc::size_t,
    alphaOBUSize: libc::size_t,
}

type avifIOReadFunc = extern "C" fn(
    io: *mut avifIO,
    readFlags: u32,
    offset: u64,
    size: libc::size_t,
    out: *mut avifROData,
) -> avifResult;
type avifIOWriteFunc = extern "C" fn(
    io: *mut avifIO,
    writeFlags: u32,
    offset: u64,
    size: libc::size_t,
    data: *const u8,
    size: libc::size_t,
) -> avifResult;

#[repr(C)]
#[derive(Debug)]
pub struct avifIO {
    pub destroy: extern "C" fn(io: *mut avifIO),
    pub read: avifIOReadFunc,

    /// This is reserved for future use - but currently ignored. Set it to a null pointer.
    pub write: avifIOWriteFunc,

    /// If non-zero, this is a hint to internal structures of the max size offered by the content
    /// this avifIO structure is reading. If it is a static memory source, it should be the size of
    /// the memory buffer; if it is a file, it should be the file's size. If this information cannot
    /// be known (as it is streamed-in), set a reasonable upper boundary here (larger than the file
    /// can possibly be for your environment, but within your environment's memory constraints). This
    /// is used for sanity checks when allocating internal buffers to protect against
    /// malformed/malicious files.
    pub sizeHint: u64,

    // If true, *all* memory regions returned from *all* calls to read are guaranteed to be
    // persistent and exist for the lifetime of the avifIO object. If false, libavif will make
    // in-memory copies of samples and metadata content, and a memory region returned from read must
    // only persist until the next call to read.
    pub persistent: avifBool,

    // The contents of this are defined by the avifIO implementation, and should be fully destroyed
    // by the implementation of the associated destroy function, unless it isn't owned by the avifIO
    // struct. It is not necessary to use this pointer in your implementation.
    pub data: *mut libc::c_void,
}

pub const AVIF_QUANTIZER_LOSSLESS: libc::c_int = 0;
pub const AVIF_QUANTIZER_BEST_QUALITY: libc::c_int = 0;
pub const AVIF_QUANTIZER_WORST_QUALITY: libc::c_int = 63;

pub const AVIF_SPEED_DEFAULT: libc::c_int = -1;
pub const AVIF_SPEED_SLOWEST: libc::c_int = 0;
pub const AVIF_SPEED_FASTEST: libc::c_int = 10;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifPixelFormatInfo {
    pub monochrome: avifBool,
    pub chromaShiftX: libc::c_int,
    pub chromaShiftY: libc::c_int,
}

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
    pub alphaPremultiplied: avifBool,

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
    pub alphaPremultiplied: avifBool,
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
pub struct avifCodecSpecificOptions {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avifDecoderData {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avifDecoder {
    /// Defaults to AVIF_CODEC_CHOICE_AUTO: Preference determined by order in availableCodecs table (avif.c)
    pub codecChoice: avifCodecChoice,
    /// multithreading is disabled if <2)
    pub maxThreads: libc::c_int,
    // avifs can have multiple sets of images in them. This specifies which to decode.
    // Set this via avifDecoderSetSource().
    pub requestedSource: avifDecoderSource,
    /// All decoded image data; owned by the decoder. All information in this image is incrementally
    /// added and updated as avifDecoder*() functions are called. After a successful call to
    /// avifDecoderParse(), all values in decoder->image (other than the planes/rowBytes themselves)
    /// will be pre-populated with all information found in the outer AVIF container, prior to any
    /// AV1 decoding. If the contents of the inner AV1 payload disagree with the outer container,
    /// these values may change after calls to avifDecoderRead*(),avifDecoderNextImage(), or
    /// avifDecoderNthImage().
    ///
    /// The YUV and A contents of this image are likely owned by the decoder, so be sure to copy any
    /// data inside of this image before advancing to the next image or reusing the decoder. It is
    /// legal to call avifImageYUVToRGB() on this in between calls to avifDecoderNextImage(), but use
    /// avifImageCopy() if you want to make a complete, permanent copy of this image's YUV content or
    /// metadata.
    pub image: *mut avifImage,

    /// Counts and timing for the current image in an image sequence. Uninteresting for single image files.
    /// 9-based
    pub imageIndex: libc::c_int,
    /// Always 1 for non-sequences
    pub imageCount: libc::c_int,
    pub imageTiming: avifImageTiming,
    /// timescale of the media (Hz)
    pub timescale: u64,
    /// in seconds (durationInTimescales / timescale)
    pub duration: f64,
    /// duration in "timescales"
    pub durationInTimescales: u64,

    /// This is true when avifDecoderParse() detects an alpha plane. Use this to find out if alpha is
    /// present after a successful call to avifDecoderParse(), but prior to any call to
    /// avifDecoderNextImage() or avifDecoderNthImage(), as decoder->image->alphaPlane won't exist yet.
    pub alphaPresent: avifBool,

    /// Enable any of these to avoid reading and surfacing specific data to the decoded avifImage.
    /// These can be useful if your avifIO implementation heavily uses AVIF_RESULT_WAITING_ON_IO for
    /// streaming data, as some of these payloads are (unfortunately) packed at the end of the file,
    /// which will cause avifDecoderParse() to return AVIF_RESULT_WAITING_ON_IO until it finds them.
    /// If you don't actually leverage this data, it is best to ignore it here.
    pub ignoreExif: avifBool,
    pub ignoreXMP: avifBool,

    /// stats from the most recent read, possibly 0s if reading an image sequence
    pub ioStats: avifIOStats,

    /// Use one of the avifDecoderSetIO*() functions to set this
    pub io: *mut avifIO,

    /// Internals used by the decoder
    pub data: *mut avifDecoderData,
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
    pub csOptions: *mut avifCodecSpecificOptions,
}

pub type avifAddImageFlags = __enum;
pub const AVIF_ADD_IMAGE_FLAG_NONE: avifAddImageFlags = 0;

/// Force this frame to be a keyframe (sync frame).
pub const AVIF_ADD_IMAGE_FLAG_FORCE_KEYFRAME: avifAddImageFlags = 1 << 0;

/// Use this flag when encoding a single image. Signals "still_picture" to AV1 encoders, which
/// tweaks various compression rules. This is enabled automatically when using the
/// avifEncoderWrite() single-image encode path.
pub const AVIF_ADD_IMAGE_FLAG_SINGLE: avifAddImageFlags = 1 << 1;

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
pub const AVIF_RESULT_INVALID_IMAGE_GRID: avifResult = 18;
pub const AVIF_RESULT_INVALID_CODEC_SPECIFIC_OPTION: avifResult = 19;
pub const AVIF_RESULT_TRUNCATED_DATA: avifResult = 20;
/// the avifIO field of avifDecoder is not set
pub const AVIF_RESULT_IO_NOT_SET: avifResult = 21;
pub const AVIF_RESULT_IO_ERROR: avifResult = 22;
/// similar to EAGAIN/EWOULDBLOCK, this means the avifIO doesn't have necessary data available yet
pub const AVIF_RESULT_WAITING_ON_IO: avifResult = 23;
/// an argument passed into this function is invalid
pub const AVIF_RESULT_INVALID_ARGUMENT: avifResult = 24;
/// a requested code path is not (yet) implemented
pub const AVIF_RESULT_NOT_IMPLEMENTED: avifResult = 25;

#[repr(C)]
#[derive(Debug)]
pub struct avifROData {
    pub data: *const u8,
    pub size: libc::size_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifExtent {
    pub offset: u64,
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
    pub fn avifGetPixelFormatInfo(format: avifPixelFormat, info: *mut avifPixelFormatInfo);

    pub fn avifResultToString(result: avifResult) -> *const libc::c_char;

    pub fn avifColorPrimariesGetValues(acp: avifColorPrimaries, outPrimaries: *mut f32);
    pub fn avifColorPrimariesFind(
        inPrimaries: *const f32,
        outName: *mut *const libc::c_char,
    ) -> avifColorPrimaries;

    pub fn avifImageCreateEmpty() -> *mut avifImage;
    pub fn avifImageCreate(
        width: libc::c_int,
        height: libc::c_int,
        depth: libc::c_int,
        yuvFormat: avifPixelFormat,
    ) -> *mut avifImage;
    /// deep copy
    pub fn avifImageCopy(dstImage: *mut avifImage, srcImage: *const avifImage, planes: u32); // deep copy
    pub fn avifImageDestroy(image: *mut avifImage);

    pub fn avifImageSetProfileICC(image: *mut avifImage, icc: *const u8, iccSize: libc::size_t);
    pub fn avifImageSetMetadataExif(image: *mut avifImage, exif: *const u8, exifSize: libc::size_t);
    pub fn avifImageSetMetadataXMP(image: *mut avifImage, xmp: *const u8, xmpSize: libc::size_t);

    pub fn avifRGBFormatChannelCount(format: avifRGBFormat) -> u32;
    pub fn avifRGBFormatHasAlpha(format: avifRGBFormat) -> avifBool;

    pub fn avifEncoderCreate() -> *mut avifEncoder;
    pub fn avifEncoderWrite(
        encoder: *mut avifEncoder,
        image: *const avifImage,
        output: *mut avifRWData,
    ) -> avifResult;
    pub fn avifEncoderDestroy(encoder: *mut avifEncoder);

    pub fn avifEncoderAddImage(
        encoder: *mut avifEncoder,
        image: *const avifImage,
        durationInTimescales: u64,
        addImageFlags: u32,
    ) -> avifResult;
    pub fn avifEncoderAddImageGrid(
        encoder: *mut avifEncoder,
        gridCols: u32,
        gridRows: u32,
        cellImages: *const *const avifImage,
        addImageFlags: u32,
    ) -> avifResult;
    pub fn avifEncoderFinish(encoder: *mut avifEncoder, output: *mut avifRWData) -> avifResult;

    /// Codec-specific, optional "advanced" tuning settings, in the form of string key/value pairs. These
    /// should be set as early as possible, preferably just after creating avifEncoder but before
    /// performing any other actions.
    /// key must be non-NULL, but passing a NULL value will delete that key, if it exists.
    /// Setting an incorrect or unknown option for the current codec will cause errors of type
    /// AVIF_RESULT_INVALID_CODEC_SPECIFIC_OPTION from avifEncoderWrite() or avifEncoderAddImage().
    pub fn avifEncoderSetCodecSpecificOption(
        encoder: *mut avifEncoder,
        key: *const libc::c_char,
        value: *const libc::c_char,
    );

    pub fn avifImageUsesU16(image: *const avifImage) -> avifBool;

    pub fn avifDecoderCreate() -> *mut avifDecoder;
    pub fn avifDecoderDestroy(decoder: *mut avifDecoder);
    /// call avifDecoderSetIO*() first
    pub fn avifDecoderRead(decoder: *mut avifDecoder, image: *mut avifImage) -> avifResult;
    pub fn avifDecoderReadMemory(
        decoder: *mut avifDecoder,
        image: *mut avifImage,
        data: *const u8,
        size: libc::size_t,
    ) -> avifResult;
    pub fn avifDecoderReadFile(
        decoder: *mut avifDecoder,
        image: *mut avifImage,
        filename: *const libc::c_char,
    ) -> avifResult;

    pub fn avifDecoderSetSource(decoder: *mut avifDecoder, source: avifDecoderSource)
        -> avifResult;

    pub fn avifDecoderSetIO(decoder: *mut avifDecoder, io: *mut avifIO);
    pub fn avifDecoderSetIOMemory(
        decoder: *mut avifDecoder,
        data: *const u8,
        size: libc::size_t,
    ) -> avifResult;
    pub fn avifDecoderSetIOFile(
        decoder: *mut avifDecoder,
        filename: *const libc::c_char,
    ) -> avifResult;
    pub fn avifDecoderParse(decoder: *mut avifDecoder) -> avifResult;
    pub fn avifDecoderNextImage(decoder: *mut avifDecoder) -> avifResult;
    pub fn avifDecoderNthImage(decoder: *mut avifDecoder, frameIndex: u32) -> avifResult;
    pub fn avifDecoderReset(decoder: *mut avifDecoder) -> avifResult;

    pub fn avifDecoderIsKeyframe(decoder: *const avifDecoder, frameIndex: u32) -> avifBool;
    pub fn avifDecoderNearestKeyframe(decoder: *const avifDecoder, frameIndex: u32) -> u32;

    pub fn avifDecoderNthImageTiming(
        decoder: *const avifDecoder,
        frameIndex: u32,
        outTiming: *mut avifImageTiming,
    ) -> avifResult;

    pub fn avifDecoderNthImageMaxExtent(
        decoder: *const avifDecoder,
        frameIndex: u32,
        outExtent: *mut avifExtent,
    ) -> avifResult;

    pub fn avifRWDataFree(raw: *mut avifRWData);

    pub fn avifRGBImageSetDefaults(rgb: *mut avifRGBImage, image: *const avifImage);
    pub fn avifRGBImagePixelSize(rgb: *const avifRGBImage) -> u32;
    pub fn avifRGBImageAllocatePixels(rgb: *mut avifRGBImage);
    pub fn avifRGBImageFreePixels(rgb: *mut avifRGBImage);

    pub fn avifImageRGBToYUV(image: *mut avifImage, rgb: *const avifRGBImage) -> avifResult;
    pub fn avifImageYUVToRGB(image: *const avifImage, rgb: *mut avifRGBImage) -> avifResult;

    pub fn avifImageAllocatePlanes(image: *mut avifImage, planes: u32); // Ignores any pre-existing planes
    pub fn avifImageFreePlanes(image: *mut avifImage, planes: u32); // Ignores any pre-existing planes

    pub fn avifVersion() -> *const libc::c_char;
    pub fn avifCodecVersions(outBuffer: *mut libc::c_char);

    /// Returns AVIF_TRUE if input begins with a valid FileTypeBox (ftyp) that supports
    /// either the brand 'avif' or 'avis' (or both), without performing any allocations.
    pub fn avifPeekCompatibleFileType(input: *const avifROData) -> avifBool;

    pub fn avifRGBImagePremultiplyAlpha(rgb: *mut avifRGBImage) -> avifResult;
    pub fn avifRGBImageUnpremultiplyAlpha(rgb: *mut avifRGBImage) -> avifResult;

    pub fn avifFullToLimitedY(depth: libc::c_int, v: libc::c_int) -> libc::c_int;
    pub fn avifFullToLimitedUV(depth: libc::c_int, v: libc::c_int) -> libc::c_int;
    pub fn avifLimitedToFullY(depth: libc::c_int, v: libc::c_int) -> libc::c_int;
    pub fn avifLimitedToFullUV(depth: libc::c_int, v: libc::c_int) -> libc::c_int;

    pub fn avifIOCreateMemoryReader(data: *const u8, size: libc::size_t) -> *mut avifIO;
    pub fn avifIOCreateFileReader(filename: *const libc::c_char) -> *mut avifIO;
    pub fn avifIODestroy(io: *mut avifIO);

}
