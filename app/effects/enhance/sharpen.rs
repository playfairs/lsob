use image::RgbaImage;

pub fn apply(input: &RgbaImage, amount: f32) -> RgbaImage {
    if !amount.is_finite() || amount <= 0.0 {
        input.clone()
    } else {
        image::imageops::unsharpen(input, amount, 1)
    }
}
