use std::path::{Path, PathBuf};

use super::CliImfconv;
use imfconv::imfconv;
use rand::Rng;

pub struct AsFile;
impl AsFile {
    fn generate_random_filename(&self, src: &str) -> PathBuf {
        let mut rng = rand::thread_rng();
        let random: String = rng.gen::<i32>().to_string();

        let parent_path = match Path::new(src).parent() {
            Some(p) => p,
            None => todo!(),
        };

        let mut dest_file_stem = match Path::new(src).file_stem() {
            Some(s) => s.to_os_string(),
            None => todo!(),
        };

        dest_file_stem.push(String::from("-") + &random);

        Path::new(parent_path).join(&dest_file_stem)
    }
}

impl CliImfconv for AsFile {
    fn exec(
        &self,
        src: &str,
        dest: &str,
        format: &imfconv::ImageType,
        profile: &imfconv::ColorProfile,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let src_path = Path::new(src);

        // Auto decision destination path by source path.
        let dst_path = if dest == "" {
            Path::new(src)
        } else {
            Path::new(dest)
        };

        let dst_path = match dst_path.try_exists() {
            Ok(exists) => match exists {
                true => {
                    // Exists.
                    self.generate_random_filename(src)
                }
                false => {
                    // Not exists.
                    dst_path.to_path_buf()
                }
            },
            Err(_) => dst_path.to_path_buf(),
        };
        let builder = match imfconv::Imfconv::new(src_path, &dst_path) {
            Ok(b) => b,
            Err(e) => return Err(e),
        };
        let builder = builder.set_image_format(format);
        let builder = match builder.set_color_profile(profile) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        builder.convert()
    }
}
