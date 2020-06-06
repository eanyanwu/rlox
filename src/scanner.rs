use std::str;
use std::collections::HashMap;

use crate::loxerror;

pub struct Scanner<'a> {
    source: &'a str,
    chars: str::Chars<'a>,
    buf: Vec<Option<char>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    key_words: HashMap<String, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
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
            source,
            chars: source.chars(),
            buf: Vec::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            key_words: key_words,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, String::from(""), self.line));

        self.tokens
    }

    fn scan_token(&mut self) {
        let next = self.emit();

        match next {
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
                let token = if self.conditional_emit('=') { TokenType::BANG_EQUAL } else { TokenType::BANG };
                self.add_token(token);
            },
            Some('=') => {
                let token = if self.conditional_emit('=') { TokenType::EQUAL_EQUAL } else { TokenType::EQUAL };
                self.add_token(token);
            },
            Some('<') => {
                let token = if self.conditional_emit('=') { TokenType::LESS_EQUAL } else { TokenType::LESS };
                self.add_token(token);
            },
            Some('>') => {
                let token = if self.conditional_emit('=') { TokenType::GREATER_EQUAL} else { TokenType::GREATER };
                self.add_token(token);
            },

            // The slash operator is a bit special because comments also begin with a slash
            Some('/') => {
                if self.conditional_emit('/') {
                    // Ah, it's a comment line
                    // emit and discard of the next characters until we hit a new line
                    while self.peek() != Some('\n') && !self.is_at_end() {
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

    // Emits & consumes the next character
    fn emit(&mut self) -> Option<char> {
        self.current += 1;

        if self.buf.len() > 0 {
            self.buf.remove(0)
        } else {
            self.chars.next()
        }
    }

    // Emits the next character without consuming it
    fn peek(&mut self) -> Option<char> {
        if self.buf.len() > 0 {
            self.buf[self.buf.len() - 1]
        } else {
            let peeked = self.chars.next();
            self.buf.push(peeked);
            peeked
        }
    }

    fn peek_next(&mut self) -> Option<char> {
        if self.buf.len() > 0 {
            if self.buf.len() == 1 {
                let next = self.chars.next();
                self.buf.push(next);
                next
            } else if self.buf.len() == 2 {
                self.buf[self.buf.len() - 1]
            } else {
                unreachable!()
            }
        } else {
            let first = self.chars.next();
            let second = self.chars.next();

            self.buf.push(first);
            self.buf.push(second);

            second
        }
    }

    // Conditionally consumes the next character and increments the counter.
    fn conditional_emit(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else {
            if self.peek().map_or(false, |e| e == expected) {
                // discard the next character since we know what it is
                self.emit();

                true
            } else {
                false
            }
        }
    }

    // Determines if we are at the end of the iterator by peeking at the next element.
    fn is_at_end(&mut self) -> bool {
        self.peek() == None
    }

    fn is_digit(c: Option<char>) -> bool {
        c.map_or(false, |e| e.is_ascii_digit())
    }

    fn is_alphabetic(c: Option<char>) -> bool {
        c.map_or(false, |e| e.is_ascii_alphabetic() || e == '_')
    }

    fn is_alphanumeric(c: Option<char>) -> bool {
        Scanner::is_alphabetic(c) || Scanner::is_digit(c)
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.start..self.current].to_string();

        self.tokens.push(Token::new(token_type, lexeme, self.line));
    }

    fn handle_string(&mut self) {
        // Find the end of the string
        while self.peek() != Some('"') && !self.is_at_end() {
            // If we encounter a newline in the middle of the string, just increment the line counter
            // and keep looking for the end of the string
            if self.peek() == Some('\n') { self.line += 1; }

            self.emit();
        }

        if self.is_at_end() {
            loxerror::error(self.line, "Unterminated string");
            return;
        }

        // eat up the closing quote
        self.emit();

        self.add_token(TokenType::STRING);
    }

    fn handle_number(&mut self) {
        // Read digits until you canr ead no more
        while Scanner::is_digit(self.peek()) {
            self.emit();
        }

        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            self.emit();

            while Scanner::is_digit(self.peek()) {
                self.emit();
            }
        }

        self.add_token(TokenType::NUMBER);
    }

    fn handle_identifier(&mut self) {
        while Scanner::is_alphanumeric(self.peek()) {
            self.emit();
        }

        let lexeme = self.source[self.start..self.current].to_string();

        let token_type = self.key_words.get(&lexeme).map_or(TokenType::IDENTIFIER, |&e| e);

        self.add_token(token_type);
    }
}

/// A minimum sequence of symbols that means something in our language
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {}", self.token_type, self.lexeme)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR, 

    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}