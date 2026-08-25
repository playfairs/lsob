use image::RgbaImage;

pub fn apply(input: &RgbaImage, amount: f32) -> RgbaImage {
    image::imageops::brighten(input, amount.clamp(-100.0, 100.0) as i32)
}
