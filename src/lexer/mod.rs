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

        if self.ch.is_alphabetic() {
            let start_pos = self.position as usize;

            while self.peek_char().is_alphabetic() {
                self.read_char();
            }

            let end_pos = self.read_position as usize;
            
            let tok_type: TokenType;
            let literal = &self.input.to_lowercase()[start_pos..end_pos];

            match literal {
                "get"    => tok_type = TokenType::Get,
                "put"    => tok_type = TokenType::Put,
                "update" => tok_type = TokenType::Update,
                "delete" => tok_type = TokenType::Delete,
                "info"   => tok_type = TokenType::Info,
                "exit"   => tok_type = TokenType::Exit,
                &_       => tok_type = TokenType::Ident
            }
            
            self.read_char();
            return token::new(String::from(literal), tok_type)
        }
        
        token::new(String::new(), TokenType::Illegal)
    }
}

// ==================== Helper Function ====================
pub fn new(input: String) -> Lexer {
    let mut lexer = Lexer{
        input,
        position: 0,
        read_position: 0,
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

    fn peek_char(&self) -> char {
        self.input.as_bytes()[self.read_position as usize] as char
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}
