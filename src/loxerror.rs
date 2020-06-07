//! # Lox error-handling
//! 

use std::io::{stdout, Write};
use std::fmt;

#[derive(Debug, Clone)]
pub struct LoxError{
    message: String,
}

impl LoxError {
    pub fn new(msg: &str) -> Self {
        Self {
            message: String::from(msg),
        }
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

static mut HAD_ERROR: bool = false;

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, location, message);

    set_error(true);

    stdout().flush().unwrap();
}

pub fn get_error() -> bool {
    unsafe {
        HAD_ERROR
    }
}

pub fn set_error(had_error: bool) {

    // There will be no concurrent code touching this, so it's K
    unsafe {
        HAD_ERROR = had_error;
    }
}