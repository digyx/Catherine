pub enum TokenType {
    Illegal,
    EOF,

    PrimeKey,
    SecondKey,
    Value,

    Put,
    Get,
    Update,
    Delete,
}

pub struct Token {
    Literal: String,
    Type: TokenType
}

pub fn new(literal: String, tok_type: TokenType) -> Token {
    Token{
        Literal: literal,
        Type: tok_type,
    }
}