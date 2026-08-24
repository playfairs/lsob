use anyhow::Result;
use image::{DynamicImage, RgbaImage};
use rayon::prelude::*;
use std::sync::Arc;

use crate::effects::Effect;

#[derive(Clone, Debug)]
pub struct LsobImage {
    pub source: Arc<RgbaImage>,
}

impl LsobImage {
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self> {
        Ok(Self {
            source: Arc::new(image::open(path)?.to_rgba8()),
        })
    }

    pub fn from_dynamic(image: DynamicImage) -> Self {
        Self {
            source: Arc::new(image.to_rgba8()),
        }
    }

    pub fn render(&self, effects: &[Effect]) -> RgbaImage {
        effects
            .iter()
            .fold((*self.source).clone(), |image, effect| effect.apply(&image))
    }

    pub fn render_preview(&self, effects: &[Effect], max_dimension: u32) -> RgbaImage {
        let preview = image::imageops::thumbnail(&*self.source, max_dimension, max_dimension);
        effects
            .iter()
            .fold(preview, |image, effect| effect.apply(&image))
    }

    pub fn save(&self, path: impl AsRef<std::path::Path>, effects: &[Effect]) -> Result<()> {
        self.render(effects).save(path)?;
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct EffectStack {
    pub effects: Vec<Effect>,
}

impl EffectStack {
    pub fn render_parallel(&self, source: &RgbaImage) -> RgbaImage {
        let mut rendered = source.clone();
        for effect in &self.effects {
            rendered = effect.apply(&rendered);
        }
        rendered
    }

    pub fn names(&self) -> Vec<&'static str> {
        self.effects.par_iter().map(Effect::name).collect()
    }
}
