use std::{
    error::Error,
    path::{Path, PathBuf},
};

use self::{
    handler::format::{jpeg::JpegHandler, png::PngHandler, tiff::TiffHandler, ImfconvHandler},
    reader::read_image,
};

mod handler;
mod reader;

pub struct Imfconv {
    image: Vec<u8>,
    w: u32,
    h: u32,
    grayscale: bool,
    format: Box<dyn ImfconvHandler>,
    dest_path: PathBuf,
}

impl Imfconv {
    pub fn new(
        source_image_filepath: &Path,
        destination_file_stem: &Path,
    ) -> Result<Self, Box<dyn Error>> {
        let (w, h, i) = match read_image(source_image_filepath) {
            Ok((w, h, i)) => (w, h, i),
            Err(e) => return Err(e),
        };
        Ok(Self {
            image: i,
            w,
            h,
            grayscale: false,
            format: Box::new(PngHandler),
            dest_path: PathBuf::from(destination_file_stem),
        })
    }

    pub fn set_image_format(self, image_type: ImageType) -> Self {
        let f: Box<dyn ImfconvHandler> = match image_type {
            ImageType::JPEG => Box::new(JpegHandler),
            ImageType::PNG => Box::new(PngHandler),
            ImageType::TIFF => Box::new(TiffHandler),
        };

        Self {
            image: self.image,
            w: self.w,
            h: self.h,
            grayscale: self.grayscale,
            format: f,
            dest_path: self.dest_path,
        }
    }

    pub fn set_grayscale(self, grayscale: bool) -> Result<Self, Box<dyn Error>> {
        if self.grayscale == grayscale {
            return Ok(self);
        }

        let img = image::load_from_memory(&self.image);
        let img = match img {
            Ok(i) => i.grayscale(),
            Err(e) => return Err(Box::new(e)),
        };

        Ok(Self {
            image: img.into_bytes(),
            w: self.w,
            h: self.h,
            grayscale,
            format: self.format,
            dest_path: self.dest_path,
        })
    }

    pub fn convert(&self) -> Result<(), Box<dyn Error>> {
        match self
            .format
            .exec(self.w, self.h, &self.image, &self.dest_path)
        {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        };
    }
}

pub enum ImageType {
    JPEG,
    PNG,
    TIFF,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jpg_to_png() {
        let image_builder = match Imfconv::new(Path::new("test/1.jpeg"), Path::new("test/result")) {
            Ok(img) => img,
            Err(e) => panic!("{}", e),
        };
    }
}
