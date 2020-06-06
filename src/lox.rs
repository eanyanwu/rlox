use std::fs;
use std::io::{Write, Error, stdin, stdout};
use crate::loxerror;
use crate::scanner::Scanner;

pub struct Lox {
}

impl Lox {
    pub fn new() -> Self {
        Lox {}
    }

    pub fn run_file(path: &str) -> Result<(), Error> {
        let c = fs::read_to_string(path)?;
    
        Lox::run(&c)?;

        // Exit like a good citizen
        if loxerror::get_error() { std::process::exit(65); }
    
        Ok(())
    }
    
    pub fn run_prompt() -> Result<(), Error> {
        loop {
            print!("> ");
            stdout().flush()?;
    
            let mut line = String::new();
    
            stdin().read_line(&mut line)?;
            
            Lox::run(&line)?;

            // Interactive mode shouldn't fail if the user makes a mistake
            loxerror::set_error(false);
        }
    }
    
    fn run(source: &str) -> Result<(), Error> {
        let scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens();

        for t in tokens {
            println!("{:?}", t);
            stdout().flush()?;
        }

        Ok(())
    }
}
