use base64::{engine::general_purpose::STANDARD, Engine};
use image::{DynamicImage, ImageFormat, RgbaImage};
use lsob::effects::registry::{self, Filter, ParameterType};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::BTreeSet, fs, io::Cursor, path::Path};

#[derive(Clone, Deserialize)]
struct EffectSpec {
    kind: String,
    value: f32,
    enabled: bool,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct FilterInfo {
    id: &'static str,
    name: &'static str,
    category: &'static str,
    description: &'static str,
    aliases: Vec<String>,
    supports_preview: bool,
    parameters: Vec<ParameterInfo>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ParameterInfo {
    id: &'static str,
    name: &'static str,
    #[serde(rename = "type")]
    type_: &'static str,
    default: f32,
    min: Option<f32>,
    max: Option<f32>,
    step: Option<f32>,
    unit: Option<String>,
    options: Vec<EffectOption>,
}

#[derive(Clone, Serialize, Deserialize)]
struct EffectOption {
    label: String,
    value: String,
}

#[tauri::command]
fn get_effects() -> Vec<FilterInfo> {
    registry::all().into_iter().map(effect_info).collect()
}

#[tauri::command]
fn get_effect_categories() -> Vec<String> {
    let mut categories = BTreeSet::new();
    for filter in registry::all() {
        categories.insert(filter.metadata().category.to_string());
    }
    categories.into_iter().collect()
}

#[tauri::command]
fn get_effect(id: String) -> Option<FilterInfo> {
    registry::find(&id).map(effect_info)
}

#[tauri::command]
fn preview_effect(bytes: Vec<u8>, effect_id: String, value: f32) -> Result<String, String> {
    let image = decode(&bytes)?;
    let filter = registry::find(&effect_id)
        .ok_or_else(|| format!("Unknown filter: {}", effect_id))?;
    let rendered = filter.apply(&image, value);
    encode_png(rendered)
}

#[tauri::command]
fn apply_effect(bytes: Vec<u8>, effect_id: String, value: f32) -> Result<String, String> {
    preview_effect(bytes, effect_id, value)
}

#[tauri::command]
fn list_filters() -> Vec<FilterInfo> {
    get_effects()
}

#[tauri::command]
fn preview_image(bytes: Vec<u8>, effects: Vec<EffectSpec>) -> Result<String, String> {
    let image = decode(&bytes)?;
    let rendered = apply_effects(image, &effects)?;
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

#[tauri::command]
fn load_image(path: String) -> Result<String, String> {
    let bytes = fs::read(&path).map_err(|error| error.to_string())?;
    let mime = match Path::new(&path)
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "tif" | "tiff" => "image/tiff",
        _ => "image/png",
    };
    Ok(format!("data:{mime};base64,{}", STANDARD.encode(bytes)))
}

fn effect_info(filter: registry::FilterEntry) -> FilterInfo {
    let metadata = filter.metadata();
    let parameter = metadata.parameter;

    FilterInfo {
        id: metadata.id,
        name: metadata.name,
        category: metadata.category,
        description: metadata.description,
        aliases: vec![
            metadata.category.to_ascii_lowercase(),
            metadata.name.to_ascii_lowercase(),
            metadata.id.to_string(),
        ],
        supports_preview: metadata.supports_preview,
        parameters: vec![ParameterInfo {
            id: parameter.id,
            name: parameter.name,
            type_: match parameter.parameter_type {
                ParameterType::Number => "number",
            },
            default: parameter.default,
            min: Some(parameter.min),
            max: Some(parameter.max),
            step: Some(0.5),
            unit: Some(parameter.unit.to_string()),
            options: Vec::new(),
        }],
    }
}

fn decode(bytes: &[u8]) -> Result<RgbaImage, String> {
    image::load_from_memory(bytes)
        .map(|image| image.to_rgba8())
        .map_err(|error| error.to_string())
}

fn apply_effects(mut image: RgbaImage, effects: &[EffectSpec]) -> Result<RgbaImage, String> {
    for effect in effects.iter().filter(|effect| effect.enabled) {
        let filter = registry::find(&effect.kind)
            .ok_or_else(|| format!("Unknown filter: {}", effect.kind))?;
        image = filter.apply(&image, effect.value);
    }
    Ok(image)
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
        .invoke_handler(tauri::generate_handler![
            get_effects,
            get_effect_categories,
            get_effect,
            preview_effect,
            apply_effect,
            list_filters,
            preview_image,
            save_preview,
            load_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running l_SOB");
}
