use image::RgbaImage;

pub fn apply(input: &RgbaImage, amount: f32) -> RgbaImage {
    let amount = amount.clamp(0.0, 80.0) as u8;
    let mut output = input.clone();
    for (x, y, pixel) in output.enumerate_pixels_mut() {
        let seed = x.wrapping_mul(374761393) ^ y.wrapping_mul(668265263);
        let offset = ((seed ^ (seed >> 13)).wrapping_mul(1274126177) % (u32::from(amount) * 2 + 1))
            as i16
            - i16::from(amount);
        for channel in &mut pixel.0[..3] {
            *channel = (i16::from(*channel) + offset).clamp(0, 255) as u8;
        }
    }
    output
}
