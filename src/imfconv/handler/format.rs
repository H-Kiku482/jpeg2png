use std::{error::Error, path::Path};

pub mod jpeg;
pub mod png;
pub mod tiff;

pub trait ImfconvHandler {
    fn exec(
        &self,
        width: u32,
        height: u32,
        raw_image: &[u8],
        dest_filepath: &Path,
    ) -> Result<(), Box<dyn Error>>;
}
