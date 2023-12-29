use image::DynamicImage;

use crate::dithering::Ditherer;

const BRAILLE_CHARS: [char; 256] = [
'⠀', '⠁', '⠂', '⠃', '⠄', '⠅', '⠆', '⠇', '⠈', '⠉', '⠊', '⠋', '⠌', '⠍', '⠎', '⠏', '⠐', '⠑', '⠒', '⠓', '⠔', '⠕', '⠖', '⠗', '⠘', '⠙',
'⠚', '⠛', '⠜', '⠝', '⠞', '⠟', '⠠', '⠡', '⠢', '⠣', '⠤', '⠥', '⠦', '⠧', '⠨', '⠩', '⠪', '⠫', '⠬', '⠭', '⠮', '⠯', '⠰', '⠱', '⠲',
'⠳', '⠴', '⠵', '⠶', '⠷', '⠸', '⠹', '⠺', '⠻', '⠼', '⠽', '⠾', '⠿', '⡀', '⡁', '⡂', '⡃', '⡄', '⡅', '⡆', '⡇', '⡈', '⡉', '⡊', '⡋',
'⡌', '⡍', '⡎', '⡏', '⡐', '⡑', '⡒', '⡓', '⡔', '⡕', '⡖', '⡗', '⡘', '⡙', '⡚', '⡛', '⡜', '⡝', '⡞', '⡟', '⡠', '⡡', '⡢', '⡣', '⡤',
'⡥', '⡦', '⡧', '⡨', '⡩', '⡪', '⡫', '⡬', '⡭', '⡮', '⡯', '⡰', '⡱', '⡲', '⡳', '⡴', '⡵', '⡶', '⡷', '⡸', '⡹', '⡺', '⡻', '⡼', '⡽',
'⡾', '⡿', '⢀', '⢁', '⢂', '⢃', '⢄', '⢅', '⢆', '⢇', '⢈', '⢉', '⢊', '⢋', '⢌', '⢍', '⢎', '⢏', '⢐', '⢑', '⢒', '⢓', '⢔', '⢕', '⢖',
'⢗', '⢘', '⢙', '⢚', '⢛', '⢜', '⢝', '⢞', '⢟', '⢠', '⢡', '⢢', '⢣', '⢤', '⢥', '⢦', '⢧', '⢨', '⢩', '⢪', '⢫', '⢬', '⢭', '⢮', '⢯',
'⢰', '⢱', '⢲', '⢳', '⢴', '⢵', '⢶', '⢷', '⢸', '⢹', '⢺', '⢻', '⢼', '⢽', '⢾', '⢿', '⣀', '⣁', '⣂', '⣃', '⣄', '⣅', '⣆', '⣇', '⣈',
'⣉', '⣊', '⣋', '⣌', '⣍', '⣎', '⣏', '⣐', '⣑', '⣒', '⣓', '⣔', '⣕', '⣖', '⣗', '⣘', '⣙', '⣚', '⣛', '⣜', '⣝', '⣞', '⣟', '⣠', '⣡',
'⣢', '⣣', '⣤', '⣥', '⣦', '⣧', '⣨', '⣩', '⣪', '⣫', '⣬', '⣭', '⣮', '⣯', '⣰', '⣱', '⣲', '⣳', '⣴', '⣵', '⣶', '⣷', '⣸', '⣹', '⣺',
'⣻', '⣼', '⣽', '⣾', '⣿'
];

pub struct BrailleImg {
    braille_vals: Vec<u8>,
    dot_width: u32,
    dot_height: u32,
    char_width: u32,
    char_height: u32,
}

impl BrailleImg {
    pub fn new(width: u32, height: u32) -> Self {
        let mut vals: Vec<u8> = Vec::new();
        let x_size = width / 2 + (width % 2);
        let extra_row = if height % 4 != 0 {
            1
        } else {
            0
        };
        let y_size = height / 4 + extra_row;
        vals.reserve((x_size * y_size) as usize);
        for _ in 0..x_size * y_size {
            vals.push(0);
        }
        BrailleImg {
            braille_vals: vals,
            dot_width: width,
            dot_height: height,
            char_width: x_size,
            char_height: y_size,
        }
    }

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

    pub fn set_dot(&mut self, x: u32, y: u32, raised: bool) {
        let x_val_pos = x / 2;
        let y_val_pos = y / 4;
        let val = self.braille_vals.get_mut((x_val_pos + y_val_pos * self.char_width) as usize).unwrap();
        let mask = BrailleImg::get_bit_mask(x, y);
        if raised {
            *val |= mask;
        } else {
            *val &= !mask;
        }
    }

    pub fn to_str(self, no_empty_chars: bool, break_line: bool) -> String {
        let mut braille_string = String::new();
        for (i, val) in self.braille_vals.into_iter().enumerate() {
            if i % self.char_width as usize == 0 && i != 0 {
                braille_string.push(if break_line { '\n' } else { ' ' });
            }
            if val == 0 && no_empty_chars {
                braille_string.push(BRAILLE_CHARS[1])
            } else {
                braille_string.push(BRAILLE_CHARS[val as usize])
            }
        }
        return braille_string
    }

    // TODO: remove the intermediary GrayImage completely, as it's not needed anymore.
    pub fn from_image(img: DynamicImage, ditherer: &dyn Ditherer, invert: bool) -> Self {
        let mut gray_img = image::GrayImage::new(img.width(), img.height());
        let img = img.into_rgba8();

        let compute_lightness = |rgba: &[f32; 4]| -> u8 {
            return ((rgba[0] * 0.2126 + rgba[1] * 0.7152 + rgba[2] * 0.0722) * (rgba[3] / 255.0))
              .clamp(0.0, 255.0)
              .round() as u8;
        };

        for (x, y, pix) in img.enumerate_pixels() {
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
        for (x, y, pix) in gray_img.enumerate_pixels() {
            if invert {
                if pix.0[0] > 96 {
                    braille_img.set_dot(x, y, true);
                }
            } else {
                if pix.0[0] < 96 {
                    braille_img.set_dot(x, y, true);
                }
            }
        }
        return braille_img;
    }
}
