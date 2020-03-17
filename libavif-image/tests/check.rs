use image::GenericImageView;

#[test]
fn images() {
    for non_avifs in ["rgb.jpg"].iter() {
        let path = format!("tests/{}", non_avifs);
        let image = image::open(path).expect("image::open non avif input");

        let width = image.width();
        let height = image.height();

        let avif = libavif_image::save(&image).expect("encode avif");
        assert!(libavif_image::is_avif(&avif));

        let image2 = libavif_image::read(&avif).expect("decode avif");
        assert_eq!(width, image2.width());
        assert_eq!(height, image2.height());
    }
}
