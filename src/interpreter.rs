use super::tokenizer::*;
use std::{io::{self, Error, ErrorKind}, thread::current, usize};

const BUFFER_SIZE: usize = 30_000;

pub struct Interpreter {
    tokens: Vec<Token>,
    index: usize,
    pointer: usize,
    buffer_vec: [u32; BUFFER_SIZE]
}

impl Interpreter {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            pointer: 0,
            buffer_vec: [0; BUFFER_SIZE]
        }
    }

    fn consume(&mut self) -> Option<&Token> {
        if self.index >= self.tokens.len() {
            None
        } else {
            let cur = &self.tokens[self.index];
            self.index += 1;
            Some(cur)
        }
    }

    pub fn interpret(&mut self) -> io::Result<()> {
        let mut current = self.consume();
        let mut last_left_brackets: Vec<usize> = vec![];
        let mut skip = false;

        while current.is_some() {
            let token_type = &current.unwrap().token_type;
            if !skip {
                match token_type {
                    TokenType::Increment => self.buffer_vec[self.pointer] += 1,
                    TokenType::Decrement => self.buffer_vec[self.pointer] -= 1,
                    TokenType::MoveLeft => {
                        if self.pointer > 0 {
                            self.pointer -= 1
                        }
                    },  
                    TokenType::MoveRight => {
                        if self.pointer < BUFFER_SIZE {
                            self.pointer += 1
                        }
                    },
                    TokenType::LeftBracket => {
                        if self.buffer_vec[self.pointer] == 0 {
                            skip = true;
                        } else {
                            last_left_brackets.push(self.index);
                        }
                    },
                    TokenType::RightBracket => {
                        let Some(index) = last_left_brackets.last() else {
                            return Err(Error::new(ErrorKind::Other, "Too many right brackets"));
                        };

                        if self.buffer_vec[self.pointer] == 0 {
                            last_left_brackets.pop();
                        } else if skip {
                            skip = false;
                        } else {
                            self.index = index.to_owned();
                        }
                    },
                    TokenType::Output => {
                        print!("{}", char::from_u32(self.buffer_vec[self.pointer]).unwrap());
                    },
                };
            } else if token_type.to_owned() == TokenType::RightBracket {
                skip = false;
            }

            current = self.consume();
        }

        Ok(())
    }
}

