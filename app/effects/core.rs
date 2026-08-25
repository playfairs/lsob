use image::{Rgba, RgbaImage};

pub fn pixelate(input: &RgbaImage, size: u32) -> RgbaImage {
    let size = size.max(1);
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
                    for channel in 0..4 {
                        sums[channel] += u64::from(input.get_pixel(sample_x, sample_y).0[channel]);
                    }
                    count += 1;
                }
            }
            let average = Rgba(sums.map(|value| (value / count) as u8));
            for block_y in y..y_end {
                for block_x in x..x_end {
                    output.put_pixel(block_x, block_y, average);
                }
            }
        }
    }
    output
}
