use image::{Rgba, RgbaImage};

pub fn apply(input: &RgbaImage, amount: f32) -> RgbaImage {
    let thickness = (amount.clamp(0.0, 100.0) / 10.0).round() as u32;
    let mut output = input.clone();
    let (width, height) = input.dimensions();
    for (x, y, pixel) in output.enumerate_pixels_mut() {
        if x < thickness
            || y < thickness
            || x >= width.saturating_sub(thickness)
            || y >= height.saturating_sub(thickness)
        {
            *pixel = Rgba([217, 243, 106, pixel.0[3]]);
        }
    }
    output
}
