#[derive(Copy, Clone, PartialEq)]
pub enum TokenType {
    Ident,
    Illegal,
    EOF,

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

    pub fn literal(&self) -> &str {
        self.literal.as_str()
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal)
    }
}