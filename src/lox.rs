use std::fs;
use std::io::{Error, stdin};

static mut HAD_ERROR: bool = false;

pub struct Lox {
}

impl Lox {
    pub fn new() -> Self {
        Lox {}
    }

    fn set_error(had_error: bool) {

        // There will be no concurrent code touching this, so it's K
        unsafe {
            HAD_ERROR = had_error;
        }
    }

    fn get_error() -> bool {
        let mut he = false;

        unsafe {
            he = HAD_ERROR;
        }

        he
    }


    pub fn run_file(path: &str) -> Result<(), Error> {
        let c = fs::read_to_string(path)?;
    
        Lox::run(&c);

        // Exit like a good citizen
        if Lox::get_error() { std::process::exit(65); }
    
        Ok(())
    }
    
    pub fn run_prompt() -> Result<(), Error> {
        let input = stdin();
    
        loop {
            println!("> ");
    
            let mut line = String::new();
    
            input.read_line(&mut line)?;
    
            Lox::run(&line);

            // Interactive mode shouldn't fail if the user makes a mistake
            Lox::set_error(false);
        }
    }
    
    fn run(source: &str) {
        // TODO:
    }
    
    fn error(line: usize, message: &str) {
        Lox::report(line, "", message);
    }
    
    fn report(line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, location, message);

        Lox::set_error(true);
    }
}
