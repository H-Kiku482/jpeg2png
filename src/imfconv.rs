use std::{
    error::Error,
    path::{Path, PathBuf},
};

use self::{
    handler::{
        color_profile::{
            grayscale::Grayscale, rgb::RgbColor, rgba::RgbaColor, ImfconvColorProfile,
        },
        format::{jpeg::JpegHandler, png::PngHandler, tiff::TiffHandler, ImfconvHandler},
    },
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
    color: Box<dyn ImfconvColorProfile>,
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
            color: Box::new(RgbaColor),
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
            color: self.color,
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
            color: self.color,
            dest_path: self.dest_path,
        })
    }

    pub fn set_color_profile(self, color_profile: ColorProfile) -> Result<Self, Box<dyn Error>> {
        match color_profile {
            ColorProfile::RGBA => {
                return Ok(Self {
                    image: self.image,
                    w: self.w,
                    h: self.h,
                    grayscale: self.grayscale,
                    format: self.format,
                    color: Box::new(RgbaColor),
                    dest_path: self.dest_path,
                })
            }
            ColorProfile::RGB => {
                return Ok(Self {
                    image: self.image,
                    w: self.w,
                    h: self.h,
                    grayscale: self.grayscale,
                    format: self.format,
                    color: Box::new(RgbColor),
                    dest_path: self.dest_path,
                })
            }
            ColorProfile::GRAYSCALE => {
                return Ok(Self {
                    image: self.image,
                    w: self.w,
                    h: self.h,
                    grayscale: self.grayscale,
                    format: self.format,
                    color: Box::new(Grayscale),
                    dest_path: self.dest_path,
                })
            }
        }
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

pub enum ColorProfile {
    RGBA,
    RGB,
    GRAYSCALE,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jpg_to_png() {
        match Imfconv::new(Path::new("test/1.jpeg"), Path::new("test/result")) {
            Ok(img) => img,
            Err(e) => panic!("{}", e),
        };
    }
}
