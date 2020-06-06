use rlox::lox::{Lox};
use std::env::{args};
use std::process;

fn main() {
    let cmd_args = args().collect::<Vec<String>>();

    if cmd_args.len() > 2 {
        println!("Usage: rlox [Script]");
        process::exit(64);
    }
    else if cmd_args.len() == 2 {
        Lox::run_file(&cmd_args[1]).unwrap();
    }
    else {
        Lox::run_prompt().unwrap();
    }
}
