use anyhow::Result;
use clap::{CommandFactory, Parser};

use crate::effects::Effect;
use crate::image::LsobImage;

#[derive(Parser, Debug)]
#[command(
    name = "lsob",
    version,
    about = "lsob (l_SOB) is a simple CLI tool to lovingly destroy the clarity of emojis and images."
)]
pub struct Args {
    pub input: Option<String>,
    pub output: Option<String>,
    #[arg(long)]
    pub blur: Option<f32>,
    #[arg(long)]
    pub pixelate: Option<u32>,
}

pub fn run() -> Result<()> {
    let args = Args::parse();
    match (args.input, args.output) {
        (Some(input), Some(output)) => {
            let image = LsobImage::open(input)?;
            let mut effects = Vec::new();
            if let Some(radius) = args.blur {
                effects.push(Effect::GaussianBlur { radius });
            }
            if let Some(size) = args.pixelate {
                effects.push(Effect::Pixelate { size });
            }
            image.save(output, &effects)
        }
        _ => {
            Args::command().print_help()?;
            Ok(())
        }
    }
}
