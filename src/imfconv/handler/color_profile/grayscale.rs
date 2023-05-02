use std::{error::Error, fmt::Error as FmtError};

use super::ImfconvColorProfile;

pub struct Grayscale;

impl ImfconvColorProfile for Grayscale {
    fn edit(&self, width: u32, height: u32, raw_image: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let d = match image::RgbImage::from_vec(width, height, raw_image.to_vec()) {
            Some(i) => image::DynamicImage::from(i),
            None => return Err(Box::new(FmtError)),
        };
        let d = d.grayscale().into_rgb8();
        return Ok(d.to_vec());
    }
}
