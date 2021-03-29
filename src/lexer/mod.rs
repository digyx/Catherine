pub mod token;

use token::TokenType;

pub struct Lexer {
    input: String,
    position: u32,
    read_position: u32,
    ch: char,
}

impl Lexer {
    pub fn next_token(&self) ->  token::Token {
        // PLACEHOLDER
        token::new(String::from(""), TokenType::EOF)
    }
}

pub fn new(input: String) -> Lexer {
    let mut lexer = Lexer{
        input,
        position: 0,
        read_position: 1,
        ch: 'a',
    };

    lexer.read_char();

    lexer
}

impl Lexer {
    fn read_char(&mut self) {
        if self.position >= self.input.len() as u32 {
            self.ch = '0';
        } else {

        }
    }
}
