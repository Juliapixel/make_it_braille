use std::{ops::Deref, sync::OnceLock};

use image::GrayImage;

pub trait Ditherer {
    fn dither(&self, buffer: &mut GrayImage);
}

impl<T> Ditherer for T where T: Deref<Target = dyn Ditherer> {
    fn dither(&self, buffer: &mut GrayImage) {
        self.deref().dither(buffer)
    }
}

fn _srgb_to_linear(val: u8) -> u8 {
    static LUT: OnceLock<[u8; 256]> = OnceLock::new();

    LUT.get_or_init(|| {
        std::array::from_fn(|i| {
            ((i as f32 / 255.0).powf(1.0/2.2) * 255.0) as u8
        })
    })[val as usize]
}

/// the Sierra two-row error-difusion dithering algorithm
pub struct Sierra2Row;

impl Ditherer for Sierra2Row {
    fn dither(&self, buffer: &mut GrayImage) {
        let add_error = |img: &mut image::GrayImage, x: Option<u32>, y: Option<u32>, err: i32, importance: i32| {
            if let Some(xpos) = x {
                if let Some(ypos) = y {
                    if let Some(pix) = img.get_pixel_mut_checked(xpos, ypos) {
                        *pix = image::Luma([(pix.0[0] as i32 + err * importance).clamp(0, 255) as u8]);
                    }
                }
            }
        };

        for y in 0..buffer.height() {
            for x in 0..buffer.width() {
                let cur_pix = buffer.get_pixel_mut(x, y);
                let error = if cur_pix.0[0] > 96 {
                    cur_pix.0[0] as i32 - 255
                } else {
                    cur_pix.0[0] as i32
                } >> 5;
                if cur_pix.0[0] > 96 {
                    cur_pix.0[0] = 255;
                } else {
                    cur_pix.0[0] = 0;
                }

                add_error(buffer, x.checked_add(1), Some(y)    , error, 5);
                add_error(buffer, x.checked_add(2), Some(y)    , error, 3);
                add_error(buffer, x.checked_sub(2), Some(y + 1), error, 2);
                add_error(buffer, x.checked_sub(1), Some(y + 1), error, 4);
                add_error(buffer, Some(x)         , Some(y + 1), error, 5);
                add_error(buffer, x.checked_add(1), Some(y + 1), error, 4);
                add_error(buffer, x.checked_add(2), Some(y + 1), error, 2);
                add_error(buffer, x.checked_sub(1), Some(y + 2), error, 2);
                add_error(buffer, Some(x)         , Some(y + 2), error, 3);
                add_error(buffer, x.checked_add(1), Some(y + 2), error, 2);
            }
        }
    }
}

/// the Bayer ordered dithering algorithm, with a 4x4 matrix
pub struct Bayer4x4;

const BAYER4X4_MATRIX: [[u8; 4]; 4] = [
    [0  , 128, 32 , 160],
    [192, 64 , 224, 96 ],
    [48 , 176, 16 , 144],
    [240, 112, 208, 80 ]
];

impl Ditherer for Bayer4x4 {
    fn dither(&self, buffer: &mut GrayImage) {
        for (x, y, pix) in buffer.enumerate_pixels_mut() {
            if pix.0[0] > BAYER4X4_MATRIX[(y % 4) as usize][(x % 4) as usize] {
                pix.0[0] = 255;
            } else {
                pix.0[0] = 0;
            }
        }
    }
}

/// the Bayer ordered dithering algorithm, with a 4x4 matrix
pub struct Bayer2x2;

const BAYER2X2_MATRIX: [[u8; 2]; 2] = [
    [0  , 128],
    [192, 64 ]
];

impl Ditherer for Bayer2x2 {
    fn dither(&self, buffer: &mut GrayImage) {
        for (x, y, pix) in buffer.enumerate_pixels_mut() {
            if pix.0[0] > BAYER2X2_MATRIX[(y % 2) as usize][(x % 2) as usize] {
                pix.0[0] = 255;
            } else {
                pix.0[0] = 0;
            }
        }
    }
}

/// No dithering, raises dots over 96
pub struct None;

impl Ditherer for None {
    fn dither(&self, buffer: &mut GrayImage) {
        for pix in buffer.pixels_mut() {
            if pix.0[0] > 96 {
                pix.0[0] = 255;
            } else {
                pix.0[0] = 0;
            }
        }
    }
}
