use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(long, long_help, help = "width in dots of the output image")]
    /// width in dots of the output image, defaults to 64, keeps aspect ratio if
    /// only height is defined instead
    pub width: Option<u32>,

    #[arg(long, long_help, help = "height in dots of output image")]
    /// height in dots of the output image, keeps aspect ratio if not defined
    pub height: Option<u32>,

    #[arg(long, long_help, default_value = "sierra2", help = "dithering algorithm to use")]
    /// dithering algorithm to use, defaults to the Sierra two-row algorithm
    pub dithering: DitheringOption,

    #[arg(long, long_help, help = "allow blank braille characters")]
    /// allows blank braille characters, instead of replacing them with a single dot,
    /// which can cause images to appear skewed, especially on windows, even with
    /// a monospace font.
    pub allow_blank_chars: bool,

    #[arg(long)]
    /// invert dots, making light values in the source image be raised dots instead
    pub invert: bool,

    #[arg(long, default_value = "1.0")]
    pub contrast: f32,

    #[arg(long, default_value = "1")]
    pub brighten: i32,

    #[arg(short, action = clap::ArgAction::Count)]
    /// -v to see INFO logging, -vv to see DEBUG logging
    pub verbose: u8,

    /// either the path to a local image file or an http(s) URL to one
    pub file: String
}

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum DitheringOption {
    #[default]
    Sierra2,
    None
}
