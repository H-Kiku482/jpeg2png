use std::error::Error;

pub mod grayscale;
pub mod rgb;
pub mod rgba;

pub trait ImfconvColorProfile {
    fn edit(&self, width: u32, height: u32, raw_image: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
}
