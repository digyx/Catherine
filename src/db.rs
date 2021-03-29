use std::collections::HashMap;

// Type alias
pub struct Database {
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
        // Placeholder
    }
    
    fn get(&self, prime_key: String, second_key: String) -> Option<SecondaryNode> {
        // Grab from the primary or secondary key
        None
    }
    
    fn update(&mut self, prime_key: String, second_key: String, value: String) {
        self.put(prime_key, second_key, value);
    }
    
    fn delete(&mut self, prime_key: String, second_key: String) {
        // Delete from secondary key
    }
}


// ==================== Utility Functions ====================
pub fn new() -> Database {
    Database{
        store: HashMap::new(),
        name: String::from("Hello world")
    }
}

fn unwrap(result: Option<&String>) -> String {
    match result {
        Some(item) => (*item).clone(),
        None => String::from("nil")
    }
}

fn exit() {
    println!("bye");
    std::process::exit(0);
}
