use image::GrayImage;

pub trait Ditherer {
    fn dither(&self, buffer: &mut GrayImage);
}

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
                let error = if cur_pix.0[0] > 127 {
                    cur_pix.0[0] as i32 - 255
                } else {
                    cur_pix.0[0] as i32
                } >> 5;
                if cur_pix.0[0] > 127 {
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

pub struct None;

impl Ditherer for None {
    fn dither(&self, buffer: &mut GrayImage) {
        for pix in buffer.pixels_mut() {
            if pix.0[0] > 127 {
                pix.0[0] = 255;
            } else {
                pix.0[0] = 0;
            }
        }
    }
}
