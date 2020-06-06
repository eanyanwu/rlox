use std::collections::HashMap;

use crate::loxerror;
use crate::token::{Token, TokenType};

pub struct Scanner {
    chars: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    key_words: HashMap<String, TokenType>,
}

impl Scanner {
    fn is_digit(c: Option<char>) -> bool {
        c.map_or(false, |e| e.is_ascii_digit())
    }

    fn is_alphabetic(c: Option<char>) -> bool {
        c.map_or(false, |e| e.is_ascii_alphabetic() || e == '_')
    }

    fn is_alphanumeric(c: Option<char>) -> bool {
        Scanner::is_alphabetic(c) || Scanner::is_digit(c)
    }

    pub fn new(source: &str) -> Self {
        let key_words: HashMap<String, TokenType> = [
            (String::from("and"), TokenType::AND),
            (String::from("class"), TokenType::CLASS),
            (String::from("else"), TokenType::ELSE),
            (String::from("false"), TokenType::FALSE),
            (String::from("for"), TokenType::FOR),
            (String::from("fun"), TokenType::FUN),
            (String::from("if"), TokenType::IF),
            (String::from("nil"), TokenType::NIL),
            (String::from("or"), TokenType::OR),
            (String::from("print"), TokenType::PRINT),
            (String::from("return"), TokenType::RETURN),
            (String::from("super"), TokenType::SUPER),
            (String::from("this"), TokenType::THIS),
            (String::from("true"), TokenType::TRUE),
            (String::from("var"), TokenType::VAR),
            (String::from("while"), TokenType::WHILE),
        ].iter().cloned().collect();

        Self {
            chars: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            key_words: key_words,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while self.peek() != None {
            self.start = self.current;
            
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, String::from(""), self.line));

        self.tokens
    }

    fn scan_token(&mut self) {
        match self.emit() {
            // Single-character lexemes
            Some('(') => self.add_token(TokenType::LEFT_PAREN),
            Some(')') => self.add_token(TokenType::RIGHT_PAREN),
            Some('{') => self.add_token(TokenType::LEFT_BRACE),
            Some('}') => self.add_token(TokenType::RIGHT_BRACE),
            Some(',') => self.add_token(TokenType::COMMA),
            Some('.') => self.add_token(TokenType::DOT),
            Some('-') => self.add_token(TokenType::MINUS),
            Some('+') => self.add_token(TokenType::PLUS),
            Some(';') => self.add_token(TokenType::SEMICOLON),
            Some('*') => self.add_token(TokenType::STAR),

            // Lexemes that could be both single or double characters
            Some('!') => {
                let token = if self.try_match('=') { TokenType::BANG_EQUAL } else { TokenType::BANG };
                self.add_token(token);
            },
            Some('=') => {
                let token = if self.try_match('=') { TokenType::EQUAL_EQUAL } else { TokenType::EQUAL };
                self.add_token(token);
            },
            Some('<') => {
                let token = if self.try_match('=') { TokenType::LESS_EQUAL } else { TokenType::LESS };
                self.add_token(token);
            },
            Some('>') => {
                let token = if self.try_match('=') { TokenType::GREATER_EQUAL} else { TokenType::GREATER };
                self.add_token(token);
            },

            // The slash operator is a bit special because comments also begin with a slash
            Some('/') => {
                if self.try_match('/') {
                    // Ah, it's a comment line
                    // emit and discard of the next characters until we hit a new line
                    while self.peek() != Some('\n') {
                        self.emit();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }

            // Ignore non-meaningful whitespace characters
            Some(' ') | Some('\r') | Some('\t') => {},

            // When we encounter a new-line character, increment our line count
            Some('\n') => { self.line += 1; } ,

            // strings
            Some('"') => self.handle_string(),

            // numbers
            Some(n) if n.is_ascii_digit() => self.handle_number(),

            // identifiers
            Some(i) if Scanner::is_alphabetic(Some(i)) => self.handle_identifier(),

            Some(_) => loxerror::error(self.line, "Unexpected character"),

            // the method calling `scan_token` checks before hand that we are not at the end
            None => unreachable!(), 
        }
    }

    fn emit(&mut self) -> Option<char> {
        self.current += 1;
        
        self.chars.get(self.current - 1).cloned()
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.current).cloned()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.current + 1).cloned()
    }

    fn try_match(&mut self, expected: char) -> bool {
        if self.peek().map_or(false, |e| e == expected) {
            // discard the next character since we know what it is
            self.emit();

            true
        } else {
            false
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.chars[self.start..self.current].iter().collect();

        self.tokens.push(Token::new(token_type, lexeme, self.line));
    }

    fn handle_string(&mut self) {
        // Find the end of the string
        while self.peek() != Some('"') && self.peek() != None {
            // If we encounter a newline in the middle of the string, just increment the line counter
            // and keep looking for the end of the string
            if self.peek() == Some('\n') { self.line += 1; }

            self.emit();
        }

        if self.peek() == None {
            loxerror::error(self.line, "Unterminated string");
            return;
        }

        // eat up the closing quote
        self.emit();

        self.add_token(TokenType::STRING);
    }

    fn handle_number(&mut self) {
        // Read digits until you can read no more
        while Scanner::is_digit(self.peek()) { self.emit(); }

        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            self.emit();

            while Scanner::is_digit(self.peek()) { self.emit(); }
        }

        self.add_token(TokenType::NUMBER);
    }

    fn handle_identifier(&mut self) {
        while Scanner::is_alphanumeric(self.peek()) { self.emit(); }

        let lexeme = self.chars[self.start..self.current].iter().collect::<String>();

        let token_type = self.key_words.get(&lexeme).map_or(TokenType::IDENTIFIER, |&e| e);

        self.add_token(token_type);
    }
}