use std::{error::Error, path::Path};

use clap::{Arg, Command};
use imfconv::imfconv::{ColorProfile, ImageType};

mod directory;
mod file;

const APP_NAME: &str = "imfconv";
const VERSION: &str = "3.0.0";
const AUTHOR: &str = "hkiku482 <h.kikuchi482@gmail.com>";
const ABOUT: &str = "Convert image format.";

trait CliImfconv {
    /// Image format conversion for file or directory.
    /// - src: source file or directory path
    /// - dest: destination file or directory path
    /// - format: image format after conversion
    /// - profile: image profile after conversion
    fn exec(
        &self,
        src: &str,
        dest: &str,
        format: &imfconv::imfconv::ImageType,
        profile: &imfconv::imfconv::ColorProfile,
    ) -> Result<(), Box<dyn Error>>;
}

struct OsArgsId;
impl OsArgsId {
    const SOURCE_ITEM_PATH: &str = "source-item-path";
    const FORMAT: &str = "format";
    const COLOR_PROFILE: &str = "color-profile";
    const OUTPUT_FILE_PATH: &str = "output-filename";
}

struct ImageFormat;
impl ImageFormat {
    const JPEG: &str = "jpg";
    const PNG: &str = "png";
    const TIFF: &str = "tif";
}

struct ImageColorProfile;
impl ImageColorProfile {
    const GRAYSCALE: &str = "grayscale";
    const RGB: &str = "rgb";
}

/// execute imfconv for cli
pub fn run() {
    let command = Command::new(APP_NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(
            Arg::new(OsArgsId::SOURCE_ITEM_PATH)
                .help("The source image file or directory.")
                .num_args(1..)
                .required(true),
        )
        .arg(
            Arg::new(OsArgsId::FORMAT)
                .long("format")
                .short('f')
                .help("Output file format.")
                .ignore_case(true)
                .value_parser([ImageFormat::JPEG, ImageFormat::PNG, ImageFormat::TIFF])
                .default_value(ImageFormat::PNG),
        )
        .arg(
            Arg::new(OsArgsId::COLOR_PROFILE)
                .long("color-profile")
                .short('c')
                .help("Image color profile.")
                .ignore_case(true)
                .value_parser([ImageColorProfile::GRAYSCALE, ImageColorProfile::RGB])
                .default_value(ImageColorProfile::RGB),
        )
        .arg(
            Arg::new(OsArgsId::OUTPUT_FILE_PATH)
                .long("output-item-name")
                .short('o')
                .help("Output file or directory name.")
                .required(false),
        )
        .get_matches();

    let source_pathes: Vec<&String> = match command.get_many::<String>(OsArgsId::SOURCE_ITEM_PATH) {
        Some(values) => values.collect(),
        None => return,
    };

    let extension = match command.get_one::<String>(OsArgsId::FORMAT) {
        Some(item) => item,
        None => ImageFormat::PNG,
    };

    let format = match extension {
        ImageFormat::JPEG => ImageType::JPEG,
        ImageFormat::TIFF => ImageType::TIFF,
        _ => ImageType::PNG,
    };

    let profile = match command.get_one::<String>(OsArgsId::COLOR_PROFILE) {
        Some(p) => match p.as_str() {
            ImageColorProfile::GRAYSCALE => ColorProfile::GRAYSCALE,
            ImageColorProfile::RGB => ColorProfile::RGB,
            _ => ColorProfile::RGB,
        },
        None => ColorProfile::RGB,
    };

    for source_path in source_pathes {
        let output: &str = match command.get_one::<String>(OsArgsId::OUTPUT_FILE_PATH) {
            Some(item) => item,
            None => "",
        };

        let (handler, output_path): (Box<dyn CliImfconv>, String) =
            if Path::new(source_path).is_dir() {
                match output {
                    "" => {
                        let mut output_dir = String::from(source_path);
                        output_dir.push_str("_");
                        output_dir.push_str(&extension);
                        let p = Path::new(&output_dir);
                        let p = p.to_str();
                        match p {
                            Some(s) => (Box::new(directory::AsDirectory), String::from(s)),
                            None => (Box::new(directory::AsDirectory), String::new()),
                        }
                    }
                    _ => (Box::new(directory::AsDirectory), String::new()),
                }
            } else {
                match output {
                    "" => {
                        let p = Path::new(&source_path).with_extension(&extension);
                        let p = p.to_str();
                        match p {
                            Some(s) => (Box::new(file::AsFile), String::from(s)),
                            None => (Box::new(file::AsFile), String::new()),
                        }
                    }
                    _ => (Box::new(file::AsFile), String::new()),
                }
            };

        match handler.exec(source_path, &output_path, &format, &profile) {
            Ok(_) => continue,
            Err(e) => eprint!("{}\n", e),
        };
    }
}
