//! Load and Save AVIF from [image](https://crates.io/crates/image)
//! types.
//!
//! Converts to and from YUV (`image` only does RGB).

/// Very efficiently detects AVIF files
///
/// returns true if the file header matches the AVIF type
/// Does not necessarily confirm that the file can actually
/// be decoded.
pub fn is_avif(bytes: &[u8]) -> bool
{
	if bytes.len() < 14 { return false; }
	eprintln!("bytes = {:?}", &bytes[4.. 12]);
	&bytes[4.. 12] == b"ftypavif"
}

/// Read data that is in an AVIF file and load it into an image
pub fn read(bytes: &[u8])
	-> Result<image::DynamicImage, String>
{
	let pixels = libavif::decode_rgb(bytes)
		.map_err(|e| format!("decoding AVIF: {}", e))?;
	let mut im = image::RgbImage::new(pixels.width(), pixels.height());

	for y in 0 .. im.height()
	{
		for x in 0 .. im.width()
		{
			let (r,g,b,_a) = pixels.pixel(x,y);
			im.put_pixel(x,y, [r,g,b].into());
		}
	}

	Ok(image::DynamicImage::ImageRgb8(im))
}


/// Save an image into an AVIF file
pub fn save(src_image: &image::DynamicImage)
	-> Result<Vec<u8>, String>
{
	let src;
	match src_image
	{
		image::DynamicImage::ImageRgb8(image)
			=> src = image,
		_ => return Err("image type not supported".to_string()),
	}

	let rows = src.rows()
		.map(
			|row|
			{
				row
					.map(
						|c|
							(c[0],c[1],c[2])
					)
					.collect()
			}
		);

	let data = libavif::encode_rgb(
		src.width(),
		src.height(),
		rows,
		0,
	).map_err(|e| format!("encoding AVIF: {:?}", e))?;
	Ok(data)
}


