use super::lexer::Lexer;
use super::lexer::token;

pub struct Parser {
    cmd: token::Token,
}

pub fn new(lexer: Lexer) -> Parser {
    Parser{
        cmd: lexer.next_token(),

    }
}

impl Parser {
    pub fn eval(&self, db: super::db::Database) -> super::db::Database {
        db
    }
}
