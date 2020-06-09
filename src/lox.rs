use std::fs;
use std::io::{Write, stdin, stdout};
use crate::loxerror::{set_error, get_error, LoxError};
use crate::scanner::Scanner;
use crate::parser::Parser;
use crate::interpreter::Interpreter;

pub struct Lox;

impl Lox {
    pub fn run_file(path: &str) -> Result<(), LoxError> {
        let c = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => panic!("Could not read file {}: {}", path, e) 
        };
    
        Lox::run(&c)?;

        // Exit like a good citizen
        if get_error() { std::process::exit(65); }
    
        Ok(())
    }
    
    pub fn run_prompt() -> Result<(), LoxError> {
        loop {
            print!("> ");

            stdout().flush().unwrap(); // Not sure what kind of errors can happen here.
    
            let mut line = String::new();
    
            match stdin().read_line(&mut line) {
                Ok(_) => {},
                Err(_) => { println!("Could not read input. Try again."); continue; }
            }
            
            // Until we seriously tackle error handling and synchronization,
            // discard any errors and keep looping.
            match Lox::run(&line) {
                Ok(_) => {},
                Err(e) => println!("{}", e)
            };

            // Interactive mode shouldn't fail if the user makes a mistake
            set_error(false);
        }
    }
    
    fn run(source: &str) -> Result<(), LoxError> {
        let scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);

        let expression = parser.parse()?;

        let interpreter = Interpreter::new();

        let value = interpreter.interpret(expression)?;

        println!("{}", value);

        Ok(())
    }
}