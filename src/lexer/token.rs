type TokenType = String;

struct Token {
    Literal: String,
    Type: TokenType
}

const ILLEGAL: String = "ILLEGAL";
const EOF: String = "EOF";

const PRIME_KEY: String = "PRIME_KEY";
const SECOND_KEY: String = "SECOND_KEY";
const VAL: String = "VAL";

// Main Operators
const PUT: String = "PUT";
const GET: String = "GET";
const UPDATE: String = "UPDATE";
const DELETE: String = "DELETE";
