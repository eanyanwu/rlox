use rlox::lox::{Lox};
use std::env::{args};
use std::process;

fn main() {
    let cmd_args = args().collect::<Vec<String>>();

    let lox = Lox::new();

    if cmd_args.len() > 2 {
        println!("Usage: rlox [Script]");
        process::exit(64);
    }
    else if cmd_args.len() == 2 {
        Lox::run_file(&cmd_args[1]);
    }
    else {
        Lox::run_prompt();
    }
}
