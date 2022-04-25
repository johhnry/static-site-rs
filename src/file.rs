use std::{io::Write, path::Path};

use crate::log::{log_error, log_info, log_info_depth_file};

/// Return the file name of a Path as a String slice
pub fn get_file_name<'a, P: ?Sized + AsRef<Path>>(path: &'a P) -> &'a str {
    path.as_ref().file_name().unwrap().to_str().unwrap()
}

/// Replace include directive in html _include folder
/// Currently doesn't handle longer UTF-8 characters
fn replace_html_include(path: &Path, include_folder: &Path) -> Result<String, std::io::Error> {
    let mut file_content = std::fs::read_to_string(path).unwrap();
    let chars: Vec<char> = file_content.chars().collect();
    let mut directive_start: usize = 0;
    let mut directive_end: usize;
    let mut offset = 0;
    let mut utf8_len = 0;

    for i in 0..chars.len() {
        if chars[i] == '{' && i < chars.len() - 1 && chars[i + 1] == '%' {
            directive_start = utf8_len;
        }

        if chars[i] == '}' && i > 0 && chars[i - 1] == '%' {
            directive_end = utf8_len;
            let char_slice = &chars[(directive_start + 2)..=(directive_end - 2)];
            let directive_str = String::from_iter(char_slice);
            let directive = directive_str.trim();

            log_info(
                format!(
                    "Found include directive {} -> {}",
                    directive,
                    get_file_name(path)
                )
                .as_str(),
            );

            let include_content =
                replace_html_include(&include_folder.join(directive), include_folder);

            match include_content {
                Ok(content) => {
                    file_content.replace_range(
                        (directive_start + offset)..=(directive_end + offset),
                        &content,
                    );
                    offset += content.len() - (directive_end - directive_start + 1);
                }
                Err(err) => {
                    log_error(
                        format!(
                            "Error when reading include file {:?}",
                            include_folder.join(directive)
                        )
                        .as_str(),
                    );
                    return Err(err);
                }
            }
        }

        utf8_len += chars[i].len_utf8();
    }

    return Ok(file_content);
}

/// Recursive copy
/// Ex: cp_recursive(Path::new("foo"), Path::new("bar"))
pub fn cp_recursive(
    src: &Path,
    destination: &Path,
    force: bool,
    depth: u32,
    include_folder: &Path,
) -> Result<(), std::io::Error> {
    if Path::is_file(src) {
        // Manual handling of html files
        // TODO: should be more plug and play
        if src.extension().unwrap() == "html" {
            let mut file = std::fs::File::create(destination).unwrap();
            let file_content = replace_html_include(src, include_folder).unwrap();
            file.write(file_content.as_bytes()).unwrap();
        } else {
            std::fs::copy(src, destination)?;
        }

        return Ok(());
    } else {
        if !Path::exists(&destination) {
            std::fs::create_dir(destination).unwrap();
        }
    }

    log_info_depth_file("", depth as usize, &src);

    // Recursively call the function on child entries
    for child in src.read_dir().unwrap().filter_map(|c| c.ok()) {
        let child_pathbuf = child.path();
        let child_path = child_pathbuf.as_path();
        let destination_child_path = &destination.join(child_path.file_name().unwrap());

        log_info_depth_file("", (depth + 1) as usize, &destination_child_path);

        cp_recursive(
            child_path,
            destination_child_path,
            force,
            depth + 1,
            include_folder,
        )
        .expect("Error when recursively calling copy");
    }

    Ok(())
}
