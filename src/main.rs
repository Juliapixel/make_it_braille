use std::{path::PathBuf, time::Instant, str::FromStr, io::{Read, stdin}};

use clap::Parser;
use image::{DynamicImage, EncodableLayout};
use log::{debug, info, error};
use make_it_braille as lib;
use lib::{braille, dithering::{Ditherer, self}, cli::{Args, DitheringOption}};
use reqwest::{header::HeaderMap, Url};
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    InvalidImage(#[from] image::error::ImageError),
    #[error("there as an I/O error!")]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    FetchError(#[from] FetchError)
}

#[derive(Debug, Error)]
enum FetchError {
    #[error("the provided string was not an URL")]
    NotAnUrl,
    #[error("the provided URL was not valid")]
    BadUrl,
    #[error("the request could not be completed")]
    RequestError,
    #[error("the server's response was bad")]
    BadResponse,
    #[error(transparent)]
    BadImageData(#[from] image::error::ImageError)
}

fn try_get_from_url(url: &str) -> Result<DynamicImage, FetchError> {
    let url = Url::from_str(url).map_err(|_| { FetchError::NotAnUrl })?;

    let mut headers = HeaderMap::new();
    headers.insert("Accept", "image/png,image/jpeg,image/webp,image/gif,image/tiff".parse().unwrap());
    headers.insert("Host", url.host_str().ok_or(FetchError::BadUrl)?.parse().unwrap());

    let client = reqwest::blocking::ClientBuilder::new()
        .default_headers(headers)
        .build().unwrap();

    let resp = client.get(url)
        .send().map_err(|_| { FetchError::RequestError })?
        .bytes().map_err(|_| { FetchError::BadResponse })?;

    image::load_from_memory(resp.as_bytes()).map_err(|e| { FetchError::BadImageData(e) })
}

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

    let img_path = PathBuf::from(&args.file);

    info!("opening image: {}", args.file);

    let mut image = if img_path.is_file() {
        debug!("opening image as file");
        match image::open(args.file) {
            Ok(o) => o,
            Err(e) => {
                error!("{e}");
                return Err(e)?;
            }
        }
    } else if args.file == "-" {
        let mut input = Vec::new();
        stdin().read_to_end(&mut input)?;

        image::load_from_memory(&input)?
    } else {
        debug!("path either didnt exist or wasn't a file, trying to fetch image as URL");
        match try_get_from_url(&args.file) {
            Ok(o) => o,
            Err(e) => {
                match e {
                    FetchError::NotAnUrl => {
                        error!("the provided file was neither a valid URL nor a valid file: {}", &args.file);
                        return Err(e)?;
                    },
                    e => {
                        error!("{e}");
                        return Err(e)?;
                    }
                }
            },
        }
    };

    debug!("source image dimensions: {}x{}", image.width(), image.height());
    debug!("image color type: {:?}", image.color());

    let start = Instant::now();
    let (width, height) = match (args.width, args.height) {
        (None, None) => {
            let aspect_ratio = image.width() as f32 / image.height() as f32;
            let h = (64 as f32 / aspect_ratio).round() as u32;
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

    image = image.resize_exact(width, height, image::imageops::FilterType::Triangle);
    image = image.adjust_contrast(args.contrast); // for some reason this also affects the alpha channel???
    image = image.brighten(args.brighten);

    // this is just so i can make sure the output is right and the filters are working properly
    #[cfg(debug_assertions)]
    {
        let out_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/debug.png"));
        debug!("saving debug image to {}", out_dir.canonicalize().unwrap_or(out_dir.clone()).as_os_str().to_string_lossy());
        image.save(out_dir.clone()).unwrap();
    }

    let ditherer: &dyn Ditherer = match args.dithering {
        DitheringOption::Sierra2 => &dithering::Sierra2Row,
        DitheringOption::None => &dithering::None,
    };

    let braille = braille::BrailleImg::from_image(
        image,
        ditherer,
        !args.invert
    );

    println!("{}", braille.to_str(!args.allow_blank_chars, true));

    debug!("turned image into braille in {}s", start.elapsed().as_secs_f32());

    Ok(())
}
