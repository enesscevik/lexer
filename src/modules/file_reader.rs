use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, Read};

pub fn take_sources_as_string(source_path: &String) -> io::Result<String> {
    let mut source_file = File::open(source_path)?;
    let mut source = String::new();
    source_file.read_to_string(&mut source)?;
    Ok(source)
}

pub fn take_line_from_source(source_path: &String, line_num: usize) -> io::Result<String> {
    let source_file = File::open(source_path)?;
    let reader = BufReader::new(source_file);
    for (line_number, line) in reader.lines().enumerate() {
        if line_number == line_num - 1 {
            return line;
        }
    }
    Err(Error::new(io::ErrorKind::NotFound, "Line not found"))
}
