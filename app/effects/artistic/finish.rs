use image::RgbaImage;

pub fn apply(input: &RgbaImage, amount: f32) -> RgbaImage {
    let intensity = amount.clamp(0.0, 100.0) as u32;
    if intensity == 0 {
        return input.clone();
    }
    let smallest_side = input.width().min(input.height()).max(1);
    let detail = (100 - intensity) * smallest_side / 100;
    let proxy = image::imageops::thumbnail(
        input,
        detail.clamp(10, smallest_side),
        detail.clamp(10, smallest_side),
    );
    let softened = image::imageops::blur(&proxy, 0.7 + intensity as f32 / 45.0);
    let mut output = image::imageops::resize(
        &softened,
        input.width(),
        input.height(),
        image::imageops::FilterType::Nearest,
    );
    let halo = image::imageops::blur(&output, 0.4 + intensity as f32 / 35.0);
    for (pixel, glow) in output.pixels_mut().zip(halo.pixels()) {
        let fringe = u16::from(glow.0[3]).saturating_sub(u16::from(pixel.0[3])) * 3 / 4;
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
