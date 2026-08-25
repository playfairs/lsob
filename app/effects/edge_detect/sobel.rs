use image::{Rgba, RgbaImage};

pub fn apply(input: &RgbaImage, amount: f32) -> RgbaImage {
    let amount = amount.clamp(0.0, 100.0) / 100.0;
    let (width, height) = input.dimensions();
    let mut output = input.clone();
    if width < 3 || height < 3 {
        return output;
    }
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let sample = |dx: u32, dy: u32| -> f32 {
                let pixel = input.get_pixel(x + dx - 1, y + dy - 1).0;
                0.299 * f32::from(pixel[0])
                    + 0.587 * f32::from(pixel[1])
                    + 0.114 * f32::from(pixel[2])
            };
            let horizontal = sample(2, 0) + 2.0 * sample(2, 1) + sample(2, 2)
                - sample(0, 0)
                - 2.0 * sample(0, 1)
                - sample(0, 2);
            let vertical = sample(0, 2) + 2.0 * sample(1, 2) + sample(2, 2)
                - sample(0, 0)
                - 2.0 * sample(1, 0)
                - sample(2, 0);
            let edge = (horizontal.hypot(vertical) * amount).clamp(0.0, 255.0) as u8;
            output.put_pixel(x, y, Rgba([edge, edge, edge, input.get_pixel(x, y).0[3]]));
        }
    }
    output
}
