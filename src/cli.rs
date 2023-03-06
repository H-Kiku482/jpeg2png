use std::{error::Error, path::Path};

use clap::{error::ErrorKind, Arg, Command};
use imfconv::imfconv::{ColorProfile, ImageType};

mod directory;
mod file;

const APP_NAME: &str = "imfconv";
const VERSION: &str = "3.0.0";
const AUTHOR: &str = "hkiku482 <h.kikuchi482@gmail.com>";
const ABOUT: &str = "Convert image format.";

trait CliImfconv {
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
    const RGBA: &str = "rgba";
}

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
                .value_parser([
                    ImageColorProfile::GRAYSCALE,
                    ImageColorProfile::RGB,
                    ImageColorProfile::RGBA,
                ])
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

    // Parse os arguments.
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
            ImageColorProfile::RGBA => ColorProfile::RGBA,
            _ => ColorProfile::RGB,
        },
        None => ColorProfile::RGB,
    };

    for source_path in source_pathes {
        let output_path: &str = match command.get_one::<String>(OsArgsId::OUTPUT_FILE_PATH) {
            Some(item) => item,
            None => "",
        };

        let output_path = match output_path {
            "" => {
                let p = Path::new(&source_path).with_extension(&extension);
                let p = p.to_str();
                match p {
                    Some(s) => String::from(s),

                    None => String::new(),
                }
            }
            _ => String::new(),
        };

        let handler: Box<dyn CliImfconv> = if Path::new(source_path).is_dir() {
            Box::new(directory::AsDirectory)
        } else {
            Box::new(file::AsFile)
        };

        match handler.exec(source_path, &output_path, &format, &profile) {
            Ok(_) => continue,
            Err(e) => eprint!("{}", e),
        };
    }
}
