use image::RgbaImage;

pub fn apply(input: &RgbaImage, degrees: f32) -> RgbaImage {
    image::imageops::huerotate(input, degrees.clamp(-180.0, 180.0) as i32)
}
