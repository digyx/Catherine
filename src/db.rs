use::std::io;
use::std::io::Write; // Used for stdout flushing
use std::collections::HashMap;

// Type alias
struct Database {
    store: HashMap<String, PrimaryNode>,
    name: String,
}

struct PrimaryNode {
    key: String,
    value: HashMap<String, SecondaryNode>
}

struct SecondaryNode {
    key: String,
    value: String
}

impl Database {
    fn put(&mut self, prime_key: String, second_key: String, value: String) {
        // 
    }

    fn get(&self, prime_key: String, second_key: String) -> Option<SecondaryNode> {
        // Grab from the primary or secondary key
        None
    }

    fn update(&self, prime_key: String, second_key: String, value: String) {
        self.put(prime_key, second_key, value);
    }

    fn delete(&mut self, prime_key: String, second_key: String) {
        // Delete from secondary key
    }
}


// ==================== Utility Functions ====================
fn input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error: unable to read from stdin");

    buffer
}


fn unwrap(result: Option<&String>) -> String {
    match result {
        Some(item) => (*item).clone(),
        None => String::from("nil")
    }
}

// DB Management functions
pub fn start() {
    let mut db: Database = Database{
        store: HashMap::new(),
        name: String::from("Hello world")
    };

    loop {
        // let cmd = input();
        let prime_operator: Option<&str> = Some("Hello");

        match prime_operator {
            Some("put")    => print!("Put"),
            Some("get")    => print!("Get"),
            Some("update") => print!("Update"),
            Some("delete") => print!("Delete"),
            Some("exit")   => exit(),
            Some(_)        => println!("error: could not read command"),
            None           => println!("\n")
        }
    }
}


fn exit() {
    println!("bye");
    std::process::exit(0);
}
