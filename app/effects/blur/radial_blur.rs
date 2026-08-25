use image::RgbaImage;

pub fn apply(input: &RgbaImage, amount: f32) -> RgbaImage {
    let amount = amount.clamp(0.0, 100.0);
    if amount == 0.0 {
        return input.clone();
    }
    let mut output = input.clone();
    let (width, height) = input.dimensions();
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let samples = amount.round().clamp(2.0, 12.0) as u32;
    for y in 0..height {
        for x in 0..width {
            let mut sum = [0u32; 4];
            let mut count = 0u32;
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            for sample in 0..samples {
                let factor = sample as f32 / (samples - 1) as f32;
                let sample_x = (center_x + dx * (1.0 - factor * amount / 100.0)).round() as i32;
                let sample_y = (center_y + dy * (1.0 - factor * amount / 100.0)).round() as i32;
                if sample_x >= 0
                    && sample_x < width as i32
                    && sample_y >= 0
                    && sample_y < height as i32
                {
                    for (channel, value) in sum.iter_mut().enumerate() {
                        *value +=
                            u32::from(input.get_pixel(sample_x as u32, sample_y as u32).0[channel]);
                    }
                    count += 1;
                }
            }
            if count > 0 {
                output.put_pixel(x, y, image::Rgba(sum.map(|value| (value / count) as u8)));
            }
        }
    }
    output
}
