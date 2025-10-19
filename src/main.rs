use lexer::error_handling::Result;
use lexer::modules::{
    analyser::analyze_chars, args_handler::parse_args, file_reader::take_sources_as_string,
    tokenizer::tokenize,
};

fn main() -> Result<()> {
    let args = parse_args();

    let source_path = args.get_source_path()?;

    let chars = analyze_chars(&take_sources_as_string(&source_path)?);

    let tokens = tokenize(&chars, &source_path)?;

    tokens.iter().for_each(|token| {
        println!("{:?}", token);
    });

    Ok(())
}
