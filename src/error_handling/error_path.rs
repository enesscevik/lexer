use std::fs;
use std::path::Path;

pub fn path(path_arg: &String) -> String {
    match fs::canonicalize(Path::new(path_arg)) {
        Ok(abs_path) => String::from(abs_path.to_str().unwrap()),
        Err(_) => path_arg.clone(),
    }
}
