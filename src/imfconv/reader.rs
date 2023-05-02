use std::{error::Error, path::Path};

use image::io::Reader;

/// Read the image located at `src` specified.
///
/// The image format is interpreted from memory block of the image.
///
/// Returns `Ok(width: u32, height: u32, image: Vec<u8>)`
pub fn read_image(src: &Path) -> Result<(u32, u32, Vec<u8>), Box<dyn Error>> {
    let reader = match Reader::open(src) {
        Ok(r) => match r.with_guessed_format() {
            Ok(r) => r,
            Err(e) => return Err(Box::new(e)),
        },
        Err(e) => return Err(Box::new(e)),
    };
    match reader.decode() {
        Ok(d) => return Ok((d.width(), d.height(), Vec::from(d.as_bytes()))),
        Err(e) => panic!("{:?}", e),
    };
}

#[cfg(test)]
mod test {
    use std::path::Path;

    #[test]
    fn read_image_test() {
        let src = Path::new("test/jpg.txt");
        match image::open(src) {
            Ok(i) => (i.width(), i.height(), i.into_bytes()),
            Err(e) => panic!("{}", e),
        };
    }
}
