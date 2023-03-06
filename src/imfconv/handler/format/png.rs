use super::ImfconvHandler;
use std::{error::Error, fmt::Error as FmtError, path::Path};

pub struct PngHandler;
impl ImfconvHandler for PngHandler {
    fn exec(
        &self,
        width: u32,
        height: u32,
        raw_image: &[u8],
        dest_filepath: &Path,
    ) -> Result<(), Box<dyn Error>> {
        let raw_image = match image::RgbImage::from_vec(width, height, raw_image.to_vec()) {
            Some(i) => i,
            None => return Err(Box::new(FmtError)),
        };

        let dest_filepath = dest_filepath.with_extension("png");
        let decoded_image = image::DynamicImage::from(raw_image);
        match decoded_image.save(&dest_filepath) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(Box::new(e)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::PngHandler;
    use crate::imfconv::handler::format::ImfconvHandler;
    use std::path::Path;

    #[test]
    fn exec_test() {
        let image = match image::open("test/1.jpeg") {
            Ok(i) => i,
            Err(e) => panic!("{}", e),
        };

        let dest_filepath = Path::new("test/result.png");

        let handler = Box::new(PngHandler);
        match handler.exec(
            image.width(),
            image.height(),
            &image.into_bytes(),
            dest_filepath,
        ) {
            Ok(_) => return,
            Err(e) => panic!("{}", e),
        };
    }
}
