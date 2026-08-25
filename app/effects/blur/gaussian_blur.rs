use image::RgbaImage;

pub fn apply(input: &RgbaImage, radius: f32) -> RgbaImage {
    image::imageops::blur(input, radius.max(0.0))
}
