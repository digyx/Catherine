pub mod token;

use token::TokenType;

pub struct Lexer {
    input: String,
    position: u32,
    read_position: u32,
    ch: char,
}

impl Lexer {
    pub fn next_token(&mut self) ->  token::Token {
        self.skip_whitespace();

        token::new(String::new(), TokenType::EOF)
    }
}

// ==================== Helper Function ====================
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
            self.ch = self.input.as_bytes()[self.read_position as usize] as char;
        }

        self.position = self.read_position;
        self.read_position  += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}
