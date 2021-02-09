mod token;

struct Lexer {
    input: String,
    position: i32,
    readPosition: i32,
    ch: char,
}

impl Lexer {
    pub fn next_token() ->  token::Token {
        
    }
}

pub fn new(input: String) -> Lexer {
    let mut lexer = Lexer{
        input,
        position: 0,
        readPosition: 1,
    };

    lexer.read_char()
}

impl Lexer {
    fn read_char(&mut self) {
        if (self.position >= self.input.len) {
            self.ch = '0';
        } else {

        }
    }
}
