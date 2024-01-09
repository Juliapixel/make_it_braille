use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// width in dots of the output image, defaults to 64, keeps aspect ratio if
    /// only height is defined instead
    #[arg(long, long_help, help = "width in dots of the output image")]
    #[arg(value_parser = validate_greater_than_zero)]
    pub width: Option<u32>,

    /// height in dots of the output image, keeps aspect ratio if not defined
    #[arg(long, long_help, help = "height in dots of output image")]
    #[arg(value_parser = validate_greater_than_zero)]
    pub height: Option<u32>,

    /// dithering algorithm to use, defaults to the Sierra two-row algorithm
    #[arg(long, long_help, default_value = "sierra2", help = "dithering algorithm to use")]
    pub dithering: DitheringOption,

    /// allows blank braille characters, instead of replacing them with a single dot,
    /// which can cause images to appear skewed, especially on windows, even with
    /// a monospace font.
    #[arg(long, long_help, help = "allow blank braille characters")]
    pub allow_blank_chars: bool,

    /// invert dots, making light values in the source image be raised dots instead
    #[arg(long)]
    pub invert: bool,

    /// adjust contrast, positive values increase contrast, negative values decrease it
    #[arg(long, long_help, default_value = "0.0", help = "adjust contrast")]
    pub contrast: f32,

    /// adjust brightness, positive values increase brightness, negative values decrease it
    #[arg(long, long_help, default_value = "0", help = "adjust brightness")]
    pub brighten: i32,

    /// -v to see INFO logging, -vv to see DEBUG logging
    #[arg(short, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// either the path to a local image file, an http(s) URL to one or "-" to read from stdin
    pub file: String
}

fn validate_greater_than_zero(val: &str) -> Result<u32, &'static str> {
    match val.parse::<u32>() {
        Ok(o) => {
            if o > 0 {
                return Ok(o)
            } else {
                return Err("this argument cannot be 0")
            }
        },
        Err(_) => Err("must be a positive integer"),
    }
}

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum DitheringOption {
    #[default]
    Sierra2,
    None
}
