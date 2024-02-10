//! this crate provides methods to create, manipulate and output images made out
//! of unicode braille characters such as `⠝`.
//!
//! # Basic Usage
//! ```rust no_run
//! use image::imageops::FilterType;
//! use make_it_braille::{BrailleImg, dithering::Sierra2Row};
//!
//! let mut img = image::open("image.png").unwrap();
//! img = img.resize_exact(64, 64, FilterType::Triangle);
//!
//! let mut img = BrailleImg::from_image(img, Sierra2Row, false);
//!
//! println!("{}", img.as_str(true, true));
//! ```
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod braille;

#[cfg(feature = "image")]
pub mod dithering;

pub use braille::{BrailleImg, Error};
#[cfg(feature = "image")]
pub use dithering::{Bayer2x2, Bayer4x4, None, Sierra2Row};
