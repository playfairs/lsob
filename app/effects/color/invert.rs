use image::RgbaImage;

pub fn apply(input: &RgbaImage, amount: f32) -> RgbaImage {
    let amount = amount.clamp(0.0, 100.0) / 100.0;
    let mut output = input.clone();
    for pixel in output.pixels_mut() {
        for channel in &mut pixel.0[..3] {
            let inverted = 255.0 - f32::from(*channel);
            *channel = (f32::from(*channel) * (1.0 - amount) + inverted * amount) as u8;
        }
    }
    output
}
