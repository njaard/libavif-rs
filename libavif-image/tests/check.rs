use image::{GenericImageView, Pixel};

#[test]
fn images() {
    for non_avifs in ["rgb.jpg"].iter() {
        let path = format!("tests/{}", non_avifs);
        let image = image::open(path).expect("image::open non avif input");

        let avif = libavif_image::save(&image).expect("encode avif");
        assert!(libavif_image::is_avif(avif.as_slice()));

        let image2 = libavif_image::read(avif.as_slice()).expect("decode avif");
        assert_eq!(image.width(), image2.width());
        assert_eq!(image.height(), image2.height());

        let image = image.to_rgba();
        let image2 = image2.to_rgba();

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
}
