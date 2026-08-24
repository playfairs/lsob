use image::RgbaImage;

#[derive(Clone, Debug)]
pub enum Effect {
    GaussianBlur { radius: f32 },
    Pixelate { size: u32 },
    BoxBlur { radius: u32 },
    Brightness { amount: i32 },
    Contrast { amount: f32 },
    HueShift { degrees: i32 },
    Sharpen { amount: f32 },
    Noise { amount: u8 },
    RgbShift { amount: u32 },
    RadialBlur { amount: u32 },
    Melt { amount: u32 },
    Glitch { amount: u32 },
    Finish { amount: u32 },
}

impl Effect {
    pub fn name(&self) -> &'static str {
        match self {
            Self::GaussianBlur { .. } => "Gaussian Blur",
            Self::Pixelate { .. } => "Pixelate",
            Self::BoxBlur { .. } => "Box Blur",
            Self::Brightness { .. } => "Brightness",
            Self::Contrast { .. } => "Contrast",
            Self::HueShift { .. } => "Hue Shift",
            Self::Sharpen { .. } => "Sharpen",
            Self::Noise { .. } => "Noise",
            Self::RgbShift { .. } => "RGB Shift",
            Self::RadialBlur { .. } => "Radial Blur",
            Self::Melt { .. } => "Melt",
            Self::Glitch { .. } => "Glitch",
            Self::Finish { .. } => "Finish",
        }
    }

    pub fn apply(&self, input: &RgbaImage) -> RgbaImage {
        match self {
            Self::GaussianBlur { radius } => image::imageops::blur(input, *radius),
            Self::BoxBlur { radius } => image::imageops::blur(input, *radius as f32),
            Self::Pixelate { size } => pixelate(input, (*size).max(1)),
            Self::Brightness { amount } => image::imageops::brighten(input, *amount),
            Self::Contrast { amount } => image::imageops::contrast(input, *amount),
            Self::HueShift { degrees } => image::imageops::huerotate(input, *degrees),
            Self::Sharpen { amount } if !amount.is_finite() || *amount <= 0.0 => input.clone(),
            Self::Sharpen { amount } => image::imageops::unsharpen(input, *amount, 1),
            Self::Noise { amount } => noise(input, *amount),
            Self::RgbShift { amount } => rgb_shift(input, *amount),
            Self::RadialBlur { amount } => radial_blur(input, *amount),
            Self::Melt { amount } => melt(input, *amount),
            Self::Glitch { amount } => glitch(input, *amount),
            Self::Finish { amount } => finish(input, *amount),
        }
    }
}

