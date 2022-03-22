pub mod cmd;
use std::fs::create_dir_all;
use std::path::Path;
use walkdir::WalkDir;

pub fn main(args: &Vec<String>) {
    let file_paths = &args[1..];
    for p in file_paths {
        let source_path = Path::new(p);
        if source_path.is_file() {
            let mut dest_path = source_path.to_path_buf();
            dest_path.pop();

            // set filename with png extension
            dest_path.push(match Path::new(p).file_name() {
                Some(os_str) => match os_str.to_str() {
                    Some(s) => s,
                    None => {
                        cmd::failed_message(source_path);
                        continue;
                    }
                },
                None => {
                    cmd::failed_message(source_path);
                    continue;
                }
            });
            dest_path.set_extension("png");

            while dest_path.exists() {
                let file_stem = match dest_path.file_stem() {
                    Some(os_str) => match os_str.to_str() {
                        Some(s) => s,
                        None => {
                            cmd::failed_message(source_path);
                            return;
                        }
                    },
                    None => {
                        cmd::failed_message(source_path);
                        return;
                    }
                };
                let mut renamed = String::from(file_stem);
                renamed.push_str("-");
                renamed.push_str(&cmd::get_random_string());
                renamed.push_str(".png");
                dest_path.pop();
                dest_path.push(renamed);
            }

            println!("Work in Progress");
            println!("\tsrc: {}", source_path.to_str().unwrap());
            println!("\tdst: {}", &dest_path.to_str().unwrap());

            if cmd::duplicate(source_path, &dest_path) {
                println!("\x1b[32mSucceed\x1b[0m");
            } else {
                cmd::failed_message(source_path);
            }
        } else if source_path.is_dir() {
            let mut dest_root = source_path.to_path_buf();

            while {
                let mut renamed = String::from("png_");
                renamed.push_str(&cmd::get_random_string());
                renamed.push_str("");
                dest_root.pop();
                dest_root.push(renamed);
                dest_root.exists()
            } {}

            // convert immutable
            let dest_root = dest_root.clone();
            match create_dir_all(&dest_root) {
                Ok(()) => {}
                Err(_) => {
                    cmd::failed_message(source_path);
                    return;
                }
            };

            println!("{}", dest_root.to_str().unwrap());

            for entry in WalkDir::new(p) {
                let sub = match entry {
                    Ok(d) => d,
                    Err(_) => {
                        cmd::failed_message(source_path);
                        continue;
                    }
                };
                if !sub.path().is_file() {
                    continue;
                }

                let r_path = match Path::new(sub.path()).strip_prefix(source_path) {
                    Ok(p) => p,
                    Err(_) => {
                        cmd::failed_message(source_path);
                        continue;
                    }
                };

                let mut dest_path = Path::new(&dest_root).to_path_buf();
                dest_path.push(r_path);
                dest_path.set_extension("png");

                println!("Work in Progress");
                println!("\tsrc: {}", sub.path().to_str().unwrap());
                println!("\tdst: {}", &dest_path.to_str().unwrap());

                match dest_path.parent() {
                    Some(p) => match create_dir_all(p) {
                        Ok(()) => {}
                        Err(_) => {
                            return cmd::failed_message(source_path);
                        }
                    },
                    None => {
                        break;
                    }
                }
                if cmd::duplicate(sub.path(), &dest_path) {
                    println!("\x1b[32mSucceed\x1b[0m");
                } else {
                    return cmd::failed_message(source_path);
                }
            }
        } else {
            println!(
                "\x1b[31mskipped\x1b[0m: {}",
                match source_path.to_str() {
                    Some(s) => s,
                    None => "unknown file",
                }
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_a_file_test() {
        let dest_path = "./test/test1.jpg";
        let mut paths: Vec<String> = Vec::new();
        paths.push(String::from("process"));
        paths.push(String::from(dest_path));
        main(&paths);
    }

    #[test]
    fn is_a_dir_test() {
        let dest_path = "./test/TestImages";
        let mut paths: Vec<String> = Vec::new();
        paths.push(String::from("process"));
        paths.push(String::from(dest_path));
        main(&paths);
    }
}
