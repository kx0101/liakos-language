use std::fs;
use std::process;

use crate::environment::Environment;
use crate::evaluator::eval_statement;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;

pub fn run_file(file_path: &str) {
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not read file: {}", e);
            process::exit(1);
        }
    };
    let lexer = Lexer::new(content);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if !parser.errors.is_empty() {
        print_parser_errors(&parser.errors);
        process::exit(1);
    }
    let env = Environment::new();
    for stmt in &program.statements {
        let evaluated = eval_statement(stmt, env.clone());
        if !matches!(evaluated, Object::Null) {
            println!("{}", evaluated.inspect());
        }
    }
}

fn print_parser_errors(errors: &[String]) {
    println!("Parser errors:");
    for msg in errors {
        println!("\t{}", msg);
    }
}
