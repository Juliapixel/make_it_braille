#[cfg(feature = "image")]
use crate::dithering::Ditherer;

/// this is just all 256 braille characters, with the raised dots meaning each
/// of the following bits, with 0 being the least significant:
///
/// ```text
/// 0  3
/// 1  4
/// 2  5
/// 6  7
/// ```
///
pub const BRAILLE_CHARS: [char; 256] = [
'⠀', '⠁', '⠂', '⠃', '⠄', '⠅', '⠆', '⠇',
'⠈', '⠉', '⠊', '⠋', '⠌', '⠍', '⠎', '⠏',
'⠐', '⠑', '⠒', '⠓', '⠔', '⠕', '⠖', '⠗',
'⠘', '⠙', '⠚', '⠛', '⠜', '⠝', '⠞', '⠟',
'⠠', '⠡', '⠢', '⠣', '⠤', '⠥', '⠦', '⠧',
'⠨', '⠩', '⠪', '⠫', '⠬', '⠭', '⠮', '⠯',
'⠰', '⠱', '⠲', '⠳', '⠴', '⠵', '⠶', '⠷',
'⠸', '⠹', '⠺', '⠻', '⠼', '⠽', '⠾', '⠿',
'⡀', '⡁', '⡂', '⡃', '⡄', '⡅', '⡆', '⡇',
'⡈', '⡉', '⡊', '⡋', '⡌', '⡍', '⡎', '⡏',
'⡐', '⡑', '⡒', '⡓', '⡔', '⡕', '⡖', '⡗',
'⡘', '⡙', '⡚', '⡛', '⡜', '⡝', '⡞', '⡟',
'⡠', '⡡', '⡢', '⡣', '⡤', '⡥', '⡦', '⡧',
'⡨', '⡩', '⡪', '⡫', '⡬', '⡭', '⡮', '⡯',
'⡰', '⡱', '⡲', '⡳', '⡴', '⡵', '⡶', '⡷',
'⡸', '⡹', '⡺', '⡻', '⡼', '⡽', '⡾', '⡿',
'⢀', '⢁', '⢂', '⢃', '⢄', '⢅', '⢆', '⢇',
'⢈', '⢉', '⢊', '⢋', '⢌', '⢍', '⢎', '⢏',
'⢐', '⢑', '⢒', '⢓', '⢔', '⢕', '⢖', '⢗',
'⢘', '⢙', '⢚', '⢛', '⢜', '⢝', '⢞', '⢟',
'⢠', '⢡', '⢢', '⢣', '⢤', '⢥', '⢦', '⢧',
'⢨', '⢩', '⢪', '⢫', '⢬', '⢭', '⢮', '⢯',
'⢰', '⢱', '⢲', '⢳', '⢴', '⢵', '⢶', '⢷',
'⢸', '⢹', '⢺', '⢻', '⢼', '⢽', '⢾', '⢿',
'⣀', '⣁', '⣂', '⣃', '⣄', '⣅', '⣆', '⣇',
'⣈', '⣉', '⣊', '⣋', '⣌', '⣍', '⣎', '⣏',
'⣐', '⣑', '⣒', '⣓', '⣔', '⣕', '⣖', '⣗',
'⣘', '⣙', '⣚', '⣛', '⣜', '⣝', '⣞', '⣟',
'⣠', '⣡', '⣢', '⣣', '⣤', '⣥', '⣦', '⣧',
'⣨', '⣩', '⣪', '⣫', '⣬', '⣭', '⣮', '⣯',
'⣰', '⣱', '⣲', '⣳', '⣴', '⣵', '⣶', '⣷',
'⣸', '⣹', '⣺', '⣻', '⣼', '⣽', '⣾', '⣿'
];

const BRAILLE_LEN: usize = BRAILLE_CHARS[0].len_utf8();

