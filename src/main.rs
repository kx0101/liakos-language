use std::io::{self, BufReader};
use std::process;

use liakos::{repl, runner};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            println!("Welcome to the Liakos REPL!");
            let stdin = io::stdin();
            repl::start(BufReader::new(stdin.lock()), io::stdout());
        }
        2 => {
            runner::run_file(&args[1]);
        }
        _ => {
            println!("Usage: liakos <filename>");
            println!("or run without arguments to start the REPL.");
            process::exit(1);
        }
    }
}
