use super::args::Args;
use clap::{Arg, ArgGroup, Command};

pub fn parse_args() -> Args {
    Args::new(
        Command::new("enoc")
            .version("0.1.0")
            .author("enes")
            .about("lexer")
            .arg(Arg::new("source").short('s').long("source"))
            .arg(Arg::new("example").short('e').long("example"))
            .group(
                ArgGroup::new("input")
                    .args(["source", "example"])
                    .required(true),
            )
            .get_matches(),
    )
}
