use std::{
    fs::read_to_string,
    env::args,
    io::{self, Error, ErrorKind}
};

mod tokenizer;
mod interpreter;

fn get_filepath() -> Option<String> {
    let args_vec: Vec<String> = args().collect();

    if args_vec.len() < 2 {
        return None;
    }

    let filepath = &args_vec[1];
    
    Some(filepath.clone())
}

fn main() -> io::Result<()> {
    let filepath = get_filepath().ok_or(Error::new(ErrorKind::Other, "Not filepath provided"))?;
    if !(filepath.ends_with(".b") || filepath.ends_with(".bf")) {
        return Err(Error::new(ErrorKind::Other, "Wrong file extention"));
    }

    let content = read_to_string(&filepath)?;
    let mut main_tokenizer = tokenizer::Tokenizer::new(content);

    let tokens = main_tokenizer.tokenizer()?;
    let mut main_interpreter = interpreter::Interpreter::new(tokens);

    main_interpreter.interpret()?;

    Ok(())
}