#[derive(Debug, Clone, Copy)]
pub enum Error {
    OutOfBounds(u32, u32, u32, u32),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::OutOfBounds(x, y, w, h) => {
                write!(f, "the coordinates (x: {x}, y: {y}) were outside the bounds of the BrailleImg (width: {w}, height: {h})")
            },
        }
    }
}

pub struct BrailleImg {
    braille_vals: Vec<u8>,
    dot_width: u32,
    dot_height: u32,
    char_width: u32,
    char_height: u32,
}

impl BrailleImg {
    /// create a new [BrailleImg] with `width` and `height` dimensions, in dots,
    /// where each character is 2 dots wide and 4 dots tall
    /// # Panics
    /// if either width or height is 0
    pub fn new(width: u32, height: u32) -> Self {
        assert!(width != 0 && height != 0, "width and height must be greater than 0");
        let x_size = width / 2 + (width % 2);
        let extra_row = if height % 4 != 0 {
            1
        } else {
            0
        };
        let y_size = height / 4 + extra_row;

        let vals = vec![0; (x_size * y_size) as usize];

        BrailleImg {
            braille_vals: vals,
            dot_width: width,
            dot_height: height,
            char_width: x_size,
            char_height: y_size,
        }
    }

    /// maps x and y coordinates to which bit will represent the dot on the
    /// character according to [BRAILLE_CHARS]
    fn get_bit_mask(x: u32, y: u32) -> u8 {
        if x % 2 == 0 {
            match y % 4 {
                0 => 0b00000001,
                1 => 0b00000010,
                2 => 0b00000100,
                _ => 0b01000000
            }
        } else {
            match y % 4 {
                0 => 0b00001000,
                1 => 0b00010000,
                2 => 0b00100000,
                _ => 0b10000000
            }
        }
    }

    pub fn set_dot(&mut self, x: u32, y: u32, raised: bool) -> Result<(), Error> {
        if x > (self.dot_width - 1) || y > (self.dot_height - 1) {
            return Err(Error::OutOfBounds(x, y, self.char_width, self.char_height))
        }
        let x_val_pos = x / 2;
        let y_val_pos = y / 4;
        // unwrapping here is safe since we already did a bounds check beforehand
        let val = self.braille_vals.get_mut((x_val_pos + y_val_pos * self.char_width) as usize).unwrap();
        let mask = BrailleImg::get_bit_mask(x, y);
        if raised {
            *val |= mask;
        } else {
            *val &= !mask;
        }
        Ok(())
    }

    pub fn get_dot(&self, x: u32, y: u32) -> Option<bool> {
        if x > (self.dot_width - 1) || y > (self.dot_height - 1) {
            return None;
        }
        let x_val_pos = x / 2;
        let y_val_pos = y / 4;
        // unwrapping here is safe since we already did a bounds check beforehand
        let val = self.braille_vals.get((x_val_pos + y_val_pos * self.char_width) as usize).unwrap();
        let mask = BrailleImg::get_bit_mask(x, y);
        Some(*val & mask != 0)
    }

    /// # Arguments
    /// - `no_empty chars` if true, empty braille characters will be replaced by
    ///   another char with a single dot raised, which avoids skewing of rows of
    ///   characters
    /// - `break_line` if true, each row of characters will be separated by a
    ///   newline character `\n`, otherwise they will be separated by a space
    #[deprecated = "you should use BrailleImg::as_str() instead"]
    pub fn to_str(self, no_empty_chars: bool, break_line: bool) -> String {
        let mut braille_string = String::new();
        for (i, val) in self.braille_vals.into_iter().enumerate() {
            if i % self.char_width as usize == 0 && i != 0 {
                braille_string.push(if break_line { '\n' } else { ' ' });
            }
            if val == 0 && no_empty_chars {
                braille_string.push(BRAILLE_CHARS[1 << 2])
            } else {
                braille_string.push(BRAILLE_CHARS[val as usize])
            }
        }
        braille_string
    }

