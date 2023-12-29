use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(long)]
    pub width: Option<u32>,

    #[arg(long)]
    pub height: Option<u32>,

    #[arg(long, default_value = "sierra2")]
    pub dithering: DitheringOption,

    #[arg(long)]
    pub allow_blank_chars: bool,

    #[arg(long)]
    pub invert: bool,

    #[arg(long, default_value = "1.0")]
    pub contrast: f32,

    #[arg(long, default_value = "1")]
    pub brighten: i32,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    pub file: String
}

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum DitheringOption {
    #[default]
    Sierra2,
    None
}