fn noise(input: &RgbaImage, amount: u8) -> RgbaImage {
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

fn rgb_shift(input: &RgbaImage, amount: u32) -> RgbaImage {
    let mut output = input.clone();
    let width = input.width();
    for (x, y, pixel) in output.enumerate_pixels_mut() {
        let red_x = x.saturating_sub(amount).min(width.saturating_sub(1));
        let blue_x = x.saturating_add(amount).min(width.saturating_sub(1));
        pixel.0[0] = input.get_pixel(red_x, y).0[0];
        pixel.0[2] = input.get_pixel(blue_x, y).0[2];
    }
    output
}

fn radial_blur(input: &RgbaImage, amount: u32) -> RgbaImage {
    if amount == 0 {
        return input.clone();
    }
    let mut output = input.clone();
    let (width, height) = input.dimensions();
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let samples = amount.clamp(2, 12);
    for y in 0..height {
        for x in 0..width {
            let mut sum = [0u32; 4];
            let mut count = 0u32;
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            for sample in 0..samples {
                let factor = sample as f32 / (samples - 1) as f32;
                let sample_x =
                    (center_x + dx * (1.0 - factor * amount as f32 / 100.0)).round() as i32;
                let sample_y =
                    (center_y + dy * (1.0 - factor * amount as f32 / 100.0)).round() as i32;
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

fn melt(input: &RgbaImage, amount: u32) -> RgbaImage {
    if amount == 0 {
        return input.clone();
    }
    let mut output = input.clone();
    let (width, height) = input.dimensions();
    for y in 0..height {
        let wave = ((y.wrapping_mul(1103515245) ^ (y / 7).wrapping_mul(12345))
            % (amount.min(80) + 1)) as i32;
        let shift = wave - amount.min(80) as i32 / 2;
        for x in 0..width {
            let source_x = (x as i32 + shift).clamp(0, width.saturating_sub(1) as i32) as u32;
            output.put_pixel(x, y, *input.get_pixel(source_x, y));
        }
    }
    output
}

fn glitch(input: &RgbaImage, amount: u32) -> RgbaImage {
    if amount == 0 {
        return input.clone();
    }
    let mut output = input.clone();
    let (width, height) = input.dimensions();
    let intensity = amount.min(80);
    for y in 0..height {
        let seed = y.wrapping_mul(2654435761);
        if seed % 11 < intensity.min(7) {
            let shift = ((seed % (intensity + 1)) as i32) - intensity as i32 / 2;
            for x in 0..width {
                let source_x = (x as i32 + shift).rem_euclid(width.max(1) as i32) as u32;
                output.put_pixel(x, y, *input.get_pixel(source_x, y));
            }
        }
    }
    output
}

fn finish(input: &RgbaImage, amount: u32) -> RgbaImage {
    if amount == 0 {
        return input.clone();
    }
    let intensity = amount.min(100);
    let smallest_side = input.width().min(input.height()).max(1);
    let detail = (100 - intensity) * smallest_side / 100;
    let proxy_side = detail.clamp(10, smallest_side);
    let proxy = image::imageops::thumbnail(input, proxy_side, proxy_side);
    let softened = image::imageops::blur(&proxy, 0.7 + intensity as f32 / 45.0);
    let mut output = image::imageops::resize(
        &softened,
        input.width(),
        input.height(),
        image::imageops::FilterType::Nearest,
    );
    let halo = image::imageops::blur(&output, 0.4 + intensity as f32 / 35.0);
    for (pixel, glow) in output.pixels_mut().zip(halo.pixels()) {
        let base_alpha = u16::from(pixel.0[3]);
        let glow_alpha = u16::from(glow.0[3]);
        let fringe = glow_alpha.saturating_sub(base_alpha) * 3 / 4;
        if fringe > 0 {
            let blend = fringe.min(180);
            pixel.0[0] = ((u16::from(pixel.0[0]) * (255 - blend) + 248 * blend) / 255) as u8;
            pixel.0[1] = ((u16::from(pixel.0[1]) * (255 - blend) + 248 * blend) / 255) as u8;
            pixel.0[2] = ((u16::from(pixel.0[2]) * (255 - blend) + 248 * blend) / 255) as u8;
            pixel.0[3] = pixel.0[3].max(fringe as u8);
        }
    }
    output
}

fn pixelate(input: &RgbaImage, size: u32) -> RgbaImage {
    let mut output = input.clone();
    let (width, height) = input.dimensions();
    for y in (0..height).step_by(size as usize) {
        for x in (0..width).step_by(size as usize) {
            let x_end = (x + size).min(width);
            let y_end = (y + size).min(height);
            let mut sums = [0u64; 4];
            let mut count = 0u64;
            for sample_y in y..y_end {
                for sample_x in x..x_end {
                    let pixel = input.get_pixel(sample_x, sample_y).0;
                    for channel in 0..4 {
                        sums[channel] += u64::from(pixel[channel]);
                    }
                    count += 1;
                }
            }
            let average = image::Rgba(sums.map(|value| (value / count) as u8));
            for block_y in y..y_end {
                for block_x in x..x_end {
                    output.put_pixel(block_x, block_y, average);
                }
            }
        }
    }
    output
}
