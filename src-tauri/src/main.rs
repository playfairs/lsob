use base64::{engine::general_purpose::STANDARD, Engine};
use image::{DynamicImage, ImageFormat, RgbaImage};
use lsob::effects::Effect;
use serde::Deserialize;
use std::{fs, io::Cursor};

#[derive(Clone, Deserialize)]
struct EffectSpec {
    kind: String,
    value: f32,
    enabled: bool,
}

#[tauri::command]
fn preview_image(bytes: Vec<u8>, effects: Vec<EffectSpec>) -> Result<String, String> {
    let image = decode(&bytes)?;
    let rendered = apply_effects(image, &effects);
    encode_png(rendered)
}

#[tauri::command]
fn save_preview(data_url: String, output_path: String) -> Result<(), String> {
    let encoded = data_url
        .split_once(',')
        .map(|(_, value)| value)
        .ok_or_else(|| String::from("Invalid preview data"))?;
    let bytes = STANDARD.decode(encoded).map_err(|error| error.to_string())?;
    fs::write(output_path, bytes).map_err(|error| error.to_string())
}

fn decode(bytes: &[u8]) -> Result<RgbaImage, String> {
    image::load_from_memory(bytes)
        .map(|image| image.to_rgba8())
        .map_err(|error| error.to_string())
}

fn apply_effects(mut image: RgbaImage, effects: &[EffectSpec]) -> RgbaImage {
    for effect in effects.iter().filter(|effect| effect.enabled) {
        let operation = match effect.kind.as_str() {
            "blur" => Effect::GaussianBlur { radius: effect.value },
            "pixelate" => Effect::Pixelate { size: effect.value.max(1.0) as u32 },
            "brightness" => Effect::Brightness { amount: effect.value as i32 },
            "contrast" => Effect::Contrast { amount: effect.value },
            "hue" => Effect::HueShift { degrees: effect.value as i32 },
            "sharpen" => Effect::Sharpen { amount: effect.value },
            "noise" => Effect::Noise { amount: effect.value.max(0.0) as u8 },
            "rgb" => Effect::RgbShift { amount: effect.value.max(0.0) as u32 },
            "radial" => Effect::RadialBlur { amount: effect.value.max(0.0) as u32 },
            "melt" => Effect::Melt { amount: effect.value.max(0.0) as u32 },
            "glitch" => Effect::Glitch { amount: effect.value.max(0.0) as u32 },
            _ => continue,
        };
        image = operation.apply(&image);
    }
    image
}

fn encode_png(image: RgbaImage) -> Result<String, String> {
    let bytes = encode_bytes(image, ImageFormat::Png)?;
    Ok(format!("data:image/png;base64,{}", STANDARD.encode(bytes)))
}

fn encode_bytes(image: RgbaImage, format: ImageFormat) -> Result<Vec<u8>, String> {
    let mut bytes = Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(image)
        .write_to(&mut bytes, format)
        .map_err(|error| error.to_string())?;
    Ok(bytes.into_inner())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![preview_image, save_preview])
        .run(tauri::generate_context!())
        .expect("error while running l_SOB");
}
