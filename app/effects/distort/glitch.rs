use image::RgbaImage;

pub fn apply(input: &RgbaImage, amount: f32) -> RgbaImage {
    let amount = amount.clamp(0.0, 80.0) as u32;
    if amount == 0 {
        return input.clone();
    }
    let mut output = input.clone();
    let (width, height) = input.dimensions();
    for y in 0..height {
        let seed = y.wrapping_mul(2654435761);
        if seed % 11 < amount.min(7) {
            let shift = (seed % (amount + 1)) as i32 - amount as i32 / 2;
            for x in 0..width {
                let source_x = (x as i32 + shift).rem_euclid(width.max(1) as i32) as u32;
                output.put_pixel(x, y, *input.get_pixel(source_x, y));
            }
        }
    }
    output
}
