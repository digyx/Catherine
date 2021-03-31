use std::collections::HashMap;

// Type alias
pub struct Database {
    name: String,
    store: HashMap<String, PrimaryNode>,
    response: String,
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
    pub fn put(&mut self, prime_key: String, second_key: String, value: String) {
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

    pub fn get(&self, prime_key: String, second_key: String) -> String {
        let prime = self.store.get(prime_key.as_str());

        match prime {
            Some(node) => unwrap(node.value.get(second_key.as_str())),
            None => String::from("null"),
        }
    }

    pub fn delete(&mut self, prime_key: String, second_key: String) {
        if second_key == String::new() {
            self.store.remove(prime_key.as_str());
        }

        let prime = self.store.get_mut(prime_key.as_str());
        
        match prime {
            Some(node) => {
                node.value.remove(second_key.as_str());
                self.response = format!("{} : {}", prime_key, second_key);
            },
            None => self.response = String::from("failure")
        }
    }

    pub fn set_response(&mut self, msg: String) {
        self.response = msg;
    }

    pub fn get_response(&self) -> String{
        self.response.clone()
    }

    pub fn return_info(&mut self) {
        self.response = format!(
            "Name: {}\n\
             Size: {}",
             self.name, self.store.len());
    }
}


// ==================== Utility Functions ====================
pub fn new() -> Database {
    Database{
        store: HashMap::new(),
        name: String::from("default"),
        response: String::new(),
    }
}

fn unwrap(result: Option<&SecondaryNode>) -> String {
    match result {
        Some(item) => (*item).value.clone(),
        None => String::from("nil")
    }
}
