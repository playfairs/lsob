use crate::effects::core;
use image::RgbaImage;

pub fn apply(input: &RgbaImage, size: f32) -> RgbaImage {
    core::pixelate(input, size.max(1.0) as u32)
}
