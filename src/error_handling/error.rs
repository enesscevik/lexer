use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::result::Result as StdResult;
pub type Result<T> = StdResult<T, Error>;
use crate::modules::file_reader::take_line_from_source;

use super::error_path;

pub enum ErrorType {
    MissingArgument,
    IoError,
    LexingError,
    ParsingError,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::MissingArgument => "MissingArgument",
            Self::IoError => "IoError",
            Self::LexingError => "LexingError",
            Self::ParsingError => "ParsingError",
            //_ => "ErrorType",
        };
        write!(f, "{s}")
    }
}

pub struct Error {
    err_type: ErrorType,
    message: Option<String>,
    line: Option<usize>,
    column: Option<usize>,
    file_path: Option<String>,
    length: Option<usize>,
    //source: Option<Box<dyn StdError + Send + Sync>>,
}

impl Error {
    pub fn new(err_typ: ErrorType) -> Error {
        Error {
            err_type: err_typ,
            message: None,
            line: None,
            column: None,
            file_path: None,
            length: None,
            //source: None,
        }
    }
    pub fn with_message(mut self, msg: String) -> Self {
        self.message = Some(msg);
        self
    }

    pub fn with_line(mut self, lin: usize) -> Self {
        self.line = Some(lin);
        self
    }
    pub fn with_column(mut self, col: usize) -> Self {
        self.column = Some(col);
        self
    }
    pub fn with_file_path(mut self, path: &str) -> Self {
        self.file_path = Some(path.to_owned());
        self
    }
    pub fn with_length(mut self, len: usize) -> Self {
        self.length = Some(len);
        self
    }
    fn message(&self) -> String {
        match &self.message {
            Some(msg) => msg.clone(),
            _ => "An error occured!".to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.err_type, self.message())?;
        if let Some(line) = self.line {
            write!(f, " at line {}", line)?;
        }
        if let Some(column) = self.column {
            write!(f, ", column {}", column)?;
        }
        Ok(())
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n  Type: {}\n  Message: {}",
            self.err_type,
            self.message()
        )?;
        if let Some(path) = self.file_path.clone() {
            let this_path = error_path::path(&path);
            write!(f, "\n  file_path: '{}'", this_path)?;
        }
        if let Some(line) = self.line {
            write!(f, "\n  Line: {}", line)?;
        }
        if let Some(column) = self.column {
            write!(f, "\n  Column: {}", column)?;
        }
        if let Some(length) = self.length {
            write!(f, "\n  Length: {}", length)?;
        }
        let line = match self.line {
            Some(l) => l,
            None => 1,
        };
        let fpath = match &self.file_path {
            Some(p) => p,
            None => &String::new(),
        };
        let line_data = match take_line_from_source(fpath, line) {
            Ok(data) => data,
            Err(e) => format!("Error reading line: {}", e),
        };
        let col = match self.column {
            Some(c) => c,
            None => 0,
        };
        println!("\n {} | {}", line, line_data);
        for _ in 0..col + 4 {
            print!(" ");
        }
        let len = match self.length {
            Some(l) => l,
            None => 0,
        };
        for _ in 0..len {
            print!("^");
        }
        print!("\n");
        Ok(())
    }
}

impl StdError for Error {}

use std::io;

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::new(ErrorType::IoError).with_message(e.to_string())
    }
}