    /// # Arguments
    /// - `no_empty chars` if true, empty braille characters will be replaced by
    ///   another char with a single dot raised, which avoids skewing of rows of
    ///   characters
    /// - `break_line` if true, each row of characters will be separated by a
    ///   newline character `\n`, otherwise they will be separated by a space
    pub fn as_str(&self, no_empty_chars: bool, break_line: bool) -> String {
        let mut braille_string = String::with_capacity(self.str_len());
        for (i, val) in self.braille_vals.iter().enumerate() {
            if i % self.char_width as usize == 0 && i != 0 {
                braille_string.push(if break_line { '\n' } else { ' ' });
            }
            if *val == 0 && no_empty_chars {
                braille_string.push(BRAILLE_CHARS[1 << 2])
            } else {
                braille_string.push(BRAILLE_CHARS[*val as usize])
            }
        }
        braille_string
    }

    fn str_len(&self) -> usize {
        ((self.char_width * self.char_height) as usize * BRAILLE_LEN) + (self.char_height - 1) as usize
    }

    #[cfg(feature = "image")]
    pub fn from_image(img: impl image::GenericImageView<Pixel=image::Rgba<u8>>, ditherer: impl Ditherer, invert: bool) -> Self {
        let mut gray_img = image::GrayImage::new(img.width(), img.height());

        let compute_lightness = |rgba: &[f32; 4]| -> u8 {
            ((rgba[0] * 0.2126 + rgba[1] * 0.7152 + rgba[2] * 0.0722) * (rgba[3] / 255.0))
              .clamp(0.0, 255.0)
              .round() as u8
        };

        for (x, y, pix) in img.pixels() {
            let lightness = compute_lightness(
                &[
                    pix.0[0] as f32,
                    pix.0[1] as f32,
                    pix.0[2] as f32,
                    pix.0[3] as f32
                ]
            );
            gray_img.put_pixel(x, y, image::Luma::<u8>([lightness]));
        }

        ditherer.dither(&mut gray_img);

        let mut braille_img = BrailleImg::new(gray_img.width(), gray_img.height());
        // this is fine since the dimensions of gray_img are always the same as braille_img's
        #[allow(unused_must_use)]
        for (x, y, pix) in gray_img.enumerate_pixels() {
            if invert {
                if pix.0[0] > 96 {
                    braille_img.set_dot(x, y, true);
                }
            } else if pix.0[0] < 96 {
                braille_img.set_dot(x, y, true);
            }
        }
        braille_img
    }
}

#[cfg(test)]
mod tests {
    use crate::braille::BrailleImg;

    #[test]
    fn str_len() {
        let img = BrailleImg::new(63, 21);
        let string_form = img.as_str(true, true);
        assert_eq!(string_form.len(), string_form.capacity())
    }

    #[test]
    fn bounds_check() {
        let mut img = BrailleImg::new(32, 32);

        assert!(img.set_dot(0, 0, true).is_ok());
        assert!(img.set_dot(1, 1, true).is_ok());
        assert!(img.set_dot(31, 31, true).is_ok());
        assert!(img.set_dot(32, 31, true).is_err());
        assert!(img.set_dot(31, 32, true).is_err());

        assert!(img.get_dot(0, 0).is_some());
        assert!(img.get_dot(1, 1).is_some());
        assert!(img.get_dot(31, 31).is_some());
        assert!(img.get_dot(32, 31).is_none());
        assert!(img.get_dot(31, 32).is_none());
    }

    #[test]
    fn get_dot() {
        let mut img = BrailleImg::new(4, 4);

        assert_eq!(img.get_dot(0, 0), Some(false));
        img.set_dot(0, 0, true).unwrap();
        assert_eq!(img.get_dot(0, 0), Some(true));
    }

    #[test]
    #[should_panic]
    fn new_null_width() {
        let _img = BrailleImg::new(0, 1);
    }

    #[test]
    #[should_panic]
    fn new_null_height() {
        let _img = BrailleImg::new(1, 0);
    }

    #[test]
    #[should_panic]
    fn new_null_both() {
        let _img = BrailleImg::new(0, 0);
    }
}
