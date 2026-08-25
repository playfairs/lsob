pub mod artistic;
pub mod blur;
pub mod color;
pub mod core;
pub mod decor;
pub mod distort;
pub mod edge_detect;
pub mod enhance;
pub mod light_shadow;
pub mod noise;
pub mod registry;

use image::RgbaImage;

use registry::Filter;

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
    pub fn id(&self) -> &'static str {
        match self {
            Self::GaussianBlur { .. } => "gaussian-blur",
            Self::Pixelate { .. } => "pixelate",
            Self::BoxBlur { .. } => "box-blur",
            Self::Brightness { .. } => "brightness",
            Self::Contrast { .. } => "contrast",
            Self::HueShift { .. } => "hue-shift",
            Self::Sharpen { .. } => "sharpen",
            Self::Noise { .. } => "rgb-noise",
            Self::RgbShift { .. } => "rgb-shift",
            Self::RadialBlur { .. } => "radial-blur",
            Self::Melt { .. } => "melt",
            Self::Glitch { .. } => "glitch",
            Self::Finish { .. } => "finish",
        }
    }

    pub fn name(&self) -> &'static str {
        registry::find(self.id()).map_or("Unknown effect", |filter| filter.metadata().name)
    }

    pub fn apply(&self, input: &RgbaImage) -> RgbaImage {
        registry::apply(self.id(), input, self.value()).unwrap_or_else(|| input.clone())
    }

    fn value(&self) -> f32 {
        match self {
            Self::GaussianBlur { radius } => *radius,
            Self::Pixelate { size } => *size as f32,
            Self::BoxBlur { radius } => *radius as f32,
            Self::Brightness { amount } => *amount as f32,
            Self::Contrast { amount } => *amount,
            Self::HueShift { degrees } => *degrees as f32,
            Self::Sharpen { amount } => *amount,
            Self::Noise { amount } => *amount as f32,
            Self::RgbShift { amount } => *amount as f32,
            Self::RadialBlur { amount } => *amount as f32,
            Self::Melt { amount } => *amount as f32,
            Self::Glitch { amount } => *amount as f32,
            Self::Finish { amount } => *amount as f32,
        }
    }
}
