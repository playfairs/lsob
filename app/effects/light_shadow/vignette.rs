use image::RgbaImage;

pub fn apply(input: &RgbaImage, amount: f32) -> RgbaImage {
    let amount = amount.clamp(0.0, 100.0) / 100.0;
    let (width, height) = input.dimensions();
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let max_distance = center_x.hypot(center_y).max(1.0);
    let mut output = input.clone();
    for (x, y, pixel) in output.enumerate_pixels_mut() {
        let distance =
            ((x as f32 - center_x).hypot(y as f32 - center_y) / max_distance).clamp(0.0, 1.0);
        let factor = 1.0 - distance * distance * amount * 0.85;
        for channel in &mut pixel.0[..3] {
            *channel = (f32::from(*channel) * factor) as u8;
        }
    }
    output
}
