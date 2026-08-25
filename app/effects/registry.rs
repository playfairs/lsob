use image::RgbaImage;

use super::{artistic, blur, color, decor, distort, edge_detect, enhance, light_shadow, noise};

#[derive(Clone, Copy, Debug)]
pub enum ParameterType {
    Number,
}

#[derive(Clone, Copy, Debug)]
pub struct ParameterDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub parameter_type: ParameterType,
    pub default: f32,
    pub min: f32,
    pub max: f32,
    pub unit: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct FilterMetadata {
    pub id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub description: &'static str,
    pub parameter: ParameterDefinition,
    pub supports_preview: bool,
}

pub trait Filter: Send + Sync {
    fn metadata(&self) -> FilterMetadata;
    fn apply(&self, input: &RgbaImage, value: f32) -> RgbaImage;
}

#[derive(Clone, Copy)]
pub struct FilterEntry {
    metadata: FilterMetadata,
    apply_fn: fn(&RgbaImage, f32) -> RgbaImage,
}

impl Filter for FilterEntry {
    fn metadata(&self) -> FilterMetadata {
        self.metadata
    }
    fn apply(&self, input: &RgbaImage, value: f32) -> RgbaImage {
        (self.apply_fn)(input, value)
    }
}

const fn parameter(
    id: &'static str,
    name: &'static str,
    default: f32,
    min: f32,
    max: f32,
    unit: &'static str,
) -> ParameterDefinition {
    ParameterDefinition {
        id,
        name,
        parameter_type: ParameterType::Number,
        default,
        min,
        max,
        unit,
    }
}

const fn entry(
    id: &'static str,
    name: &'static str,
    category: &'static str,
    description: &'static str,
    parameter: ParameterDefinition,
    apply_fn: fn(&RgbaImage, f32) -> RgbaImage,
) -> FilterEntry {
    FilterEntry {
        metadata: FilterMetadata {
            id,
            name,
            category,
            description,
            parameter,
            supports_preview: true,
        },
        apply_fn,
    }
}

pub fn all() -> [FilterEntry; 17] {
    [
        entry(
            "gaussian-blur",
            "Gaussian Blur",
            "Blur",
            "Smooth pixels with a soft radius.",
            parameter("radius", "Radius", 8.0, 0.0, 32.0, "px"),
            blur::gaussian_blur::apply,
        ),
        entry(
            "box-blur",
            "Box Blur",
            "Blur",
            "Apply an even blur across neighboring pixels.",
            parameter("radius", "Radius", 8.0, 0.0, 32.0, "px"),
            blur::box_blur::apply,
        ),
        entry(
            "radial-blur",
            "Radial Blur",
            "Blur",
            "Pull pixels toward the center for a zooming blur.",
            parameter("amount", "Amount", 28.0, 0.0, 100.0, "%"),
            blur::radial_blur::apply,
        ),
        entry(
            "sharpen",
            "Sharpen",
            "Enhance",
            "Increase local edge contrast.",
            parameter("amount", "Amount", 2.0, 0.0, 8.0, "%"),
            enhance::sharpen::apply,
        ),
        entry(
            "brightness",
            "Brightness",
            "Color",
            "Shift the overall lightness.",
            parameter("amount", "Amount", 0.0, -100.0, 100.0, "%"),
            color::brightness::apply,
        ),
        entry(
            "contrast",
            "Contrast",
            "Color",
            "Expand or compress tonal contrast.",
            parameter("amount", "Amount", 0.0, -100.0, 100.0, "%"),
            color::contrast::apply,
        ),
        entry(
            "hue-shift",
            "Hue Shift",
            "Color",
            "Rotate colors around the hue wheel.",
            parameter("degrees", "Degrees", 0.0, -180.0, 180.0, "deg"),
            color::hue_shift::apply,
        ),
        entry(
            "invert",
            "Invert",
            "Color",
            "Reverse or partially reverse the color channels.",
            parameter("amount", "Amount", 100.0, 0.0, 100.0, "%"),
            color::invert::apply,
        ),
        entry(
            "rgb-noise",
            "RGB Noise",
            "Noise",
            "Add deterministic channel noise.",
            parameter("amount", "Amount", 18.0, 0.0, 80.0, "%"),
            noise::rgb_noise::apply,
        ),
        entry(
            "rgb-shift",
            "RGB Shift",
            "Distorts",
            "Separate red and blue channels horizontally.",
            parameter("amount", "Amount", 8.0, 0.0, 40.0, "px"),
            distort::rgb_shift::apply,
        ),
        entry(
            "pixelate",
            "Pixelate",
            "Distorts",
            "Reduce detail into blocky color fields.",
            parameter("size", "Block size", 10.0, 1.0, 64.0, "px"),
            distort::pixelate::apply,
        ),
        entry(
            "melt",
            "Melt",
            "Distorts",
            "Displace horizontal image bands.",
            parameter("amount", "Amount", 20.0, 0.0, 80.0, "%"),
            distort::melt::apply,
        ),
        entry(
            "glitch",
            "Glitch",
            "Distorts",
            "Shift selected scanlines for signal damage.",
            parameter("amount", "Amount", 25.0, 0.0, 80.0, "%"),
            distort::glitch::apply,
        ),
        entry(
            "finish",
            "Finish",
            "Artistic",
            "Compress detail into a luminous low-resolution finish.",
            parameter("amount", "Amount", 86.0, 0.0, 100.0, "%"),
            artistic::finish::apply,
        ),
        entry(
            "sobel",
            "Sobel Edge",
            "Edge Detect",
            "Extract directional edges with a Sobel operator.",
            parameter("amount", "Amount", 100.0, 0.0, 100.0, "%"),
            edge_detect::sobel::apply,
        ),
        entry(
            "vignette",
            "Vignette",
            "Light & Shadow",
            "Shade the image edges toward its center.",
            parameter("amount", "Amount", 45.0, 0.0, 100.0, "%"),
            light_shadow::vignette::apply,
        ),
        entry(
            "border",
            "Add Border",
            "Decor",
            "Frame the image with a bright border.",
            parameter("amount", "Thickness", 20.0, 0.0, 100.0, "%"),
            decor::border::apply,
        ),
    ]
}

pub fn find(id: &str) -> Option<FilterEntry> {
    all().into_iter().find(|filter| filter.metadata().id == id)
}

pub fn by_category(category: &str) -> Vec<FilterEntry> {
    all()
        .into_iter()
        .filter(|filter| filter.metadata().category == category)
        .collect()
}

pub fn apply(id: &str, input: &RgbaImage, value: f32) -> Option<RgbaImage> {
    find(id).map(|filter| filter.apply(input, value))
}
