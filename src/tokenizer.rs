use std::io::{self, Error, ErrorKind};

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Increment,
    Decrement,
    MoveLeft,
    MoveRight,
    LeftBracket,
    RightBracket,
    Output
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String
}

impl Token {
    fn new(type_: TokenType, value: String) -> Self {
        Self {
            token_type: type_,
            value
        }
    }
}

#[derive(Debug)]
pub struct Tokenizer {
    text: String,
    index: usize,
    tokens: Vec<Token>
}

macro_rules! add_token {
    ($tokenizer:ident, $token_type:expr, $token_value:expr) => {
        $tokenizer.tokens.push(Token::new($token_type, $token_value));
    }
}

impl Tokenizer {
    pub fn new(text: String) -> Self {
        Self {
            text,
            index: 0,
            tokens: Vec::new()
        }
    }

    fn consume(&mut self) -> Option<char> {
        let cur = self.text.chars().nth(self.index);
        self.index += 1;
        cur
    }

    pub fn tokenizer(&mut self) -> io::Result<Vec<Token>> {
        while let Some(char) = self.consume() {
            if char.is_whitespace() {
                continue;
            } else if char == '+' {
                add_token!(self, TokenType::Increment, "+".to_string());
            } else if char == '-' {
                add_token!(self, TokenType::Decrement, "-".to_string());
            } else if char == '>' {
                add_token!(self, TokenType::MoveRight, ">".to_string());
            } else if char == '<' {
                add_token!(self, TokenType::MoveLeft, "<".to_string());
            } else if char == '[' {
                add_token!(self, TokenType::LeftBracket, "[".to_string());
            } else if char == ']' {
                add_token!(self, TokenType::RightBracket, "]".to_string());
            } else if char == '.' {
                add_token!(self, TokenType::Output, ".".to_string());
            } else {
                continue;
            }
        }

        Ok(self.tokens.to_vec())
    }
}
