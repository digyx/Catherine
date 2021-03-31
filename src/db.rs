use std::collections::HashMap;

use super::lexer::token::Token;
use super::lexer::token::TokenType;

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
    pub fn put(&mut self, prime_key: Token, second_key: Token, value: Token) {
        let inner_node = SecondaryNode{
            key: second_key.to_string(),
            value: value.to_string()
        };

        match self.store.get_mut(prime_key.literal()) {
            Some(node) => {
                node.value.insert(second_key.to_string(), inner_node);
                self.response = format!("{}: {{ {}: {} }}", prime_key, second_key, value);
            },

            // Creates a new Primary Node since one doesn't exist
            None => {
                let mut inner_store: HashMap<String, SecondaryNode> = HashMap::new();
                inner_store.insert(second_key.to_string(), inner_node);

                let prime_node = PrimaryNode{
                    key: prime_key.to_string(),
                    value: inner_store,
                };

                self.store.insert(prime_key.to_string(), prime_node);
                self.response = format!("{}: {{ {}: {} }}", prime_key, second_key, value);
            }
        }
    }

    pub fn get(&mut self, prime_key: Token, second_key: Token) {
        let prime = self.store.get(prime_key.literal());

        if second_key.get_type() == TokenType::EOF {
            // TODO: Return list of key-value pairs
            self.response = String::from("Okay, this works");
        }

        match prime {
            Some(node) => {
                let res = unwrap(node.value.get(second_key.literal()));
                self.response = res;
            },
            None => self.response = String::from("null (prime)"),
        }
    }

    pub fn delete(&mut self, prime_key: Token, second_key: Token) {
        if second_key.get_type() == TokenType::EOF {
            self.store.remove(prime_key.literal());
            self.response = String::from("success");
        }

        let prime = self.store.get_mut(prime_key.literal());

        match prime {
            Some(node) => {
                let res = node.value.remove(second_key.literal());

                if res.is_none() {
                    self.response = String::from("second key does not exist");
                    return
                }

                self.response = String::from("success");
            },
            None => self.response = String::from("prime key does not exist")
        }
    }

    pub fn response(&self) -> String{
        self.response.clone()
    }

    pub fn invalid_cmd(&mut self) {
        self.response = String::from("invalid command");
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
        None => String::from("null (second)")
    }
}
