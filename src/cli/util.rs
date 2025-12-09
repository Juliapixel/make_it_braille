use std::io::Cursor;

use image::{AnimationDecoder, ImageFormat};
use log::debug;
use reqwest::{header::HeaderMap, Url};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("the image doesn't have a frame of number {0}")]
    NoSuchFrame(u32),
    #[error(transparent)]
    InvalidImage(#[from] image::error::ImageError),
    #[error("there as an I/O error!")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Fetch(#[from] FetchError),
}

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("the provided URL was not valid")]
    BadUrl,
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error("the server's response was bad")]
    BadResponse,
    #[error(transparent)]
    BadImageData(#[from] image::error::ImageError),
}

pub fn try_get_from_url(url: Url) -> Result<image::Frames<'static>, Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Accept",
        "image/png,image/jpeg,image/webp,image/gif,image/tiff"
            .parse()
            .unwrap(),
    );
    headers.insert(
        "Referer",
        url.host_str().ok_or(FetchError::BadUrl)?.parse().unwrap(),
    );
    headers.insert("Cache-Control", "no-cache".parse().unwrap());
    headers.insert(
        "User-Agent",
        format!("{}/{}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"))
            .parse()
            .unwrap(),
    );

    let client = reqwest::blocking::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    let resp = client
        .get(url)
        .send()
        .map_err(FetchError::RequestError)?
        .error_for_status()
        .map_err(FetchError::RequestError)?;

    debug!("response info: {:#?}", &resp);

    let format_header = resp.headers().get(reqwest::header::CONTENT_TYPE).cloned();
    let payload = resp.bytes().map_err(|_| FetchError::BadResponse)?;

    let format = format_header.and_then(|h| image::ImageFormat::from_mime_type(h.to_str().ok()?));

    load_as_frames(payload.clone(), format)
}

pub fn load_as_frames(
    data: impl AsRef<[u8]> + 'static,
    format: Option<ImageFormat>,
) -> Result<image::Frames<'static>, Error> {
    match format {
        Some(ImageFormat::Gif) => {
            let decoder = image::codecs::gif::GifDecoder::new(Cursor::new(data))?;
            Ok(decoder.into_frames())
        }
        Some(ImageFormat::WebP) => {
            let decoder = image::codecs::webp::WebPDecoder::new(Cursor::new(data))?;
            Ok(decoder.into_frames())
        }
        Some(f) => {
            let frame = image::load_from_memory_with_format(data.as_ref(), f)?;
            Ok(image::Frames::new(Box::new(
                [Ok(image::Frame::new(frame.into_rgba8()))].into_iter(),
            )))
        }
        None => {
            let guessed = image::guess_format(data.as_ref())?;
            load_as_frames(data, Some(guessed))
        }
    }
}
