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
        let inner_node = SecondaryNode{
            key: second_key.clone(),
            value: value.clone()
        };

        match self.store.get_mut(prime_key.as_str()) {
            Some(node) => {
                node.value.insert(second_key, inner_node);
            },

            // Creates a new Primary Node since one doesn't exist
            None => {
                let mut inner_store: HashMap<String, SecondaryNode> = HashMap::new();
                inner_store.insert(second_key, inner_node);

                let prime_node = PrimaryNode{
                    key: prime_key.clone(),
                    value: inner_store,
                };

                self.store.insert(prime_key, prime_node);
            }
        }
    }

    fn get(&self, prime_key: String, second_key: String) -> String {
        let prime = self.store.get(prime_key.as_str());

        match prime {
            Some(node) => unwrap(node.value.get(second_key.as_str())),
            None => String::from("null"),
        }
    }

    fn update(&mut self, prime_key: String, second_key: String, value: String) {
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


fn unwrap(result: Option<&SecondaryNode>) -> String {
    match result {
        Some(item) => (*item).value.clone(),
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

        let prime_key = String::from("Hello");
        let second_key = String::from("there");
        let value = String::from("world");

        match prime_operator {
            Some("put")    => db.put(prime_key, second_key, value),
            Some("get")    => println!("{}", db.get(prime_key, second_key)),
            Some("update") => db.update(prime_key, second_key, value),
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
