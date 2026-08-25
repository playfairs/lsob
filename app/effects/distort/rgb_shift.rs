use image::RgbaImage;

pub fn apply(input: &RgbaImage, amount: f32) -> RgbaImage {
    let mut output = input.clone();
    let width = input.width();
    let amount = amount.max(0.0) as u32;
    for (x, y, pixel) in output.enumerate_pixels_mut() {
        let red_x = x.saturating_sub(amount).min(width.saturating_sub(1));
        let blue_x = x.saturating_add(amount).min(width.saturating_sub(1));
        pixel.0[0] = input.get_pixel(red_x, y).0[0];
        pixel.0[2] = input.get_pixel(blue_x, y).0[2];
    }
    output
}
