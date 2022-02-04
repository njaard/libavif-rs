use image::{DynamicImage, Pixel};

#[test]
fn images() {
    fn do_test(image: &DynamicImage) {
        let avif = libavif_image::save(&image).expect("encode avif");
        assert!(libavif_image::is_avif(avif.as_slice()));

        let image2 = libavif_image::read(avif.as_slice()).expect("decode avif");
        assert_eq!(image.width(), image2.width());
        assert_eq!(image.height(), image2.height());

        let image = image.to_rgba8();
        let image2 = image2.to_rgba8();

        let diff = image
            .pixels()
            .zip(image2.pixels())
            .map(|(pix1, pix2)| {
                pix1.channels()
                    .iter()
                    .zip(pix2.channels().iter())
                    .map(|(p1, p2)| (p1.max(p2) - p1.min(p2)) as u64)
                    .sum::<u64>()
            })
            .sum::<u64>()
            / image.pixels().count() as u64;
        assert!(diff < 20);
    }

    for non_avifs in ["rgb.jpg"].iter() {
        let path = format!("tests/{}", non_avifs);
        let rgb = image::open(path).expect("image::open non avif input");

        do_test(&rgb);

        let luma = DynamicImage::ImageLuma8(rgb.to_luma8());
        do_test(&luma);
    }
}
