use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, LineWriter, Write},
};

use crate::log::log_info;

pub fn read_previous_mtimes() -> Result<HashMap<String, u64>, std::io::Error> {
    let cwd = std::env::current_dir()?;

    log_info("Reading .static-site-rs.build");

    let mtime_cache_file = File::open(cwd.join(".static-site-rs.build"));
    let mtime_cache_file = match mtime_cache_file {
        Ok(file) => file,
        Err(_err) => return Ok(HashMap::new()),
    };

    let mtime_cache_file_reader = BufReader::new(mtime_cache_file);

    let mut hashmap: HashMap<String, u64> = HashMap::new();

    for line in mtime_cache_file_reader.lines() {
        let line = line?;
        let tokens: Vec<&str> = line.split(" ").collect();
        hashmap.insert(tokens[0].to_string(), tokens[1].parse::<u64>().unwrap());
    }

    Ok(hashmap)
}

pub fn write_mtimes(hashmap: HashMap<String, u64>) -> Result<(), std::io::Error> {
    let cwd = std::env::current_dir()?;
    let mtime_file = File::create(cwd.join(".static-site-rs.build"))?;
    let mut line_writer = LineWriter::new(mtime_file);

    for (file, mtime) in hashmap {
        line_writer.write_all(format!("{} {}\n", file, mtime).as_bytes())?;
    }

    Ok(())
}
