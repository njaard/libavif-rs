use libavif_sys as sys;

pub struct RgbPixels {
    pub(crate) rgb: sys::avifRGBImage,
}

impl RgbPixels {
    /// width of the image in pixels
    pub fn width(&self) -> u32 {
        self.rgb.width
    }

    /// height of the image in pixels
    pub fn height(&self) -> u32 {
        self.rgb.height
    }

    pub fn pixel(&self, x: u32, y: u32) -> (u8, u8, u8, u8) {
        assert!(x < self.width());
        assert!(y < self.height());

        unsafe {
            let pixels = self.rgb.pixels;
            let row_bytes = self.rgb.rowBytes as usize;
            let rgb = pixels.add((4 * x as usize) + (row_bytes * y as usize));
            let r = *rgb.add(0);
            let g = *rgb.add(1);
            let b = *rgb.add(2);
            let a = *rgb.add(3);
            (r, g, b, a)
        }
    }
}

impl Drop for RgbPixels {
    fn drop(&mut self) {
        unsafe {
            sys::avifRGBImageFreePixels(&mut self.rgb as *mut sys::avifRGBImage);
        }
    }
}
