use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

// literals
pub const IDENTIFIER: u8 = 37;
pub const INT: u8 = 38;
pub const FLOAT: u8 = 39;
pub const STRING: u8 = 40;


pub struct Token {
    pub token_string: String,
    pub token_num: u8,
    pub line_num: u64,
}

impl Token {
    pub fn new(token_string: &String, line_num: u64) -> Token {
        let token = Token {
            token_string: token_string.to_string(),
            token_num: 0,
            line_num: line_num,
        };
        token
    }

    pub fn find_token_num(&mut self, lex_language: &HashMap<String, u8>, string: &regex::Regex, integer: &regex::Regex,
        float: &regex::Regex, identifier: &regex::Regex) -> bool {
        match lex_language.get(&self.token_string) {
            Some(num) => self.token_num = *num,
            None => {
                if string.is_match(self.token_string.as_str()) {
                    self.token_num = STRING;
                } else if integer.is_match(self.token_string.as_str()) {
                    self.token_num = INT;
                } else if float.is_match(self.token_string.as_str()) {
                    self.token_num = FLOAT;
                } else if identifier.is_match(self.token_string.as_str()) {
                    self.token_num = IDENTIFIER;
                } else {
                    println!("Unkown token: {} at line {}.", self.token_string, self.line_num);
                    return true;
                }
            },
        }
        false
    }
}

pub fn lexer(source: &String) -> (Vec<Token>, bool) {
    let mut lex_language: HashMap<String, u8> = HashMap::new();

    // single char tokens
    lex_language.insert(";".to_string(), 0);
    lex_language.insert("(".to_string(), 1);
    lex_language.insert(")".to_string(), 2);
    lex_language.insert("{".to_string(), 3);
    lex_language.insert("}".to_string(), 4);
    lex_language.insert("=".to_string(), 5);
    lex_language.insert(":".to_string(), 6);
    lex_language.insert(",".to_string(), 7);
    lex_language.insert("[".to_string(), 8);
    lex_language.insert("]".to_string(), 9);
    lex_language.insert("+".to_string(), 10);
    lex_language.insert("*".to_string(), 11);
    lex_language.insert("/".to_string(), 12);
    lex_language.insert("-".to_string(), 13);
    lex_language.insert("%".to_string(), 14);
    lex_language.insert("<".to_string(), 15);
    lex_language.insert(">".to_string(), 16);

    // two char tokens
    lex_language.insert("==".to_string(), 17);
    lex_language.insert("!=".to_string(), 18);
    lex_language.insert("<=".to_string(), 19);
    lex_language.insert(">=".to_string(), 20);

    // keywords
    lex_language.insert("int".to_string(), 21);
    lex_language.insert("float".to_string(), 22);
    lex_language.insert("string".to_string(), 23);
    lex_language.insert("fn".to_string(), 24);
    lex_language.insert("let".to_string(), 25);
    lex_language.insert("and".to_string(), 26);
    lex_language.insert("or".to_string(), 27);
    lex_language.insert("while".to_string(), 28);
    lex_language.insert("return".to_string(), 29);
    lex_language.insert("if".to_string(), 30);
    lex_language.insert("else".to_string(), 31);

    // other
    lex_language.insert("void".to_string(), 32);
    lex_language.insert("break".to_string(), 33);
    lex_language.insert("vec_int".to_string(), 34);
    lex_language.insert("vec_float".to_string(), 35);
    lex_language.insert("vec_string".to_string(), 36);

    let mut single_char_tokens: HashSet<char> = HashSet::new();
    single_char_tokens.insert(';');
    single_char_tokens.insert('(');
    single_char_tokens.insert(')');
    single_char_tokens.insert('{');
    single_char_tokens.insert('}');
    single_char_tokens.insert(':');
    single_char_tokens.insert(',');
    single_char_tokens.insert('[');
    single_char_tokens.insert(']');
    single_char_tokens.insert('+');
    single_char_tokens.insert('*');
    single_char_tokens.insert('/');
    single_char_tokens.insert('-');
    single_char_tokens.insert('%');

    let string = Regex::new(r"^\x22[^\x22]*\x22$").unwrap();
    let integer = Regex::new(r"^-?[0-9]+$").unwrap();
    let float = Regex::new(r"^-?[0-9]+\.[0-9]+$").unwrap();
    let identifier = Regex::new(r"^[_a-zA-Z][_a-zA-z0-9]*$").unwrap();

    let mut tokens: Vec<Token> = Vec::new();
    let mut error: bool = false;
    let mut current_token_string: String = String::new();
    let mut is_current_char_a_string: bool = false;
    let mut current_line_number: u64 = 1;

    let mut i: u64 = 0;
    for c in source.chars() {
        if c as u8 > 127 {
            println!("Non-ascii character: {} at line {}.", c, current_line_number);
        }
        if !is_current_char_a_string {
            if c == '\n' {
                current_line_number += 1;
            }
            if single_char_tokens.contains(&c) {
                if c != '-' || (c == '-' && source.chars().nth(i as usize + 1) == Some(' ')) {
                    if !current_token_string.is_empty() {
                        let mut token = Token::new(&current_token_string, current_line_number);
                        if token.find_token_num(&lex_language, &string, &integer, &float, &identifier) {
                            error = true;
                        }
                        tokens.push(token);
                        current_token_string = String::from("");
                    }
                    let mut token = Token::new(&String::from(c), current_line_number);
                    if token.find_token_num(&lex_language, &string, &integer, &float, &identifier) {
                        error = true;
                    }
                    tokens.push(token);
                } else {
                    current_token_string.push(c);
                }
            } else if c == '"' {
                is_current_char_a_string = true;
                current_token_string.push(c);
            } else if c != ' ' && c != '\n' && c != '\t' && c != '\r' {
                current_token_string.push(c);
            } else if !current_token_string.is_empty() {
                let mut token = Token::new(&current_token_string, current_line_number);
                if token.find_token_num(&lex_language, &string, &integer, &float, &identifier) {
                    error = true;
                }
                tokens.push(token);
                current_token_string = String::from("");
            }
        } else {
            if c == '"' {
                is_current_char_a_string = false;
            }
            current_token_string.push(c);
        }
        i += 1;
    }
    (tokens, error)
}