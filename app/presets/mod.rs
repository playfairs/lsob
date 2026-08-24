use crate::effects::Effect;

pub fn named(name: &str) -> Vec<Effect> {
    match name {
        "Soft Blur" => vec![Effect::GaussianBlur { radius: 4.0 }],
        "Extremely Blurry" => vec![Effect::GaussianBlur { radius: 18.0 }],
        "Pixel Emoji" => vec![Effect::Pixelate { size: 12 }],
        "Low Quality" => vec![
            Effect::Pixelate { size: 8 },
            Effect::GaussianBlur { radius: 2.0 },
        ],
        "Destroyed" => vec![
            Effect::GaussianBlur { radius: 8.0 },
            Effect::Pixelate { size: 10 },
        ],
        _ => Vec::new(),
    }
}
