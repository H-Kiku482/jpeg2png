use std::{error::Error, fmt::Error as FmtError};

use super::ImfconvColorProfile;

pub struct RgbaColor;

impl ImfconvColorProfile for RgbaColor {
    fn edit(&self, width: u32, height: u32, raw_image: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let raw_image = match image::RgbImage::from_vec(width, height, raw_image.to_vec()) {
            Some(i) => image::DynamicImage::from(i),
            None => return Err(Box::new(FmtError)),
        };
        let rgb = match raw_image.as_rgba8() {
            Some(i) => i,
            None => return Err(Box::new(FmtError)),
        };
        return Ok(rgb.to_vec());
    }
}
