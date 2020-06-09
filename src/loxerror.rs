//! # Lox error-handling
//! 

use std::io::{stdout, Write};
use std::fmt;

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;

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


pub fn error(line: usize, message: &str) {
    report(line, "", message);
    set_error(true)
}

pub fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
    stdout().flush().unwrap();
}

pub fn get_error() -> bool {
    unsafe {
        HAD_ERROR
    }
}

pub fn get_runtime_error() -> bool {
    unsafe {
        HAD_RUNTIME_ERROR
    }
}

pub fn set_error(had_error: bool) {

    // There will be no concurrent code touching this, so it's K
    unsafe {
        HAD_ERROR = had_error;
    }
}

pub fn set_runtime_error(had_runtime_error: bool) {

    // There will be no concurrent code touching this, so it's K
    unsafe {
        HAD_RUNTIME_ERROR = had_runtime_error;
    }
}