use::std::io;
use::std::io::Write; // Used for stdout flushing

mod lexer;
mod parser;
pub mod db;

fn input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error: unable to read from stdin");
    
    buffer
}

// Start the Database!
fn main() {
    let mut db = db::new();
    
    loop {
        let raw = input();
        let lexer = lexer::new(raw);
        let mut parser = parser::new(lexer);

        db = parser.eval(db);
    }
}

fn exit() {
    println!("bye");
    std::process::exit(0);
}
