use std::path::Path;

use crate::file::get_file_name;

#[derive(Debug)]
enum LogLevel {
    Info,
    Error,
}

fn log(msg: &str, level: LogLevel) {
    let level_formatted = format!("{:?}", level).to_ascii_uppercase();
    println!("[{}] {}", level_formatted, msg);
}

pub fn log_info(msg: &str) {
    log(msg, LogLevel::Info);
}

pub fn log_error(msg: &str) {
    log(msg, LogLevel::Error);
}

pub fn log_info_depth_file(msg: &str, depth: usize, path: &Path) {
    log_info(
        format!(
            "{}⤷{} {}",
            "  ".repeat((depth) as usize),
            msg,
            get_file_name(path),
        )
        .as_str(),
    );
}

pub fn exit_with_message(msg: &str) {
    log_error(msg);
    std::process::exit(-1)
}
