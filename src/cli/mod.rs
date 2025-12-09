use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use reqwest::Url;

pub (crate) mod util;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// width in dots of the output image, defaults to 64, keeps aspect ratio if
    /// only height is defined instead
    #[arg(long, short, long_help, help = "width in dots of the output image")]
    #[arg(value_parser = validate_greater_than_zero)]
    pub width: Option<u32>,

    /// height in dots of the output image, keeps aspect ratio if not defined
    #[arg(long, short, long_help, help = "height in dots of output image")]
    #[arg(value_parser = validate_greater_than_zero)]
    pub height: Option<u32>,

    /// frame of animated image to use, starting at frame 0
    #[arg(long, short, long_help, help = "height in dots of output image")]
    pub frame: Option<u32>,

    /// dithering algorithm to use, defaults to the Sierra two-row algorithm
    #[arg(long, short, long_help, default_value = "sierra2", help = "dithering algorithm to use")]
    pub dithering: DitheringOption,

    /// allows blank braille characters, instead of replacing them with a single dot,
    /// which can cause images to appear skewed, especially on windows, even with
    /// a monospace font.
    #[arg(long, short='b', long_help, help = "allow blank braille characters")]
    pub allow_blank_chars: bool,

    /// invert dots, making light values in the source image be raised dots instead
    #[arg(long, short)]
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
    #[arg(value_parser = parse_mode)]
    pub input: Mode
}

#[derive(Debug, Clone)]
pub enum Mode {
    File(PathBuf),
    Url(Url),
    Stdin
}

fn parse_mode(val: &str) -> Result<Mode, &'static str> {
    if val == "-" {
        return Ok(Mode::Stdin)
    }

    let path = PathBuf::from(val);

    let exists = path.exists();
    if exists && path.is_file() {
        return Ok(Mode::File(path));
    } else if exists {
        return Err("the given path exists but is not a file");
    }

    match Url::try_from(val) {
        Ok(url) => match url.scheme().to_ascii_lowercase() {
            x if x == "http" || x == "https" => { Ok(Mode::Url(url)) },
            _ => { Err("the given URL must be either http or https") }
        },
        Err(_) => Err("the given input was not a valid argument"),
    }
}

fn validate_greater_than_zero(val: &str) -> Result<u32, &'static str> {
    match val.parse::<u32>() {
        Ok(o) => {
            if o > 0 {
                Ok(o)
            } else {
                Err("this argument cannot be 0")
            }
        },
        Err(_) => Err("must be a positive integer"),
    }
}

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum DitheringOption {
    #[default]
    #[value(alias("s2"))]
    Sierra2,
    #[value(alias("b4"))]
    Bayer4x4,
    #[value(alias("b2"))]
    Bayer2x2,
    #[value(alias("n"))]
    None,
}
