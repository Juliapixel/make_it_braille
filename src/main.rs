#![cfg(feature = "bin")]

use std::{fs::read, io::{stdin, Read}, path::PathBuf, time::Instant};

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use image::GenericImageView;
use log::{debug, error};
use make_it_braille as lib;
use lib::{braille, dithering::{Ditherer, self}};

mod cli;
use cli::{util::{load_as_frames, try_get_from_url, Error}, Args, DitheringOption};

use crate::cli::Mode;

fn main() -> Result<(), Error>{
    let args = Args::parse();

    let dbg = match args.verbose {
        0 => "ERROR",
        1 => "INFO",
        _ => "DEBUG"
    };

    env_logger::init_from_env(
        env_logger::Env::new()
            .filter_or("BRAILLE_LOG", dbg)
    );

    debug!("parsed arguments: {args:#?}");

    let mut image = match args.input {
        Mode::File(path) => {
            debug!("opening image as file");
            let buf = read(path)?;
            match load_as_frames(buf, None)?.nth(args.frame.unwrap_or_default() as usize) {
                Some(Ok(f)) => image::DynamicImage::ImageRgba8(f.into_buffer()),
                Some(Err(e)) => {
                    error!("{e}");
                    return Err(e)?;
                }
                None => {
                    error!("no such frame");
                    return Err(Error::NoSuchFrame(args.frame.unwrap_or_default()));
                },
            }
        },
        Mode::Url(url) => {
            debug!("trying to fetch image as URL");
            match try_get_from_url(url) {
                Ok(mut o) => {
                    match o.nth(args.frame.unwrap_or_default() as usize) {
                        Some(Ok(f)) => image::DynamicImage::ImageRgba8(f.into_buffer()),
                        Some(Err(e)) => {
                            error!("{e}");
                            return Err(e)?;
                        }
                        None => {
                            error!("no such frame");
                            return Err(Error::NoSuchFrame(args.frame.unwrap_or_default()));
                        },
                    }
                },
                Err(e) => {
                    error!("{e}");
                    return Err(e)?;
                },
            }
        },
        Mode::Stdin => {
            debug!("reading image from stdin");
            let mut input = Vec::new();
            stdin().read_to_end(&mut input)?;

            match load_as_frames(input, None)?.nth(args.frame.unwrap_or_default() as usize) {
                Some(Ok(f)) => image::DynamicImage::ImageRgba8(f.into_buffer()),
                Some(Err(e)) => {
                    error!("{e}");
                    return Err(e)?;
                }
                None => {
                    error!("no such frame");
                    return Err(Error::NoSuchFrame(args.frame.unwrap_or_default()));
                },
            }
        },
        Mode::Completions(sh) => {
            let cmd = std::env::args().next().unwrap_or_else(|| env!("CARGO_BIN_NAME").to_string());
            generate(sh, &mut Args::command(), &cmd, &mut std::io::stdout());
            return Ok(());
        }
    };

    debug!("source image dimensions: {}x{}", image.width(), image.height());
    debug!("image color type: {:?}", image.color());

    let start = Instant::now();
    let (width, height) = match (args.width, args.height) {
        (None, None) => {
            let aspect_ratio = image.width() as f32 / image.height() as f32;
            let h = (64.0 / aspect_ratio).round() as u32;
            (64, h.clamp(1, u32::MAX))
        },
        (None, Some(h)) => {
            let aspect_ratio = image.width() as f32 / image.height() as f32;
            let w = (h as f32 * aspect_ratio).round() as u32;
            (w.clamp(1, u32::MAX), h.clamp(1, u32::MAX))
        },
        (Some(w), None) => {
            let aspect_ratio = image.width() as f32 / image.height() as f32;
            let h = (w as f32 / aspect_ratio).round() as u32;
            (w.clamp(1, u32::MAX), h.clamp(1, u32::MAX))
        }
        (Some(w), Some(h)) => (w.clamp(1, u32::MAX), h.clamp(1, u32::MAX)),
    };

    debug!("target dimensions: {}x{}", width, height);

    if (width, height) != image.dimensions() {
        image = image.resize_exact(width, height, image::imageops::FilterType::Triangle);
    }
    if args.contrast != 0.0 {
        image = image.adjust_contrast(args.contrast); // for some reason this also affects the alpha channel???
    }
    if args.brighten != 0 {
        image = image.brighten(args.brighten);
    }

    // this is just so i can make sure the output is right and the filters are working properly
    #[cfg(debug_assertions)]
    {
        let out_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/debug.png"));
        debug!("saving debug image to {}", out_dir.canonicalize().unwrap_or(out_dir.clone()).as_os_str().to_string_lossy());
        image.save(out_dir.clone()).unwrap();
    }

    let ditherer: Box<dyn Ditherer> = match args.dithering {
        DitheringOption::Sierra2 => Box::new(dithering::Sierra2Row),
        DitheringOption::None => Box::new(dithering::None),
        DitheringOption::Bayer4x4 => Box::new(dithering::Bayer4x4),
        DitheringOption::Bayer2x2 => Box::new(dithering::Bayer2x2),
    };

    let braille = braille::BrailleImg::from_image(
        image,
        ditherer,
        !args.invert
    );

    println!("{}", braille.as_str(!args.allow_blank_chars, true));

    debug!("turned image into braille in {}s", start.elapsed().as_secs_f32());

    Ok(())
}
