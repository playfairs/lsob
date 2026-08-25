use image::RgbaImage;

pub fn apply(input: &RgbaImage, amount: f32) -> RgbaImage {
    image::imageops::contrast(input, amount.clamp(-100.0, 100.0))
}
