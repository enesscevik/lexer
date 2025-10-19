use crate::error_handling::{Error, ErrorType};
use clap::ArgMatches;

pub struct Args {
    args: ArgMatches,
}

impl Args {
    pub fn new(arg_matches: ArgMatches) -> Args {
        Args {
            args: arg_matches.to_owned(),
        }
    }

    pub fn get_arg(&self, arg_title: &str) -> Result<String, Error> {
        match self.args.get_one::<String>(arg_title) {
            Some(arg) => Ok(arg.to_string()),
            _ => Err(Error::new(ErrorType::MissingArgument)),
        }
    }

    pub fn get_source_path(&self) -> Result<String, Error> {
        if let Some(example_file) = self.args.get_one::<String>("example") {
            Ok(format!("example_syntaxes/{}", example_file))
        } else {
            self.get_arg("source")
        }
    }
}
