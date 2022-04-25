use std::path::Path;

mod file;
mod log;
use file::cp_recursive;
use log::{exit_with_message, log_error, log_info};

static INCLUDE_FOLDER: &str = "_include";

fn build(force: bool) -> Result<(), std::io::Error> {
    log_info("Starting build...");
    let cwd = std::env::current_dir().unwrap();
    let src_folder = cwd.join("src");

    if !Path::exists(&src_folder) {
        exit_with_message(format!("Can't find any src folder in {:?}", cwd).as_str())
    }

    log_info(format!("Using src folder at {:?}", src_folder).as_str());
    let build_folder = cwd.join("build");
    let include_folder = src_folder.join(INCLUDE_FOLDER);

    if !Path::exists(&build_folder) {
        log_info("Build folder doesn't exist, creating it...");
        std::fs::create_dir(&build_folder).unwrap();
    } else {
        log_info("Using existing build folder");
    }

    log_info(format!("Using force mode: {:?}", force).as_str());

    // For every file and folder in src copy it to build
    for src_entry in src_folder.read_dir().unwrap() {
        match src_entry {
            Ok(entry) => {
                let entry_path = &entry.path();

                if entry.file_name() == INCLUDE_FOLDER {
                    continue;
                }

                let destination = build_folder.join(entry.file_name());
                let copy_type = if Path::is_file(entry_path) {
                    "file"
                } else {
                    "folder"
                };

                log_info(
                    format!(
                        "Copying {} {1} to build/{1}",
                        copy_type,
                        file::get_file_name(entry_path)
                    )
                    .as_str(),
                );

                cp_recursive(entry_path, &destination, force, 0, &include_folder)?;
            }
            Err(err) => exit_with_message(format!("Error when reading folder {:?}", err).as_str()),
        }
    }

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        exit_with_message("No action given, please use the following keywords: build")
    }

    if args[1] == "build" {
        let force = args.contains(&"--force".to_string());

        match build(force) {
            Ok(()) => log_info("Build successfull!"),
            Err(err) => log_error(format!("Error during build: {:?}", err).as_str()),
        }
    } else {
        exit_with_message("Incorrect arguments given")
    }

    Ok(())
}
