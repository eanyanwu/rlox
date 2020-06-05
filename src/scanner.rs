use std::str;

struct Scanner<'a> {
    source: &'a str,
    chars: str::Chars<'a>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { 
            source: source,
            chars: source.chars(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1 
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while self.current <= self.source.len() {
            self.start = self.current;
            
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, String::from(""), self.line));

        self.tokens
    }

    fn scan_token(&self) {
        
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;

        self.chars.next()
    }
}

/// A minimum sequence of symbols that means something in our language
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
