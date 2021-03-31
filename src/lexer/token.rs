#[derive(Copy, Clone)]
pub enum TokenType {
    Ident,
    Illegal,

    Put,
    Get,
    Update,
    Delete,

    Info,
    Exit,
}

pub struct Token {
    literal: String,
    tok_type: TokenType
}

pub fn new(literal: String, tok_type: TokenType) -> Token {
    Token{
        literal,
        tok_type,
    }
}

impl Token {
    pub fn get_type(&self) -> TokenType {
        self.tok_type
    }

    pub fn get_literal(&self) -> String {
        self.literal.clone()
    }
}