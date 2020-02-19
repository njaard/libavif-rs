use libavif_sys as sys;

pub struct RgbPixels
{
	decoded: *mut sys::avifImage,
}


impl RgbPixels
{
	/// width of the image in pixels
	pub fn width(&self) -> u32
	{
		unsafe { (*self.decoded).width }
	}

	/// height of the image in pixels
	pub fn height(&self) -> u32
	{
		unsafe { (*self.decoded).height }
	}

	pub fn pixel(&self, x: u32, y: u32) -> (u8,u8,u8,u8)
	{
		assert!(x < self.width());
		assert!(y < self.height());
		unsafe
		{
			let x = x as usize;
			let y = y as usize;

			let pitch = (*self.decoded).rgbRowBytes;
			let rgb = (*self.decoded).rgbPlanes;

			let r = * rgb[0].add(x + y*(pitch[0] as usize));
			let g = * rgb[1].add(x + y*(pitch[1] as usize));
			let b = * rgb[2].add(x + y*(pitch[2] as usize));
			let a;
			if ! (*self.decoded).alphaPlane.is_null()
			{
				let pitch = (*self.decoded).alphaRowBytes;
				let aplane = (*self.decoded).alphaPlane;
				a = *aplane.add(x + y*(pitch as usize));
			}
			else
			{
				a = 255;
			}
			(r,g,b,a)
		}
	}
}

impl Drop for RgbPixels
{
	fn drop(&mut self)
	{
		unsafe
		{
			sys::avifImageDestroy(self.decoded);
		}
	}
}

/// Decode into RGB pixels
pub fn decode_rgb(avif_bytes: &[u8])
	-> std::io::Result<RgbPixels>
{
	unsafe
	{
		let mut raw = sys::avifROData
		{
			data: avif_bytes.as_ptr(),
			size: avif_bytes.len(),
		};

		let decoded = sys::avifImageCreateEmpty();
		let decoder = sys::avifDecoderCreate();
		let result = sys::avifDecoderRead(decoder, decoded, &mut raw);
		sys::avifDecoderDestroy(decoder);
		if result != sys::AVIF_RESULT_OK
		{
			return Err(
				std::io::Error::new(
					std::io::ErrorKind::InvalidData,
					format!("result={}", result)
				)
			);
		}
		sys::avifImageYUVToRGB(decoded);
		Ok(RgbPixels{ decoded })
	}
}


/// Encode rows of pixels
pub fn encode_rgb<Rows: Iterator<Item=Vec<(u8,u8,u8)>>>(
	width: u32, height: u32, rows: Rows, _q: u32,
) -> std::io::Result<Vec<u8>>
{
	unsafe
	{
		let image = sys::avifImageCreate(
			width as _, height as _, 8, sys::AVIF_PIXEL_FORMAT_YUV444
		);
		sys::avifImageAllocatePlanes(image, sys::AVIF_PLANES_RGB as _);

		let width = width as usize;

		let pitch = (*image).rgbRowBytes;
		let rgb = (*image).rgbPlanes;

		for (y, row) in rows.enumerate()
		{
			for x in 0 .. width
			{
				* rgb[0].add(x + y*(pitch[0] as usize)) = row[x].0;
				* rgb[1].add(x + y*(pitch[1] as usize)) = row[x].1;
				* rgb[2].add(x + y*(pitch[2] as usize)) = row[x].2;
			}
		}

		let mut encoder = sys::avifEncoderCreate();
		(*encoder).maxThreads = 1;
		(*encoder).minQuantizer = 5;
		(*encoder).maxQuantizer = 40;
		let mut raw = Default::default();
		let result = sys::avifEncoderWrite(encoder, image, &mut raw);
		sys::avifEncoderDestroy(encoder);
		sys::avifImageDestroy(image);
		if result != sys::AVIF_RESULT_OK
		{
			return Err(
				std::io::Error::new(
					std::io::ErrorKind::InvalidData,
					format!("result={}", result)
				)
			);
		}

		let v = std::slice::from_raw_parts(raw.data, raw.size).to_vec();

		sys::avifRWDataFree(&mut raw);

		Ok(v)
	}
}

