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
pub const AVIF_PLANES_RGB: avifPlanesFlags = 1 << 0;
pub const AVIF_PLANES_YUV: avifPlanesFlags = 1 << 1;
pub const AVIF_PLANES_A: avifPlanesFlags = 1 << 2;
pub const AVIF_PLANES_ALL: avifPlanesFlags = 0xff;

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
pub struct avifIOStats
{
    colorOBUSize: libc::size_t,
    alphaOBUSize: libc::size_t,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct avifNclxColorProfile
{
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
pub struct avifImage
{
	pub width: u32,
	pub height: u32,
	pub depth: u32, // all planes (RGB/YUV/A) must share this depth; if depth>8, all planes are uint16_t internally

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
pub struct avifDecoder{ _private: [u8; 0] }

#[allow(non_camel_case_types)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct avifEncoder
{
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

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct avifROData
{
	pub data: *const u8,
	pub size: libc::size_t,
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct avifRWData
{
	pub data: *mut u8,
	pub size: libc::size_t,
}

impl Default for avifRWData
{
	fn default() -> Self
		{ Self { data: std::ptr::null_mut(), size: 0 } }
}

#[link(name = "avif", kind="static")]
extern
{
	pub fn avifImageCreateEmpty() -> *mut avifImage;
	pub fn avifImageCreate(width: libc::c_int, height: libc::c_int, depth: libc::c_int, yuvFormat: avifPixelFormat) -> *mut avifImage;
	pub fn avifDecoderCreate() -> *mut avifDecoder;
	pub fn avifEncoderCreate() -> *mut avifEncoder;
	pub fn avifEncoderWrite(encoder: *mut avifEncoder, image: *mut avifImage, output: *mut avifRWData)
		-> avifResult;
	pub fn avifEncoderDestroy(encoder: *mut avifEncoder);

	pub fn avifDecoderRead(decoder: *mut avifDecoder, image: *mut avifImage, data: *mut avifROData)
		-> avifResult;
	pub fn avifRWDataFree(raw: *mut avifRWData);

	pub fn avifImageYUVToRGB(image: *mut avifImage) -> avifResult;
	pub fn avifImageRGBToYUV(image: *mut avifImage) -> avifResult;

	pub fn avifImageDestroy(image: *mut avifImage) -> avifResult;
	pub fn avifDecoderDestroy(decoder: *mut avifDecoder) -> avifResult;

	pub fn avifImageAllocatePlanes(image: *mut avifImage, planes: u32); // Ignores any pre-existing planes
	pub fn avifImageFreePlanes(image: *mut avifImage, planes: u32); // Ignores any pre-existing planes

	pub fn avifImageSetProfileNCLX(image: *mut avifImage, nclx: *mut avifNclxColorProfile);
}

