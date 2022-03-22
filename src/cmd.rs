use image::io::Reader as ImageReader;
use image::ImageFormat;
use rand::Rng;
use std::path::Path;

pub fn failed_message(source: &Path) {
    println!(
        "\x1b[31mfailed\x1b[0m: {}",
        match source.to_str() {
            Some(s) => s,
            None => "unknown file",
        }
    );
}

pub fn get_random_string() -> String {
    let mut s = String::new();
    let mut rng = rand::thread_rng();
    s.push_str(&format!("{:x}", rng.gen::<u32>()));
    s.push_str(&format!("{:x}", rng.gen::<u32>()));
    s
}

pub fn duplicate(src: &Path, dest: &Path) -> bool {
    let source_image = match ImageReader::open(src) {
        Ok(r) => match r.with_guessed_format() {
            Ok(i) => i,
            Err(_) => {
                return false;
            }
        },
        Err(_) => {
            return false;
        }
    };

    let img_format = match source_image.format() {
        Some(f) => f,
        None => {
            return false;
        }
    };

    if img_format != ImageFormat::Jpeg {
        return false;
    }

    let decoded = match source_image.decode() {
        Ok(d) => d,
        Err(_) => {
            return false;
        }
    };

    match decoded.save(dest) {
        Ok(_) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    }
}
