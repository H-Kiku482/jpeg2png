use std::{error::Error, path::Path};

pub fn read_image(src: &Path) -> Result<(u32, u32, Vec<u8>), Box<dyn Error>> {
    match image::open(src) {
        Ok(i) => return Ok((i.width(), i.height(), i.into_bytes())),
        Err(e) => return Err(Box::new(e)),
    };
}

#[cfg(test)]
mod test {
    use std::path::Path;

    #[test]
    fn read_image_test() {
        let src = Path::new("test/1.jpeg");
        match image::open(src) {
            Ok(i) => (i.width(), i.height(), i.into_bytes()),
            Err(e) => panic!("{}", e),
        };
    }
}
