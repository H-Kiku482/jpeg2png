use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use imfconv::imfconv::Imfconv;
use rand::Rng;
use walkdir::WalkDir;

use super::CliImfconv;

pub struct AsDirectory;

impl AsDirectory {
    fn generate_random_dirname(&self, src: &str) -> PathBuf {
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

impl CliImfconv for AsDirectory {
    fn exec(
        &self,
        src: &str,
        dest: &str,
        format: &imfconv::imfconv::ImageType,
        profile: &imfconv::imfconv::ColorProfile,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let src_path_root = Path::new(src);

        let origin = if dest == "" {
            Path::new(src)
        } else {
            Path::new(dest)
        };

        let dest_path_root = match origin.try_exists() {
            Ok(exists) => match exists {
                true => self.generate_random_dirname(src),
                false => origin.to_path_buf(),
            },
            Err(_) => origin.to_path_buf(),
        };

        match create_dir_all(&dest_path_root) {
            Ok(_) => {}
            Err(_) => todo!(),
        };

        for entry in WalkDir::new(src) {
            let source_path = match entry {
                Ok(p) => p,
                Err(_) => todo!(),
            };
            let source_path = source_path.path();

            if !source_path.is_file() {
                continue;
            }

            let related_source_image_path = match Path::new(source_path).strip_prefix(src_path_root)
            {
                Ok(p) => p,
                Err(_) => todo!(),
            };

            let dest_path = Path::new(&dest_path_root)
                .join(&related_source_image_path)
                .with_extension("");

            println!("{:?}", source_path);
            println!("{:?}", dest_path);
            let handler = match Imfconv::new(&source_path, &dest_path.as_path()) {
                Ok(h) => match h.set_image_format(format).set_color_profile(profile) {
                    Ok(h) => h,
                    Err(_) => todo!(),
                },
                Err(_) => todo!(),
            };
            match handler.convert() {
                Ok(_) => continue,
                Err(_) => todo!(),
            };
        }
        Ok(())
    }
}
